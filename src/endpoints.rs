use crate::{errors::ApiError, extractors::Authentication, extractors::Query, ApiState};
use axum::{extract::Path, extract::State, http::StatusCode, http::Uri, Json};
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};
use blake2::{Blake2b512, Digest};
use chrono::NaiveDateTime;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::{error::ErrorKind::UniqueViolation, query, query_as, SqliteConnection, SqlitePool};
use std::{future::Future, pin::Pin};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct Authenticate {
	username: String,
	server_id: String,
}

#[derive(Deserialize)]
struct BasicUserInfo {
	#[serde(rename = "id")]
	uuid: Uuid,
	#[serde(rename = "name")]
	username: String,
}

#[derive(Serialize)]
pub struct AuthenticateResponse {
	uuid: Uuid,
	username: String,
	access_token: String,
}

pub async fn authenticate(
	State(ApiState { database, client }): State<ApiState>,
	Query(Authenticate { username, server_id }): Query<Authenticate>,
) -> Result<Json<AuthenticateResponse>, ApiError> {
	let response = client
		.get("https://sessionserver.mojang.com/session/minecraft/hasJoined")
		.query(&[("username", &username), ("serverId", &server_id)])
		.send()
		.await?;

	let response: BasicUserInfo = match response.status() {
		reqwest::StatusCode::OK => response.json().await?,
		reqwest::StatusCode::NO_CONTENT => return Err(ApiError::authentication_failed()),
		_ => return Err(ApiError::internal_server_error()),
	};

	let response = BasicUserInfo {
		uuid: Uuid::parse_str("7c85c805-a137-4671-bc79-89c8480c2548").unwrap(),
		username: response.username,
	};

	let BasicUserInfo { uuid, username } = response;

	let mut transaction = database.begin().await?;
	let uuid_ref = uuid.as_ref();

	// Step 1: Make sure someone else doesn't have the username
	// Yes I know recursion performs terribly, but we're doing IO here, so it has basically zero effect + this is easier
	// ~~Okay on second thought after writing this recursive async madness, might have been easier to do a loop~~
	fn ensure_username_not_in_use<'f>(
		client: &'f Client,
		database: &'f mut SqliteConnection,
		uuid: &'f Uuid,
		username: &'f String,
	) -> Pin<Box<dyn Future<Output = Result<(), ApiError>> + 'f>> {
		Box::pin(async move {
			let uuid_ref = uuid.as_ref();
			let other_user = query_as!(
				BasicUserInfo,
				"SELECT username, uuid AS 'uuid: Uuid' FROM users WHERE uuid != ? AND username = ?",
				uuid_ref,
				username
			)
			.fetch_optional(&mut *database)
			.await?;

			if let Some(BasicUserInfo { uuid, .. }) = other_user {
				// Someone else had the username, this means they changed username and we need to update
				let BasicUserInfo {
					uuid,
					username: new_username,
				} = client
					.get(format!("https://sessionserver.mojang.com/session/minecraft/profile/{uuid}"))
					.send()
					.await?
					.json()
					.await?;

				ensure_username_not_in_use(client, &mut *database, &uuid, &new_username).await?;

				query!(
					"UPDATE users SET username = ? WHERE uuid = ?;\
					 INSERT INTO old_usernames(user, username) VALUES (?, ?)",
					new_username,
					uuid_ref,
					uuid_ref,
					username
				)
				.execute(database)
				.await?;
			}

			Ok(())
		})
	}

	ensure_username_not_in_use(&client, &mut transaction, &uuid, &username);

	// Step 2: Make sure the user exists in the database
	query!("INSERT OR IGNORE INTO users(uuid, username) VALUES (?, ?)", uuid_ref, username)
		.fetch_optional(&mut *transaction)
		.await?;

	// Step 3: Update the user's username
	let old_username = query!("SELECT username FROM users WHERE uuid = ?", uuid_ref)
		.fetch_one(&mut *transaction)
		.await?
		.username;
	if old_username.to_lowercase() != username.to_lowercase() {
		query!(
			"UPDATE users SET username = ? WHERE uuid = ?;\
			 INSERT INTO old_usernames(user, username) VALUES (?, ?)",
			username,
			uuid_ref,
			uuid_ref,
			old_username
		)
		.execute(&mut *transaction)
		.await?;
	}

	// Step 4: Generate access token
	let access_token = loop {
		let mut hasher = Blake2b512::new();
		hasher.update(&username);
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

	// Step 5: Update last activity
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
}

