use std::str::FromStr;

use crate::{errors::ApiError, extractors::Authentication, id::Id, ApiState};
use axum::{
	extract::{Path, State},
	Json,
};
use chrono::{Duration, TimeDelta};
use garde::Validate;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::query;
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
	persistence: Persistence,
	participants: Vec<Uuid>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
enum Persistence {
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

	fn from(id: i16, count: Option<u32>, duration: Option<Duration>) -> Option<Persistence> {
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

pub async fn get(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
	Path(channel_id): Path<Id>,
) -> Result<Json<Channel>, ApiError> {
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
	.fetch_optional(&database)
	.await?
	.ok_or(StatusCode::BAD_REQUEST)?;

	let participants: Vec<Uuid> = query!("SELECT * FROM channel_memberships WHERE $1 = ANY(channels)", channel_id as _)
		.fetch_all(&database)
		.await?
		.iter()
		.map(|rec| rec.player)
		.collect();

	if channel.owner == uuid || participants.contains(&uuid) {
		if let Some(persistence) = Persistence::from(
			channel.persistence,
			channel.persistence_count.map(|i| i as u32),
			channel.persistence_duration_seconds.map(TimeDelta::seconds),
		) {
			return Ok(Json(Channel {
				id: channel_id,
				channel_data: ChannelData {
					name: channel.name,
					persistence,
					participants,
				},
			}));
		}
	}

	Err(StatusCode::BAD_REQUEST)?
}

pub async fn post(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(owner): Authentication,
	Json(channel_data): Json<ChannelData>,
) -> Result<String, ApiError> {
	channel_data.validate(&())?;

	let id = Id::new();
	let persistence = channel_data.persistence.id();
	let persistence_count = channel_data.persistence.count();
	let persistence_duration_seconds = channel_data
		.persistence
		.duration()
		.map(|duration| duration.num_seconds());
	let participants = channel_data.participants;

	query!(
		r#"INSERT INTO channels(
			id,
			name,
			owner,
			persistence,
			persistence_count,
			persistence_duration_seconds
		) VALUES ($1, $2, $3, $4, $5, $6)"#,
		id as _,
		channel_data.name,
		owner,
		persistence as i8,
		persistence_count.map(|c| *c as i32),
		persistence_duration_seconds
	)
	.execute(&database)
	.await?;

	for uuid in participants {
		query!("UPDATE channel_memberships SET channels = ARRAY_APPEND(channels, $2) WHERE player = $1", uuid, id as _)
			.execute(&database)
			.await?;
	}

	Ok(id.to_string())
}

pub async fn patch(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
	Path(channel_id): Path<Id>,
	Json(value): Json<Value>,
) -> Result<StatusCode, ApiError> {
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
	.fetch_optional(&database)
	.await?
	.ok_or(StatusCode::BAD_REQUEST)?;

	if channel.owner == uuid {
		if let Some(mut persistence) = Persistence::from(
			channel.persistence,
			channel.persistence_count.map(|i| i as u32),
			channel.persistence_duration_seconds.map(TimeDelta::seconds),
		) {
			let mut name = channel.name;
			let mut participants: Vec<Uuid> =
				query!("SELECT * FROM channel_memberships WHERE $1 = ANY(channels)", channel_id as _)
					.fetch_all(&database)
					.await?
					.iter()
					.map(|rec| rec.player)
					.collect();
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

			let persistence_id = persistence.id() as i8;
			let persistence_count = persistence.count();
			let persistence_duration_seconds = persistence.duration().map(|duration| duration.num_seconds());
			query!(
				r#"UPDATE channels SET
					name = coalesce($1, name),
					persistence = coalesce($2, persistence),
					persistence_count = coalesce($3, persistence_count),
					persistence_duration_seconds = coalesce($4, persistence_duration_seconds)
					WHERE id = $5"#,
				name,
				persistence_id as _,
				persistence_count.map(|c| *c as i32),
				persistence_duration_seconds,
				channel_id as _
			)
			.execute(&database)
			.await?;
			return Ok(StatusCode::NO_CONTENT);
		}
	}

	Err(StatusCode::BAD_REQUEST)?
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
