use std::collections::HashMap;

use crate::{errors::ApiError, extractors::Authentication, id::Id, ApiState};
use axum::{extract::Path, extract::Query, extract::State, Json};
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};
use chrono::{DateTime, TimeDelta, Utc};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::{query, query_as, PgPool};
use uuid::Uuid;

use super::{channel::Persistence, user::Activity};

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
	relations: HashMap<Uuid, String>,
	channels: Vec<ChannelExport>,
	channel_invites: Vec<ChannelInvitesExport>,
	images: Vec<ImageExport>,
}

pub async fn get_data(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
) -> Result<Json<UserData>, ApiError> {
	Ok(Json(UserData {
		user: User::get(&database, &uuid).await?,
		settings: Settings::get(&database, &uuid).await?,
		relations: UserData::get_relations(&database, &uuid).await?,
		channels: ChannelExport::get(&database, &uuid).await?,
		channel_invites: ChannelInvitesExport::get(&database, &uuid).await?,
		images: ImageExport::get(&database, &uuid).await?,
	}))
}

impl UserData {
	pub async fn get_relations(database: &PgPool, uuid: &Uuid) -> Result<HashMap<Uuid, String>, ApiError> {
		let relations =
			query!(r#"SELECT player_b, relation as "relation: String" FROM relations WHERE player_a = $1"#, uuid)
				.fetch_all(database)
				.await?;

		let mut map: HashMap<Uuid, String> = HashMap::new();
		for en in relations {
			map.insert(en.player_b, en.relation);
		}
		return Ok(map);
	}
}

#[derive(Serialize)]
pub struct Settings {
	show_registered: bool,
	retain_usernames: bool,
	show_last_online: bool,
	show_activity: bool,
	allow_friends_image_access: bool,
}

impl Settings {
	pub async fn get(database: &PgPool, uuid: &Uuid) -> Result<Settings, ApiError> {
		Ok(query_as!(
			Settings,
			"SELECT show_registered, retain_usernames, show_last_online, show_activity, allow_friends_image_access FROM players WHERE uuid = $1",
			uuid
		)
		.fetch_one(database)
		.await?)
	}
}

#[derive(Serialize)]
pub struct ChannelExport {
	id: u64,
	name: String,
	settings: Option<ChannelSettingsExport>,
	participants: Option<Vec<Uuid>>,
	messages: Vec<MessageExport>,
}

#[derive(Serialize)]
pub struct ChannelSettingsExport {
	created: DateTime<Utc>,
	last_updated: DateTime<Utc>,
	last_message: DateTime<Utc>,
	persistence: Persistence,
}

#[derive(Serialize)]
pub struct MessageExport {
	id: u64,
	sender_name: String,
	content: String,
	send_time: DateTime<Utc>,
}

impl ChannelExport {
	pub async fn get(database: &PgPool, uuid: &Uuid) -> Result<Vec<ChannelExport>, ApiError> {
		let mut response = Vec::new();

		let owned = query!("SELECT * FROM channels WHERE owner = $1", uuid)
			.fetch_all(database)
			.await?;
		let participating = query!("SELECT channels FROM channel_memberships WHERE player = $1", uuid)
			.fetch_optional(database)
			.await?;

		for en in owned {
			response.push(ChannelExport {
				id: en.id as u64,
				name: en.name,
				settings: Some(ChannelSettingsExport {
					created: en.created.and_utc(),
					last_updated: en.last_updated.and_utc(),
					last_message: en.last_message.and_utc(),
					persistence: Persistence::from(
						en.persistence,
						en.persistence_count.map(|i| i as u32),
						en.persistence_duration_seconds.map(TimeDelta::seconds),
					)
					.unwrap(),
				}),
				participants: Some(
					query!("SELECT * FROM channel_memberships WHERE $1 = ANY(channels)", &en.id)
						.fetch_all(database)
						.await?
						.iter()
						.map(|rec| rec.player)
						.collect(),
				),
				messages: ChannelExport::get_messages(database, uuid, en.id).await?,
			});
		}

		if let Some(rec) = participating {
			for en in rec.channels {
				response.push(ChannelExport {
					id: en as u64,
					settings: None,
					participants: None,
					name: query!("SELECT name FROM channels WHERE id = $1", en)
						.fetch_one(database)
						.await?
						.name,
					messages: ChannelExport::get_messages(database, uuid, en).await?,
				});
			}
		}
		return Ok(response);
	}

	async fn get_messages(database: &PgPool, uuid: &Uuid, channel_id: i64) -> Result<Vec<MessageExport>, ApiError> {
		Ok(query!(
			"SELECT id, sender_name, content, send_time FROM messages WHERE channel_id = $1 AND sender = $2",
			channel_id,
			uuid
		)
		.fetch_all(database)
		.await?
		.iter()
		.map(|rec| MessageExport {
			id: rec.id as u64,
			sender_name: rec.sender_name.clone(),
			content: rec.content.clone(),
			send_time: rec.send_time.and_utc(),
		})
		.collect())
	}
}

#[derive(Serialize)]
pub struct ChannelInvitesExport {
	channel: u64,
	from: Uuid,
}

impl ChannelInvitesExport {
	pub async fn get(database: &PgPool, uuid: &Uuid) -> Result<Vec<ChannelInvitesExport>, ApiError> {
		Ok(query!("SELECT channel, sender AS from FROM channel_invites WHERE player = $1", uuid)
			.fetch_all(database)
			.await?
			.iter()
			.map(|r| ChannelInvitesExport {
				channel: r.channel as u64,
				from: r.from,
			})
			.collect())
	}
}

#[derive(Serialize)]
pub struct ImageExport {
	id: u64,
	filename: String,
	file: String,
	timestamp: DateTime<Utc>,
}

impl ImageExport {
	pub async fn get(database: &PgPool, uuid: &Uuid) -> Result<Vec<ImageExport>, ApiError> {
		Ok(query!("SELECT id, filename, file, timestamp FROM images WHERE player = $1", uuid)
			.fetch_all(database)
			.await?
			.iter()
			.map(|rec| ImageExport {
				id: rec.id as u64,
				filename: String::from_utf8(rec.filename.clone()).unwrap(),
				file: STANDARD_NO_PAD.encode(rec.file.clone()),
				timestamp: rec.timestamp.and_utc(),
			})
			.collect())
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
	allow_friends_image_access: Option<bool>,
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
				show_activity = coalesce($4, show_activity),
				allow_friends_image_access = coalesce($5, allow_friends_image_access)
			WHERE uuid = $6
		"#,
		user_settings_patch.show_registered,
		user_settings_patch.retain_usernames,
		user_settings_patch.show_last_online,
		user_settings_patch.show_activity,
		user_settings_patch.allow_friends_image_access,
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

	if online_users.contains_key(&uuid) {
		online_users.insert(uuid, Some(activity));
	}

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
