use crate::{errors::ApiError, ApiState};
use axum::{extract::Path, extract::State, Json};
use chrono::NaiveDateTime;
use log::warn;
use reqwest::StatusCode;
use serde::Serialize;
use sqlx::{query, query_scalar};
use uuid::Uuid;

#[derive(Serialize)]
pub struct User {
	uuid: Uuid,

	username: String,

	#[serde(skip_serializing_if = "Option::is_none")]
	registered: Option<NaiveDateTime>,

	status: Status,

	#[serde(skip_serializing_if = "Vec::is_empty")]
	previous_usernames: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Status {
	Offline {
		#[serde(skip_serializing_if = "Option::is_none")]
		last_online: Option<NaiveDateTime>,
	},
	Online {
		#[serde(skip_serializing_if = "Option::is_none")]
		activity: Option<Activity>,
	},
}

#[derive(Clone, Serialize)]
pub struct Activity {
	title: String,
	description: String,
	started: NaiveDateTime,
}

pub async fn get(
	State(ApiState {
		database, online_users, ..
	}): State<ApiState>,
	Path(uuid): Path<Uuid>,
) -> Result<Json<User>, ApiError> {
	let user = query!(
		r#"
			SELECT
				username,
				CASE WHEN show_registered THEN registered ELSE NULL END as registered,
				last_online,
				show_last_online,
				show_activity
			FROM players WHERE uuid = $1
		"#,
		uuid
	)
	.fetch_optional(&database)
	.await?
	.ok_or(StatusCode::NOT_FOUND)?;

	let status = match online_users.get(&uuid) {
		None => {
			let last_online = match user.show_last_online {
				true => user.last_online,
				false => match user.last_online {
					None => None,
					Some(_) => {
						warn!("players.last_online was NOT NULL, when it should be NULL");
						None
					}
				},
			};

			Status::Offline { last_online }
		}
		Some(activity) => Status::Online {
			activity: activity.clone(),
		},
	};

	let previous_usernames =
		query_scalar!("SELECT username FROM previous_usernames WHERE player = $1 AND public", uuid)
			.fetch_all(&database)
			.await?;

	Ok(Json(User {
		uuid,
		username: user.username,
		registered: user.registered,
		status,
		previous_usernames,
	}))
}
