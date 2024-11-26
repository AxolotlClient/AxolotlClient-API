use std::collections::HashMap;

use crate::{errors::ApiError, extractors::Authentication, id::Id, ApiState};
use axum::{extract::Path, extract::Query, extract::State, Json};
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{query, query_as, PgPool};
use uuid::Uuid;

use super::user::Activity;

#[derive(Serialize)]
pub struct User {
	uuid: Uuid,
	username: String,
	registered: DateTime<Utc>,
	last_online: Option<DateTime<Utc>>,
	previous_usernames: Vec<OldUsername>,
}

#[derive(Serialize)]
pub struct OldUsername {
	username: String,
	public: bool,
}

impl User {
	pub async fn get(database: &PgPool, uuid: &Uuid) -> Result<User, ApiError> {
		let user = query!("SELECT uuid, username, registered, last_online FROM players WHERE uuid = $1", uuid)
			.fetch_one(database)
			.await?;
		let previous_usernames =
			query_as!(OldUsername, "SELECT username, public FROM previous_usernames WHERE player = $1", uuid)
				.fetch_all(database)
				.await?;

		Ok(User {
			uuid: user.uuid,
			username: user.username,
			registered: user.registered.and_utc(),
			last_online: user.last_online.map(|dt| dt.and_utc()),
			previous_usernames,
		})
	}
}

pub async fn get(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
) -> Result<Json<User>, ApiError> {
	Ok(Json(User::get(&database, &uuid).await?))
}

pub async fn delete(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
) -> Result<StatusCode, ApiError> {
	query!("DELETE FROM players WHERE uuid = $1", uuid)
		.execute(&database)
		.await?;

	Ok(StatusCode::NO_CONTENT)
}

pub async fn get_channels(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
) -> Result<Json<Vec<u64>>, ApiError> {
	let mut response = Vec::new();

	let owned = query!("SELECT id FROM channels WHERE owner = $1", uuid)
		.fetch_all(&database)
		.await?;
	let participating = query!("SELECT channels FROM channel_memberships WHERE player = $1", uuid)
		.fetch_optional(&database)
		.await?;

	for id in owned {
		response.push(id.id as u64);
	}
	if let Some(ids) = participating {
		for id in ids.channels {
			response.push(id as u64);
		}
	}

	Ok(Json(response))
}

pub async fn get_friends(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
) -> Result<Json<Vec<Uuid>>, ApiError> {
	Ok(Json(
		query!("SELECT player_b FROM relations WHERE player_a = $1 AND relation = 'friend'", uuid)
			.fetch_all(&database)
			.await?
			.iter()
			.map(|r| r.player_b)
			.collect(),
	))
}

pub async fn get_blocked(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
) -> Result<Json<Vec<Uuid>>, ApiError> {
	Ok(Json(
		query!("SELECT player_b FROM relations WHERE player_a = $1 AND relation = 'blocked'", uuid)
			.fetch_all(&database)
			.await?
			.iter()
			.map(|r| r.player_b)
			.collect(),
	))
}

pub async fn get_requests(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
) -> Result<Json<HashMap<String, Vec<Uuid>>>, ApiError> {
	let mut map: HashMap<String, Vec<Uuid>> = HashMap::new();

	map.insert(
		"out".to_string(),
		query!("SELECT player_b FROM relations WHERE player_a = $1 AND relation = 'request'", uuid)
			.fetch_all(&database)
			.await?
			.iter()
			.map(|r| r.player_b)
			.collect(),
	);

	map.insert(
		"in".to_string(),
		query!("SELECT player_a FROM relations WHERE player_b = $1 AND relation = 'request'", uuid)
			.fetch_all(&database)
			.await?
			.iter()
			.map(|r| r.player_a)
			.collect(),
	);
	Ok(Json(map))
}

#[derive(Serialize)]
pub struct UserData {
	user: User,
	settings: Settings,
}

pub async fn get_data(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
) -> Result<Json<UserData>, ApiError> {
	Ok(Json(UserData {
		user: User::get(&database, &uuid).await?,
		settings: Settings::get(&database, &uuid).await?,
	}))
}

#[derive(Serialize)]
pub struct Settings {
	show_registered: bool,
	retain_usernames: bool,
	show_last_online: bool,
	show_activity: bool,
}

impl Settings {
	pub async fn get(database: &PgPool, uuid: &Uuid) -> Result<Settings, ApiError> {
		Ok(query_as!(
			Settings,
			"SELECT show_registered, retain_usernames, show_last_online, show_activity FROM players WHERE uuid = $1",
			uuid
		)
		.fetch_one(database)
		.await?)
	}
}

pub async fn get_settings(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
) -> Result<Json<Settings>, ApiError> {
	Ok(Json(Settings::get(&database, &uuid).await?))
}

#[derive(Deserialize)]
pub struct SettingsPatch {
	show_registered: Option<bool>,
	retain_usernames: Option<bool>,
	show_last_online: Option<bool>,
	show_activity: Option<bool>,
}

