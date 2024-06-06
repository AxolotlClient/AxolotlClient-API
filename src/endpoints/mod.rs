use crate::{errors::ApiError, extractors::Authentication, ApiState};
use axum::{extract::Path, extract::Query, extract::State, http::StatusCode, response::IntoResponse, Json};
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};
use blake2::{Blake2b512, Digest};
use chrono::NaiveDateTime;
use garde::Validate;
use serde::{Deserialize, Serialize};
use sqlx::{error::ErrorKind::UniqueViolation, query, query_as, query_scalar, PgPool, Type};
use std::ops::Deref;
use uuid::Uuid;

// pub mod channel;
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
					query!(
						"INSERT INTO previous_usernames(player, username) VALUES ($1, $2)",
						player_to_update.uuid,
						previous_username
					)
					.execute(&mut *transaction)
					.await?;
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

	query!("UPDATE players SET last_online = 'now' WHERE uuid = $1 AND show_last_online = true", uuid)
		.execute(&mut *transaction)
		.await?;

	transaction.commit().await?;

	Ok(Json(AuthenticateResponse {
		uuid,
		username,
		access_token,
	}))
}

#[derive(Serialize)]
pub struct User {
	uuid: Uuid,
	username: String,
	registered: NaiveDateTime,
	last_online: Option<NaiveDateTime>,
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
			registered: user.registered,
			last_online: user.last_online,
			previous_usernames,
		})
	}
}

pub async fn get_account(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
) -> Result<Json<User>, ApiError> {
	Ok(Json(User::get(&database, &uuid).await?))
}

pub async fn delete_account(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
) -> Result<StatusCode, ApiError> {
	query!("DELETE FROM players WHERE uuid = $1", uuid)
		.execute(&database)
		.await?;

	Ok(StatusCode::NO_CONTENT)
}

#[derive(Serialize)]
pub struct UserData {
	user: User,
	settings: Settings,
}

pub async fn get_account_data(
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

pub async fn get_account_settings(
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

pub async fn patch_account_settings(
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

pub async fn post_account_username(
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

pub async fn delete_account_username(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
	Path(username): Path<String>,
) -> Result<StatusCode, ApiError> {
	query!("DELETE FROM previous_usernames WHERE player = $1 AND username = $2", uuid, username)
		.execute(&database)
		.await?;
	Ok(StatusCode::NOT_FOUND)
}

pub async fn brew_coffee() -> impl IntoResponse {
	(StatusCode::IM_A_TEAPOT, "I'm a Teapot")
}

pub async fn not_found() -> impl IntoResponse {
	(StatusCode::NOT_FOUND, "Not Found")
}
