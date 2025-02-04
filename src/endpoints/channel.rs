use crate::{errors::ApiError, extractors::Authentication, id::Id, ApiState};
use axum::{
	extract::{Path, Query, State},
	Json,
};
use chrono::{DateTime, Duration, TimeDelta, Utc};
use garde::Validate;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{query, PgPool};
use std::str::FromStr;
use uuid::Uuid;

#[derive(Deserialize, Serialize)]
pub struct Channel {
	id: Id,
	#[serde(flatten)]
	channel_data: ChannelData,
}

#[derive(Deserialize, Serialize, Validate)]
#[garde(allow_unvalidated)]
pub struct ChannelData {
	#[garde(length(min = 1, max = 32))]
	name: String,
	#[serde(skip_deserializing)]
	owner: Uuid,
	persistence: Persistence,
	participants: Vec<Uuid>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Persistence {
	/// Delete messages when the channel is deleted
	Channel,
	/// Delete messages X time after they are sent
	Duration {
		#[serde(with = "duration")]
		duration: Duration,
	},
	/// Delete all but the latest X messages
	Count { count: u32 },
	/// Delete all but the latest X messages X time after they are sent
	CountAndDuration {
		count: u32,

		#[serde(with = "duration")]
		duration: Duration,
	},
}

impl Persistence {
	fn id(&self) -> u8 {
		match self {
			Self::Channel => 0,
			Self::Duration { .. } => 1,
			Self::Count { .. } => 2,
			Self::CountAndDuration { .. } => 3,
		}
	}

	fn count(&self) -> Option<&u32> {
		match self {
			Self::Count { count } | Self::CountAndDuration { count, .. } => Some(count),
			_ => None,
		}
	}

	fn duration(&self) -> Option<&Duration> {
		match self {
			Self::Duration { duration } | Self::CountAndDuration { duration, .. } => Some(duration),
			_ => None,
		}
	}

	pub fn from(id: i16, count: Option<u32>, duration: Option<Duration>) -> Option<Persistence> {
		match id {
			0 => Some(Self::Channel),
			1 => duration.map(|duration| Self::Duration { duration }),
			2 => count.map(|count| Self::Count { count }),
			3 => {
				if let Some(count) = count {
					if let Some(duration) = duration {
						return Some(Self::CountAndDuration { count, duration });
					}
				}
				None
			}
			_ => None,
		}
	}
}

async fn get_channel(database: &PgPool, uuid: &Uuid, channel_id: Id) -> Result<Channel, ApiError> {
	let channel = query!(
		r#"SELECT id,
			name,
		    owner,
			persistence,
			persistence_count, 
			persistence_duration_seconds
			FROM channels WHERE id = $1"#,
		&channel_id as _
	)
	.fetch_optional(database)
	.await?
	.ok_or(StatusCode::BAD_REQUEST)?;

	let participants: Vec<Uuid> =
		query!("SELECT * FROM channel_memberships WHERE $1 = ANY(channels)", &channel_id as _)
			.fetch_all(database)
			.await?
			.iter()
			.map(|rec| rec.player)
			.collect();

	if &channel.owner == uuid || participants.contains(&uuid) {
		if let Some(persistence) = Persistence::from(
			channel.persistence,
			channel.persistence_count.map(|i| i as u32),
			channel.persistence_duration_seconds.map(TimeDelta::seconds),
		) {
			return Ok(Channel {
				id: channel_id,
				channel_data: ChannelData {
					name: channel.name,
					owner: channel.owner,
					persistence,
					participants,
				},
			});
		}
	}

	Err(StatusCode::BAD_REQUEST)?
}

pub async fn get(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
	Path(channel_id): Path<Id>,
) -> Result<Json<Channel>, ApiError> {
	Ok(Json(get_channel(&database, &uuid, channel_id).await?))
}

pub async fn delete(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
	Path(channel_id): Path<Id>,
) -> Result<StatusCode, ApiError> {
	let channel = get_channel(&database, &uuid, channel_id).await?;

	if channel.channel_data.owner == uuid {
		query!("DELETE FROM channels WHERE id = $1", &channel.id as _)
			.execute(&database)
			.await?;
		query!(
			"UPDATE channel_memberships SET channels = array_remove(channels, $1) WHERE $1 = ANY(channels)",
			&channel.id as _
		)
		.execute(&database)
		.await?;
	} else {
		query!("UPDATE channel_memberships SET channels = array_remove(channels, $2) WHERE player = $1 AND $2 = ANY(channels)", &uuid, &channel.id as _)
		.execute(&database).await?;
	}

	Ok(StatusCode::OK)
}

pub async fn post(
	State(ApiState {
		database,
		socket_sender,
		..
	}): State<ApiState>,
	Authentication(owner): Authentication,
	Json(channel_data): Json<ChannelData>,
) -> Result<String, ApiError> {
	channel_data.validate()?;

	let id = Id::new();
	let persistence = channel_data.persistence.id();
	let persistence_count = channel_data.persistence.count();
	let persistence_duration_seconds = channel_data
		.persistence
		.duration()
		.map(|duration| duration.num_seconds());
	let participants = channel_data.participants;

	let mut transaction = database.begin().await?;
	query!(
		r#"INSERT INTO channels(
			id,
			name,
			owner,
			persistence,
			persistence_count,
			persistence_duration_seconds
		) VALUES ($1, $2, $3, $4, $5, $6)"#,
		&id as _,
		channel_data.name,
		owner,
		persistence as i16,
		persistence_count.map(|c| *c as i32),
		persistence_duration_seconds
	)
	.execute(&mut *transaction)
	.await?;

