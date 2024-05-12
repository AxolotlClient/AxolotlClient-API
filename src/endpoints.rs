use crate::{errors::ApiError, extractors::Authentication, ApiState};
use axum::{extract::Path, extract::Query, extract::State, http::StatusCode, response::IntoResponse, Json};
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};
use blake2::{Blake2b512, Digest};
use chrono::NaiveDateTime;
use garde::Validate;
use serde::{Deserialize, Serialize};
use sqlx::{error::ErrorKind::UniqueViolation, query, query_as, SqlitePool, Type};
use std::ops::Deref;
use uuid::Uuid;

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

	while let Some(user_to_update) = usernames_to_update.pop() {
		let uuid_ref: &[u8] = user_to_update.uuid.as_ref();
		let existing_user_with_name = query_as!(
			BasicUserInfo,
			"SELECT username, uuid AS 'uuid: Uuid' FROM users WHERE uuid != ? AND username = ?",
			uuid_ref,
			user_to_update.username
		)
		.fetch_optional(&mut *transaction)
		.await?;

		match existing_user_with_name {
			Some(existing_user_with_name) => {
				let existing_user_with_name = client
					.get(format!(
						"https://sessionserver.mojang.com/session/minecraft/profile/{}",
						existing_user_with_name.uuid
					))
					.send()
					.await?
					.json()
					.await?;

				usernames_to_update.push(user_to_update);
				usernames_to_update.push(existing_user_with_name);
			}
			None => {
				let old_username = query!("SELECT username FROM users WHERE uuid == ? AND retain_usernames", uuid_ref)
					.fetch_optional(&mut *transaction)
					.await?
					.map(|record| record.username);

				if let Some(old_username) = old_username {
					query!("INSERT INTO old_usernames(user, username) VALUES (?, ?)", uuid_ref, old_username)
						.execute(&mut *transaction)
						.await?;
				}

				query!(
					"INSERT INTO users(uuid, username) VALUES (?, ?) ON CONFLICT (uuid) DO UPDATE SET username = ?",
					uuid_ref,
					user_to_update.username,
					user_to_update.username
				)
				.execute(&mut *transaction)
				.await?;
			}
		}
	}

	let BasicUserInfo { uuid, username } = user;
	let uuid_ref: &[u8] = uuid.as_ref();

	let access_token = loop {
		let mut hasher = Blake2b512::new();
		hasher.update(&*username);
		hasher.update(uuid);
		hasher.update(&server_id);
		let random: [u8; 16] = rand::random();
		hasher.update(random);

		let potential_access_token = hasher.finalize();
		let potential_access_token_bytes = potential_access_token.as_slice();
		let result = query!("INSERT INTO tokens(token, user) VALUES (?, ?)", potential_access_token_bytes, uuid_ref)
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

	query!("UPDATE users SET last_activity = CURRENT_TIMESTAMP WHERE uuid = ?", uuid_ref)
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
pub struct PublicUser {
	uuid: Uuid,
	username: String,
	registered: Option<NaiveDateTime>,
	last_activity: Option<NaiveDateTime>,
	old_usernames: Vec<String>,
}

pub async fn get_user(
	State(ApiState { database, .. }): State<ApiState>,
	Path(uuid): Path<Uuid>,
) -> Result<Json<PublicUser>, ApiError> {
	let uuid_ref: &[u8] = uuid.as_ref();
	let user = query!(
		r#"
			SELECT
				uuid AS 'uuid: Uuid',
				username,
				IIF(show_registered, registered, NULL) AS registered,
				IIF(show_last_activity, last_activity, NULL) AS last_activity
			FROM users WHERE uuid = ?
		"#,
		uuid_ref
	)
	.fetch_optional(&database)
	.await?
	.ok_or(StatusCode::NOT_FOUND)?;

	let usernames = query!("SELECT username FROM old_usernames WHERE user = ? AND public", uuid_ref)
		.fetch_all(&database)
		.await?
		.into_iter()
		.map(|record| record.username)
		.collect();

	Ok(Json(PublicUser {
		uuid: user.uuid,
		username: user.username,
		registered: user.registered,
		last_activity: user.last_activity,
		old_usernames: usernames,
	}))
}

#[derive(Serialize)]
pub struct User {
	uuid: Uuid,
	username: String,
	registered: NaiveDateTime,
	last_activity: NaiveDateTime,
	old_usernames: Vec<OldUsername>,
}

#[derive(Serialize)]
pub struct OldUsername {
	username: String,
	public: bool,
}

impl User {
	pub async fn get(database: &SqlitePool, uuid: &Uuid) -> Result<User, ApiError> {
		let uuid_ref: &[u8] = uuid.as_ref();
		let user = query!(
			"SELECT uuid AS 'uuid: Uuid', username, registered, last_activity FROM users WHERE uuid = ?",
			uuid_ref
		)
		.fetch_one(database)
		.await?;
		let old_usernames =
			query_as!(OldUsername, "SELECT username, public FROM old_usernames WHERE user = ?", uuid_ref)
				.fetch_all(database)
				.await?;

		Ok(User {
			uuid: user.uuid,
			username: user.username,
			registered: user.registered,
			last_activity: user.last_activity,
			old_usernames,
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
	let uuid_ref: &[u8] = uuid.as_ref();
	query!("DELETE FROM users WHERE uuid = ?", uuid_ref)
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
	show_last_activity: bool,
	retain_usernames: bool,
}

impl Settings {
	pub async fn get(database: &SqlitePool, uuid: &Uuid) -> Result<Settings, ApiError> {
		let uuid_ref: &[u8] = uuid.as_ref();
		Ok(query_as!(
			Settings,
			"SELECT show_registered, show_last_activity, retain_usernames FROM users WHERE uuid = ?",
			uuid_ref
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
	show_last_activity: Option<bool>,
	retain_usernames: Option<bool>,
}

pub async fn patch_account_settings(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
	Json(user_settings_patch): Json<SettingsPatch>,
) -> Result<StatusCode, ApiError> {
	let uuid_ref: &[u8] = uuid.as_ref();
	query!(
		r#"
			UPDATE users SET
				show_registered = coalesce(?, show_registered),
				show_last_activity = coalesce(?, show_last_activity),
				retain_usernames = coalesce(?, retain_usernames)
			WHERE uuid = ?
		"#,
		user_settings_patch.show_registered,
		user_settings_patch.show_last_activity,
		user_settings_patch.retain_usernames,
		uuid_ref
	)
	.execute(&database)
	.await?;

	Ok(StatusCode::NO_CONTENT)
}

pub async fn post_account_username(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
	Path(username): Path<String>,
	Query(public): Query<bool>,
) -> Result<StatusCode, ApiError> {
	let uuid_ref: &[u8] = uuid.as_ref();
	let rows_affected =
		query!("UPDATE old_usernames SET public = ? WHERE user = ? AND username = ?", public, uuid_ref, username)
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
	let uuid_ref: &[u8] = uuid.as_ref();
	query!("DELETE FROM old_usernames WHERE user = ? AND username = ?", uuid_ref, username)
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