pub async fn patch_settings(
	State(ApiState {
		database, online_users, ..
	}): State<ApiState>,
	Authentication(uuid): Authentication,
	Json(user_settings_patch): Json<SettingsPatch>,
) -> Result<StatusCode, ApiError> {
	query!(
		r#"
			UPDATE players SET
				show_registered = coalesce($1, show_registered),
				retain_usernames = coalesce($2, retain_usernames),
				show_last_online = coalesce($3, show_last_online),
				show_activity = coalesce($4, show_activity)
			WHERE uuid = $5
		"#,
		user_settings_patch.show_registered,
		user_settings_patch.retain_usernames,
		user_settings_patch.show_last_online,
		user_settings_patch.show_activity,
		uuid
	)
	.execute(&database)
	.await?;

	query!("UPDATE players SET last_online = null WHERE uuid = $1 AND show_last_online = true", uuid)
		.execute(&database)
		.await?;

	if user_settings_patch.show_activity.is_some_and(|value| !value) {
		if let Some(mut activity) = online_users.get_mut(&uuid) {
			*activity = None;
		}
	}

	Ok(StatusCode::NO_CONTENT)
}

pub async fn post_username(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
	Path(username): Path<String>,
	Query(public): Query<bool>,
) -> Result<StatusCode, ApiError> {
	let rows_affected =
		query!("UPDATE previous_usernames SET public = $1 WHERE player = $2 AND username = $3", public, uuid, username)
			.execute(&database)
			.await?
			.rows_affected();
	match rows_affected {
		0 => Err(StatusCode::NOT_FOUND)?,
		_ => Ok(StatusCode::NO_CONTENT),
	}
}

pub async fn delete_username(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
	Path(username): Path<String>,
) -> Result<StatusCode, ApiError> {
	query!("DELETE FROM previous_usernames WHERE player = $1 AND username = $2", uuid, username)
		.execute(&database)
		.await?;
	Ok(StatusCode::NOT_FOUND)
}

pub async fn post_activity(
	State(ApiState {
		online_users,
		database,
		socket_sender,
		..
	}): State<ApiState>,
	Authentication(uuid): Authentication,
	Json(activity): Json<Activity>,
) -> Result<StatusCode, ApiError> {
	let friends = query!("SELECT player_b FROM relations WHERE relation = 'friend' AND player_a = $1", &uuid)
		.fetch_all(&database)
		.await?;

	for ele in friends {
		if socket_sender.contains_key(&ele.player_b) {
			let socket = socket_sender
				.get(&ele.player_b)
				.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
			socket
				.send(
					serde_json::to_string(&json!({
						"target": "activity_update",
						"user": &uuid,
						"activity": activity
					}))
					.unwrap(),
				)
				.unwrap();
		}
	}

	online_users.insert(uuid, Some(activity));

	Ok(StatusCode::OK)
}

#[derive(Serialize)]
pub struct ChannelInvite {
	id: u64,
	channel_name: String,
	from: Uuid,
}

pub async fn get_channel_invites(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
) -> Result<Json<Vec<ChannelInvite>>, ApiError> {
	let mut transaction = database.begin().await?;
	let channels = query!("SELECT channel, sender FROM channel_invites WHERE player = $1", uuid)
		.fetch_all(&mut *transaction)
		.await?;

	let mut invites = Vec::new();
	for r in channels {
		let c_name = query!("SELECT name FROM channels WHERE id = $1", r.channel as _)
			.fetch_one(&mut *transaction)
			.await?;
		invites.push(ChannelInvite {
			id: r.channel as u64,
			channel_name: c_name.name,
			from: r.sender,
		})
	}

	transaction.commit().await?;

	Ok(Json(invites))
}

#[derive(Deserialize)]
pub struct QueryChannelInvite {
	id: Id,
	accept: bool,
}

pub async fn post_channel_invite(
	State(ApiState {
		database,
		socket_sender,
		..
	}): State<ApiState>,
	Authentication(uuid): Authentication,
	Query(QueryChannelInvite { id, accept }): Query<QueryChannelInvite>,
) -> Result<StatusCode, ApiError> {
	let mut transaction = database.begin().await?;

	let sender = query!("SELECT sender FROM channel_invites WHERE player = $1 AND channel = $2", &uuid, &id as _)
		.fetch_one(&mut *transaction)
		.await?;
	let name = query!("SELECT name FROM channels WHERE id = $1", &id as _)
		.fetch_one(&mut *transaction)
		.await?;
	query!("DELETE FROM channel_invites WHERE player = $1 AND channel = $2", &uuid, &id as _)
		.execute(&mut *transaction)
		.await?;

	if accept {
		query!(
			r#"INSERT INTO channel_memberships(player, channels)
			 VALUES ($1, ARRAY [$2::bigint]) 
			 ON CONFLICT (player) DO UPDATE 
			 SET channels = ARRAY_APPEND(channel_memberships.channels, $2) 
			 WHERE channel_memberships.player = $1"#,
			uuid,
			&id as _
		)
		.execute(&mut *transaction)
		.await?;
	}

	transaction.commit().await?;

	if let Some(socket) = socket_sender.get(&sender.sender) {
		let _ = socket.send(
			serde_json::to_string(&json!({
				"target": "channel_invite_reaction",
				"channel": &id,
				"channel_name": name.name,
				"player": &uuid,
				"accepted": &accept
			}))
			.unwrap(),
		);
	}

	Ok(StatusCode::OK)
}