pub async fn get_user_public(
	State(ApiState { database, .. }): State<ApiState>,
	Path(uuid): Path<Uuid>,
) -> Result<Json<PublicUser>, ApiError> {
	let uuid_ref = uuid.as_ref();
	let user = query_as!(
		PublicUser,
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
	.fetch_one(&database)
	.await?;

	Ok(Json(user))
}

pub async fn delete_user(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
) -> Result<StatusCode, ApiError> {
	let uuid_ref = uuid.as_ref();
	query!("DELETE FROM users WHERE uuid = ?", uuid_ref)
		.execute(&database)
		.await?;

	Ok(StatusCode::NO_CONTENT)
}

#[derive(Serialize)]
pub struct User {
	uuid: Uuid,
	username: String,
	registered: NaiveDateTime,
	last_activity: NaiveDateTime,
}

impl User {
	pub async fn get(database: &SqlitePool, uuid: &Uuid) -> Result<User, ApiError> {
		let uuid_ref = uuid.as_ref();
		Ok(query_as!(
			User,
			"SELECT uuid AS 'uuid: Uuid', username, registered, last_activity FROM users WHERE uuid = ?",
			uuid_ref
		)
		.fetch_one(database)
		.await?)
	}
}

pub async fn get_user(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
) -> Result<Json<User>, ApiError> {
	Ok(Json(User::get(&database, &uuid).await?))
}

#[derive(Serialize)]
pub struct Settings {
	show_registered: bool,
	show_last_activity: bool,
}

impl Settings {
	pub async fn get(database: &SqlitePool, uuid: &Uuid) -> Result<Settings, ApiError> {
		let uuid_ref = uuid.as_ref();
		Ok(query_as!(Settings, "SELECT show_registered, show_last_activity FROM users WHERE uuid = ?", uuid_ref)
			.fetch_one(database)
			.await?)
	}
}

pub async fn get_user_settings(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
) -> Result<Json<Settings>, ApiError> {
	Ok(Json(Settings::get(&database, &uuid).await?))
}

#[derive(Deserialize)]
pub struct SettingsPatch {
	show_registered: Option<bool>,
	show_last_activity: Option<bool>,
}

pub async fn patch_user_settings(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
	Json(user_settings_patch): Json<SettingsPatch>,
) -> Result<StatusCode, ApiError> {
	let uuid_ref = uuid.as_ref();
	query!(
		r#"
			UPDATE users SET
				show_registered = coalesce(?, show_registered),
				show_last_activity = coalesce(?, show_last_activity)
			WHERE uuid = ?
		"#,
		user_settings_patch.show_registered,
		user_settings_patch.show_last_activity,
		uuid_ref
	)
	.execute(&database)
	.await?;

	Ok(StatusCode::NO_CONTENT)
}

#[derive(Serialize)]
pub struct UserData {
	user: User,
	settings: Settings,
	previous_usernames: Vec<PreviousUsername>,
}

#[derive(Serialize)]
pub struct PreviousUsername {
	username: String,
	show: bool,
}

pub async fn get_user_data(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
) -> Result<Json<UserData>, ApiError> {
	let uuid_ref = uuid.as_ref();

	let previous_usernames =
		query_as!(PreviousUsername, "SELECT username, show FROM old_usernames WHERE user = ?", uuid_ref)
			.fetch_all(&database)
			.await?;

	Ok(Json(UserData {
		user: User::get(&database, &uuid).await?,
		settings: Settings::get(&database, &uuid).await?,
		previous_usernames,
	}))
}

pub async fn brew_coffee() -> ApiError {
	ApiError::im_a_teapot()
}

pub async fn not_found(uri: Uri) -> ApiError {
	ApiError::not_found(uri.path())
}