	let friends: Vec<Uuid> =
		query!("SELECT player_b FROM relations WHERE relation = 'friend' AND player_a = $1", &owner)
			.fetch_all(&database)
			.await?
			.into_iter()
			.map(|r| r.player_b)
			.collect();
	for uuid in participants {
		if friends.contains(&uuid) {
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
		} else {
			query!(
				"INSERT INTO channel_invites (channel, player, sender) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
				&id as _,
				uuid,
				&owner
			)
			.execute(&mut *transaction)
			.await?;
			if let Some(socket) = socket_sender.get(&uuid) {
				let _ = socket.send(
					serde_json::to_string(&json!({
						"target": "channel_invite",
						"channel": &id,
						"channel_name": &channel_data.name,
						"from": &channel_data.owner
					}))
					.unwrap(),
				);
			}
		}
	}

	transaction.commit().await?;

	Ok(id.to_string())
}

pub async fn patch(
	State(ApiState {
		database,
		socket_sender,
		..
	}): State<ApiState>,
	Authentication(uuid): Authentication,
	Path(channel_id): Path<Id>,
	Json(value): Json<Value>,
) -> Result<StatusCode, ApiError> {
	let mut transaction = database.begin().await?;

	let channel = query!(
		r#"SELECT id,
			name,
		    owner,
			persistence,
			persistence_count, 
			persistence_duration_seconds
			FROM channels WHERE id = $1"#,
		channel_id as _
	)
	.fetch_optional(&mut *transaction)
	.await?
	.ok_or(StatusCode::BAD_REQUEST)?;

	if channel.owner == uuid {
		if let Some(mut persistence) = Persistence::from(
			channel.persistence,
			channel.persistence_count.map(|i| i as u32),
			channel.persistence_duration_seconds.map(TimeDelta::seconds),
		) {
			let mut name = channel.name;
			let mut participants: Vec<Uuid> = Vec::new();
			if let Some(val) = value.get("name") {
				name = val.as_str().ok_or(StatusCode::BAD_REQUEST)?.to_string()
			}
			if let Some(uuids) = value.get("participants") {
				let vec = uuids.as_array().unwrap();
				for val in vec {
					let str = val.as_str().ok_or(StatusCode::BAD_REQUEST)?;
					let uuid = Uuid::from_str(str).map_err(|_| StatusCode::BAD_REQUEST)?;
					participants.push(uuid);
				}
			}
			if let Some(val) = value.get("persistence") {
				persistence = serde_json::from_value(val.clone()).map_err(|_| StatusCode::BAD_REQUEST)?;
			}

			let persistence_id = persistence.id() as i16;
			let persistence_count = persistence.count();
			let persistence_duration_seconds = persistence.duration().map(|duration| duration.num_seconds());
			query!(
				r#"UPDATE channels SET
					name = coalesce($1, name),
					persistence = coalesce($2, persistence),
					persistence_count = coalesce($3, persistence_count),
					persistence_duration_seconds = coalesce($4, persistence_duration_seconds),
					last_updated = LOCALTIMESTAMP
					WHERE id = $5"#,
				name,
				persistence_id as _,
				persistence_count.map(|c| *c as i32),
				persistence_duration_seconds,
				channel_id as _
			)
			.execute(&mut *transaction)
			.await?;

			let friends: Vec<Uuid> =
				query!("SELECT player_b FROM relations WHERE relation = 'friend' AND player_a = $1", &channel.owner)
					.fetch_all(&database)
					.await?
					.into_iter()
					.map(|r| r.player_b)
					.collect();
			// Tried to use batch insert via UNNEST here, however Postgres was not cooperating.
			// Given that this isn't likely to be more then a few players, the cost here is negligible for the time being.
			for player in participants {
				if friends.contains(&player) {
					query!(
						r#"INSERT INTO channel_memberships(player, channels)
					 VALUES ($1, ARRAY [$2::bigint]) 
					 ON CONFLICT (player) DO UPDATE 
					 SET channels = ARRAY_APPEND(channel_memberships.channels, $2) 
					 WHERE channel_memberships.player = $1"#,
						player,
						channel_id as _
					)
					.execute(&mut *transaction)
					.await?;
				} else {
					query!(
						"INSERT INTO channel_invites (channel, player, sender) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
						channel_id as _,
						player,
						&channel.owner
					)
					.execute(&mut *transaction)
					.await?;
					if let Some(socket) = socket_sender.get(&player) {
						let _ = socket.send(
							serde_json::to_string(&json!({
								"target": "channel_invite",
								"channel": &channel_id,
								"channel_name": name.clone(),
								"from": &channel.owner
							}))
							.unwrap(),
						);
					}
				}
			}

			transaction.commit().await?;
			return Ok(StatusCode::NO_CONTENT);
		}
	}

	Err(StatusCode::BAD_REQUEST)?
}

