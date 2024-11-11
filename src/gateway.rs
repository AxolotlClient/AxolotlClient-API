use crate::{errors::ApiError, extractors::Authentication, ApiState};
use axum::extract::{ws::close_code, ws::CloseFrame, ws::Message, ws::WebSocket, State, WebSocketUpgrade};
use axum::{body::Body, http::StatusCode, response::Response};
use log::warn;
use sqlx::query;
use std::{convert::Infallible, fmt::Display, fmt::Formatter, time::Duration};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver};
use tokio::{pin, select, time::sleep, time::Instant};
use uuid::Uuid;
use DisconnectReason::*;

pub async fn gateway(
	state: State<ApiState>,
	Authentication(uuid): Authentication,
	socket: WebSocketUpgrade,
) -> Result<Response<Body>, ApiError> {
	if state.online_users.contains_key(&uuid) {
		Err(StatusCode::CONFLICT)?;
	}

	Ok(socket.on_upgrade(move |socket| gateway_accept_handler(state, uuid, socket)))
}

async fn gateway_accept_handler(
	State(ApiState {
		database,
		online_users,
		socket_sender,
		..
	}): State<ApiState>,
	uuid: Uuid,
	mut socket: WebSocket,
) {
	online_users.insert(uuid, None);
	let (sender, mut receiver) = unbounded_channel();
	socket_sender.insert(uuid, sender);

	let disconnect_reason = gateway_accept(&mut socket, &mut receiver).await.unwrap_err();
	let _ = socket
		.send(Message::Close(Some(CloseFrame {
			code: disconnect_reason as u16,
			reason: format!("{disconnect_reason}").into(),
		})))
		.await;

	if let Some((_, s)) = socket_sender.remove(&uuid) {
		drop(s);
	}
	receiver.close();
	online_users.remove(&uuid);

	let _ = query!("UPDATE players SET last_online = 'now' WHERE uuid = $1 AND show_last_online = true", uuid)
		.execute(&database)
		.await;
}

async fn gateway_accept(
	socket: &mut WebSocket,
	receiver: &mut UnboundedReceiver<String>,
) -> Result<Infallible, DisconnectReason> {
	let mut pending_pong: Option<[u8; 32]> = None;
	let keep_alive = sleep(Duration::from_secs(10));
	pin!(keep_alive);

	loop {
		select! {
			biased;
			message = socket.recv() => {
				match message.ok_or(Closed)?? {
					Message::Text(data) => {
						// When we actually use the WebSocket for something other than knowing if the player is online
						// then we will probably actually have something here, but for now we error if any messages are
						// sent

						warn!("received {data} through websocket even though there should be no messages this way!");

						return Err(InvalidData);
					}
					Message::Binary(_) => return Err(InvalidData),
					Message::Ping(_) => {} // This should be handled for us
					Message::Pong(pong) => {
						match pending_pong {
							None => return Err(InvalidData),
							Some(inner_pending_pong) => {
								if pong != inner_pending_pong {
									return Err(InvalidData);
								}
							}
						}
					}
					Message::Close(_) => return Err(Closed),
				}

				keep_alive.as_mut().reset(Instant::now() + Duration::from_secs(10));
				pending_pong = None;
			}
			socket_message = receiver.recv() => {
				if let Some(socket_message) = socket_message {
					socket.send(Message::Text(socket_message)).await?;

				}
			}
			_ = &mut keep_alive => {
				match pending_pong {
					None => {
						let ping = rand::random();
						socket.send(Message::Ping(Vec::from(&ping))).await?;
						pending_pong = Some(ping);
						keep_alive.as_mut().reset(Instant::now() + Duration::from_secs(10));
					}
					Some(_) => return Err(TimedOut),
				}
			}
		}
	}
}

#[repr(u16)]
#[derive(Copy, Clone)]
enum DisconnectReason {
	Closed = close_code::NORMAL,
	Error = close_code::ERROR,
	InvalidData = close_code::INVALID,
	TimedOut = 1014, // There is no pre-defined code for timeouts
}

impl Display for DisconnectReason {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Closed => write!(f, "Closed"),
			Error => write!(f, "Error"),
			InvalidData => write!(f, "Invalid Data"),
			TimedOut => write!(f, "Timed Out"),
		}
	}
}

impl From<axum::Error> for DisconnectReason {
	fn from(_: axum::Error) -> DisconnectReason {
		Error
	}
}
