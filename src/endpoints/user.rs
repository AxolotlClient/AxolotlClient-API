use crate::{errors::ApiError, ApiState};
use axum::{extract::Path, extract::State, Json};
use chrono::NaiveDateTime;
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

	#[serde(skip_serializing_if = "Option::is_none")]
	status: Option<Status>,

	#[serde(skip_serializing_if = "Vec::is_empty")]
	old_usernames: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Status {
	Offline { last_online: NaiveDateTime },
	Online,
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
				uuid AS 'uuid: Uuid',
				username,
				IIF(show_registered, registered, NULL) AS registered,
                last_online,
                show_status
			FROM users WHERE uuid = ?
		"#,
		uuid
	)
	.fetch_optional(&database)
	.await?
	.ok_or(StatusCode::NOT_FOUND)?;

	let old_usernames = query_scalar!("SELECT username FROM old_usernames WHERE user = ? AND public", uuid)
		.fetch_all(&database)
		.await?;

	let status = match user.show_status {
		true => match online_users.contains(&uuid) {
			true => Some(Status::Online),
			false => Some(Status::Offline {
				last_online: user.last_online,
			}),
		},
		false => None,
	};

	Ok(Json(User {
		uuid: user.uuid,
		username: user.username,
		registered: user.registered,
		status,
		old_usernames,
	}))
}