pub async fn post_channel(
	State(ApiState {
		database,
		online_users,
		socket_sender,
		..
	}): State<ApiState>,
	Authentication(uuid): Authentication,
	Path(channel_id): Path<Id>,
	Json(PostMessage { content, display_name }): Json<PostMessage>,
) -> Result<String, ApiError> {
	let channel = get_channel(&database, &uuid, channel_id).await?;

	let mut transaction = database.begin().await?;

	let id = Id::new();
	query!(
		"INSERT INTO messages (id, channel_id, sender, sender_name, content) VALUES ($1, $2, $3, $4, $5)",
		&id as _,
		&channel.id as _,
		uuid,
		display_name,
		content
	)
	.execute(&mut *transaction)
	.await?;

	query!("UPDATE channels SET last_message = LOCALTIMESTAMP WHERE id = $1", &channel.id as _)
		.execute(&mut *transaction)
		.await?;

	transaction.commit().await?;
	let message = serde_json::to_string(&json!({
		"target": "chat_message",
		"channel": &channel.id,
		"id": &id,
		"sender": &uuid,
		"sender_name": display_name,
		"content": content
	}))
	.unwrap();

	if uuid != channel.channel_data.owner && online_users.contains_key(&channel.channel_data.owner) {
		socket_sender
			.get(&channel.channel_data.owner)
			.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
			.value()
			.send(message.clone())
			.unwrap();
	}
	for participant in channel.channel_data.participants {
		if participant != uuid && online_users.contains_key(&participant) {
			socket_sender
				.get(&participant)
				.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
				.value()
				.send(message.clone())
				.unwrap();
		}
	}
	Ok(id.to_string())
}

pub async fn get_messages(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
	Path(channel_id): Path<Id>,
	Query(QueryBefore { before }): Query<QueryBefore>,
) -> Result<Json<Vec<Message>>, ApiError> {
	let channel = get_channel(&database, &uuid, channel_id).await?;

	let records = query!(
		"SELECT id, channel_id, sender, sender_name, content, send_time FROM messages WHERE channel_id = $1 AND send_time < $2 LIMIT 50",
		&channel.id as _,
		before.unwrap_or(Utc::now()) as _
	)
	.fetch_all(&database)
	.await?;

	let mut messages: Vec<Message> = records
		.iter()
		.map(|m| Message {
			id: m.id as u64,
			channel_id: m.channel_id as u64,
			sender: m.sender,
			sender_name: m.sender_name.clone(),
			content: m.content.clone(),
			timestamp: m.send_time.and_utc(),
		})
		.collect();
	messages.sort_by_key(|m| m.timestamp);

	Ok(Json(messages))
}

pub async fn remove_user(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(owner): Authentication,
	Path(channel_id): Path<Id>,
	Query(uuid): Query<Uuid>,
) -> Result<StatusCode, ApiError> {
	let channel = get_channel(&database, &uuid, channel_id).await?;
	if channel.channel_data.owner == owner {
		query!(
			"UPDATE channel_memberships SET channels = ARRAY_REMOVE(channels, $1) WHERE player = $2",
			&channel.id as _,
			uuid
		)
		.execute(&database)
		.await?;
		return Ok(StatusCode::OK);
	}
	Ok(StatusCode::BAD_REQUEST)
}

/*pub async fn report_message(State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
	Path(message_id): Path<Id>,
) -> Result<StatusCode, ApiError> {
	todo!("Store reports somewhere and notify us of them!");
	//Ok(StatusCode::BAD_REQUEST)
}*/

#[derive(Serialize)]
pub struct Message {
	id: u64,
	channel_id: u64,
	sender: Uuid,
	sender_name: String,
	content: String,
	timestamp: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct QueryBefore {
	before: Option<DateTime<Utc>>,
}

#[derive(Deserialize)]
pub struct PostMessage {
	content: String,
	display_name: String,
}

mod duration {
	use chrono::Duration;
	use serde::{Deserialize, Deserializer, Serializer};

	pub fn deserialize<'d, D: Deserializer<'d>>(deserializer: D) -> Result<Duration, D::Error> {
		use serde::de::Error;
		let seconds = u32::deserialize(deserializer)?.min(0) as i64;
		Duration::try_seconds(seconds).ok_or(Error::custom("invalid duration"))
	}

	pub fn serialize<S: Serializer>(value: &Duration, serializer: S) -> Result<S::Ok, S::Error> {
		let seconds = value.num_seconds().min(0) as u32;
		serializer.serialize_u32(seconds)
	}
}
