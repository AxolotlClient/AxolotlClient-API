use crate::{errors::ApiError, ApiState};
use axum::{extract::Query, extract::State, http::StatusCode, response::IntoResponse, Json};
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};
use blake2::{Blake2b512, Digest};
use garde::Validate;
use serde::{Deserialize, Serialize};
use sqlx::{error::ErrorKind::UniqueViolation, query, query_as, query_scalar, Type};
use std::ops::Deref;
use uuid::Uuid;

pub mod account;
pub mod channel;
pub mod global_data;
pub mod hypixel;
pub mod image;
pub mod user;

#[derive(Clone, Deserialize, Serialize, Validate, Type)]
#[repr(transparent)]
#[serde(transparent)]
#[sqlx(transparent)]
pub struct Username(#[garde(pattern(r"[a-zA-Z0-9_]{3,16}"))] String);

impl From<Username> for String {
	fn from(value: Username) -> Self {
		value.0
	}
}

impl From<String> for Username {
	fn from(value: String) -> Self {
		Username(value)
	}
}

impl Deref for Username {
	type Target = String;

	fn deref(&self) -> &Self::Target {
		&self.0
	}
}

#[derive(Deserialize, Validate)]
pub struct Authenticate {
	#[garde(dive)]
	username: Username,
	#[garde(skip)] // No documented limits ¯\_(ツ)_/¯
	server_id: String,
}

#[derive(Serialize)]
pub struct AuthenticateResponse {
	uuid: Uuid,
	username: Username,
	access_token: String,
}

pub async fn get_authenticate(
	State(ApiState { database, client, .. }): State<ApiState>,
	Query(authenticate): Query<Authenticate>,
) -> Result<Json<AuthenticateResponse>, ApiError> {
	authenticate.validate(&())?;
	let Authenticate { username, server_id } = authenticate;

	#[derive(Clone, Deserialize)]
	struct BasicUserInfo {
		#[serde(rename = "id")]
		uuid: Uuid,
		#[serde(rename = "name")]
		username: Username,
	}

	let response = client
		.get("https://sessionserver.mojang.com/session/minecraft/hasJoined")
		.query(&[("username", &*username), ("serverId", &server_id)])
		.send()
		.await?;

	let user: BasicUserInfo = match response.status() {
		reqwest::StatusCode::OK => Ok(response.json().await?),
		reqwest::StatusCode::NO_CONTENT => Err(StatusCode::UNAUTHORIZED),
		_ => Err(StatusCode::INTERNAL_SERVER_ERROR),
	}?;

	let mut transaction = database.begin().await?;

	let banned = query_scalar!("SELECT banned FROM players WHERE uuid = $1", &user.uuid)
		.fetch_optional(&mut *transaction)
		.await?;
	if let Some(val) = banned {
		if val {
			// User is banned, revoke all tokens
			query!("UPDATE tokens SET revoked = true WHERE player = $1", user.uuid)
				.execute(&mut *transaction)
				.await?;

			transaction.commit().await?;
			return Err(StatusCode::FORBIDDEN)?;
		}
	}

	let mut usernames_to_update = vec![user.clone()];

	while let Some(player_to_update) = usernames_to_update.pop() {
		let existing_player_with_name = query_as!(
			BasicUserInfo,
			"SELECT username, uuid FROM players WHERE uuid != $1 AND username = $2",
			player_to_update.uuid,
			&player_to_update.username
		)
		.fetch_optional(&mut *transaction)
		.await?;

		match existing_player_with_name {
			Some(existing_player_with_name) => {
				let existing_player_with_name = client
					.get(format!(
						"https://sessionserver.mojang.com/session/minecraft/profile/{}",
						existing_player_with_name.uuid
					))
					.send()
					.await?
					.json()
					.await?;

				usernames_to_update.push(player_to_update);
				usernames_to_update.push(existing_player_with_name);
			}
			None => {
				let previous_username = query_scalar!(
					"SELECT username FROM players WHERE uuid = $1 AND retain_usernames",
					player_to_update.uuid
				)
				.fetch_optional(&mut *transaction)
				.await?;

				if let Some(previous_username) = previous_username {
					if previous_username != String::from(player_to_update.username.clone()) {
						query!(
							"INSERT INTO previous_usernames(player, username) VALUES ($1, $2)",
							player_to_update.uuid,
							previous_username
						)
						.execute(&mut *transaction)
						.await?;
					}
				}

				query!(
					"INSERT INTO players(uuid, username) VALUES ($1, $2) ON CONFLICT (uuid) DO UPDATE SET username = $2",
					player_to_update.uuid,
					&player_to_update.username
				)
				.execute(&mut *transaction)
				.await?;
			}
		}
	}

	let BasicUserInfo { uuid, username } = user;

	// evict all expired tokens for the authenticating user
	query!("DELETE FROM tokens WHERE player = $1 AND expired = true", uuid)
		.execute(&mut *transaction)
		.await?;

	let access_token = loop {
		let mut hasher = Blake2b512::new();
		hasher.update(&*username);
		hasher.update(uuid);
		hasher.update(&server_id);
		let random: [u8; 16] = rand::random();
		hasher.update(random);

		let potential_access_token = hasher.finalize();
		let potential_access_token_bytes = potential_access_token.as_slice();
		let result = query!("INSERT INTO tokens(token, player) VALUES ($1, $2)", potential_access_token_bytes, uuid)
			.execute(&mut *transaction)
			.await;

		match result {
			Ok(_) => break STANDARD_NO_PAD.encode(potential_access_token),
			Err(error) => match error {
				sqlx::Error::Database(ref database_error) => match database_error.kind() {
					UniqueViolation => continue,
					_ => return Err(error)?,
				},
				_ => return Err(error)?,
			},
		}
	};

	query!("UPDATE players SET last_online = LOCALTIMESTAMP WHERE uuid = $1 AND show_last_online = true", uuid)
		.execute(&mut *transaction)
		.await?;

	transaction.commit().await?;

	Ok(Json(AuthenticateResponse {
		uuid,
		username,
		access_token,
	}))
}

pub async fn brew_coffee() -> impl IntoResponse {
	(StatusCode::IM_A_TEAPOT, "I'm a Teapot")
}

pub async fn not_found() -> impl IntoResponse {
	(StatusCode::NOT_FOUND, "Not Found")
}
