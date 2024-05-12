use crate::{errors::ApiError, extractors::Authentication, id::Id, ApiState};
use axum::{extract::State, Json};
use chrono::Duration;
use garde::Validate;
use serde::{Deserialize, Serialize};
use sqlx::query;

#[derive(Deserialize, Serialize)]
pub struct Channel {
	id: Id,
	#[serde(flatten)]
	channel_data: ChannelData,
}

#[derive(Deserialize, Serialize, Validate)]
pub struct ChannelData {
	#[garde(length(min = 1, max = 32))]
	name: String,
	#[garde(skip)]
	persistence: Persistence,
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
}

pub async fn post_channel(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(owner): Authentication,
	Json(channel_data): Json<ChannelData>,
) -> Result<String, ApiError> {
	channel_data.validate(&())?;

	let id = Id::new();
	let owner_ref: &[u8] = owner.as_ref();
	let persistence = channel_data.persistence.id();
	let persistence_count = channel_data.persistence.count();
	let persistence_duration_seconds = channel_data
		.persistence
		.duration()
		.map(|duration| duration.num_seconds());

	query!(
		r#"INSERT INTO channels (
			id,
			name,
			owner,
			persistence,
			persistence_count,
			persistence_duration_seconds
		) VALUES (?, ?, ?, ?, ?, ?)"#,
		id,
		channel_data.name,
		owner_ref,
		persistence,
		persistence_count,
		persistence_duration_seconds,
	)
	.execute(&database)
	.await?;

	Ok(id.to_string())
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
