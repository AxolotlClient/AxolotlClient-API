use crate::{errors::ApiError, extractors::Authentication, ApiState};
use axum::{
	extract::{Path, Query, State},
	Json,
};
use chrono::{DateTime, Utc};
use log::warn;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_scalar, Type};
use uuid::Uuid;

#[derive(Serialize)]
pub struct User {
	uuid: Uuid,

	username: String,

	#[serde(skip_serializing_if = "Option::is_none")]
	relation: Option<Relation>,

	#[serde(skip_serializing_if = "Option::is_none")]
	registered: Option<DateTime<Utc>>,

	status: Status,

	#[serde(skip_serializing_if = "Vec::is_empty")]
	previous_usernames: Vec<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Status {
	Offline {
		#[serde(skip_serializing_if = "Option::is_none")]
		last_online: Option<DateTime<Utc>>,
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
	started: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "relation", rename_all = "lowercase")]
pub enum Relation {
	Blocked,
	None,
	Request,
	Friend,
}

pub async fn get(
	State(ApiState {
		database, online_users, ..
	}): State<ApiState>,
	authentication: Option<Authentication>,
	Path(other_uuid): Path<Uuid>,
) -> Result<Json<User>, ApiError> {
	let mut transaction = database.begin().await?;

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
		other_uuid
	)
	.fetch_optional(&mut *transaction)
	.await?
	.ok_or(StatusCode::NOT_FOUND)?;

	let relation = if let Some(Authentication(uuid)) = authentication {
		Some(
			query_scalar!(
				r#"SELECT relation as "relation: Relation" FROM relations WHERE player_a = $1 AND player_b = $2"#,
				uuid,
				other_uuid
			)
			.fetch_optional(&mut *transaction)
			.await?
			.unwrap_or(Relation::None),
		)
	} else {
		None
	};

	let status = match online_users.get(&other_uuid) {
		None => {
			let last_online = match user.show_last_online {
				true => user.last_online.map(|dt| dt.and_utc()),
				false => {
					if user.last_online.is_some() {
						// show_last_online is false, yet last_online is set? This shouldn't happen, but if it does, fix it
						warn!("players.last_online for {other_uuid} was NOT NULL, when it should be NULL");
						query!("UPDATE players SET last_online = NULL WHERE uuid = $1", other_uuid)
							.execute(&mut *transaction)
							.await?;
					}
					None
				}
			};

			Status::Offline { last_online }
		}
		Some(activity) => Status::Online {
			activity: activity.clone(),
		},
	};

	let previous_usernames =
		query_scalar!("SELECT username FROM previous_usernames WHERE player = $1 AND public", other_uuid)
			.fetch_all(&mut *transaction)
			.await?;

	transaction.commit().await?;

	Ok(Json(User {
		uuid: other_uuid,
		username: user.username,
		relation,
		registered: user.registered.map(|dt| dt.and_utc()),
		status,
		previous_usernames,
	}))
}

#[derive(Deserialize)]
pub struct PostRelation {
	relation: Relation,
}

#[derive(Serialize)]
pub struct FriendRequestNotification {
	target: String,
	from: Uuid,
}

pub async fn post(
	State(ApiState {
		database,
		online_users,
		socket_sender,
		..
	}): State<ApiState>,
	Authentication(uuid): Authentication,
	Path(other_uuid): Path<Uuid>,
	Query(PostRelation { relation }): Query<PostRelation>,
) -> Result<StatusCode, ApiError> {
	if uuid == other_uuid {
		Err(StatusCode::BAD_REQUEST)?
	}

	let mut transaction = database.begin().await?;

	match relation {
		Relation::Blocked => {
			query!(
				"INSERT INTO relations VALUES ($1, $2, 'blocked') ON CONFLICT ON CONSTRAINT relations_pkey DO UPDATE SET relation = 'blocked'",
				uuid,
				other_uuid
			)
			.execute(&mut *transaction)
			.await?;

			query!(
				"DELETE FROM relations WHERE player_a = $1 AND player_b = $2 AND relation > 'none'",
				other_uuid,
				uuid
			)
			.execute(&mut *transaction)
			.await?;
		}
		Relation::None => {
			query!("DELETE FROM relations WHERE player_a = $1 AND player_b = $2", uuid, other_uuid)
				.execute(&mut *transaction)
				.await?;

			query!(
				"DELETE FROM relations WHERE player_a = $1 AND player_b = $2 AND relation = 'friend'",
				other_uuid,
				uuid
			)
			.execute(&mut *transaction)
			.await?;
		}
		Relation::Request => {
			let other_relation = query_scalar!(
				r#"SELECT relation as "relation: Relation" FROM relations WHERE player_a = $1 AND player_b = $2"#,
				other_uuid,
				uuid
			)
			.fetch_optional(&mut *transaction)
			.await?
			.unwrap_or(Relation::None);

			match other_relation {
				Relation::Blocked => Err(StatusCode::FORBIDDEN)?,

				Relation::None => {
					// Notify $other_uuid that they have a new friend request (as there hasn't yet been a relation between the two)

					if online_users.contains_key(&other_uuid) {
						if let Some(sender) = socket_sender.get(&other_uuid) {
							sender
								.send(
									serde_json::to_string(&FriendRequestNotification {
										target: "friend_request".to_string(),
										from: uuid,
									})
									.unwrap(),
								)
								.unwrap();
						}
					}
				}

				// They already sent a request, as the intent is to friend the other player, let's accept the request, and pretend we sent one
				Relation::Request => {
					query!(
						"INSERT INTO relations VALUES ($1, $2, 'friend') ON CONFLICT ON CONSTRAINT relations_pkey DO UPDATE SET relation = 'friend'",
						uuid,
						other_uuid
					)
					.execute(&mut *transaction)
					.await?;

					query!(
						"UPDATE relations SET relation = 'friend' WHERE player_a = $1 AND player_b = $2",
						other_uuid,
						uuid
					)
					.execute(&mut *transaction)
					.await?;
				}

				// Already friended? As the intent is to friend the other player, let's pretend we sent the request
				Relation::Friend => return Ok(StatusCode::NO_CONTENT),
			}
		}
		Relation::Friend => {
			let other_relation = query_scalar!(
				r#"SELECT relation as "relation: Relation" FROM relations WHERE player_a = $1 AND player_b = $2"#,
				other_uuid,
				uuid
			)
			.fetch_optional(&mut *transaction)
			.await?
			.unwrap_or(Relation::None);

			match other_relation {
				Relation::Blocked | Relation::None => Err(StatusCode::FORBIDDEN)?,

				Relation::Request => {
					query!(
						"INSERT INTO relations VALUES ($1, $2, 'friend') ON CONFLICT ON CONSTRAINT relations_pkey DO UPDATE SET relation = 'friend'",
						uuid,
						other_uuid
					)
					.execute(&mut *transaction)
					.await?;

					query!(
						"UPDATE relations SET relation = 'friend' WHERE player_a = $1 AND player_b = $2",
						other_uuid,
						uuid
					)
					.execute(&mut *transaction)
					.await?;
				}

				// Already friended? Pretend we accepted
				Relation::Friend => return Ok(StatusCode::NO_CONTENT),
			}
		}
	}

	transaction.commit().await?;

	Ok(StatusCode::NO_CONTENT)
}
