use crate::{errors::ApiError, ApiState};
use axum::{async_trait, extract::FromRequestParts, http::request::Parts, http::StatusCode};
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};
use sqlx::query;
use uuid::Uuid;

#[derive(Clone, Copy)]
pub struct Authentication(pub Uuid);

#[async_trait]
impl FromRequestParts<ApiState> for Authentication {
	type Rejection = ApiError;

	async fn from_request_parts(
		parts: &mut Parts,
		ApiState { database, .. }: &ApiState,
	) -> Result<Self, Self::Rejection> {
		let authorization = parts
			.headers
			.get("Authorization")
			.map(|value| STANDARD_NO_PAD.decode(value))
			.ok_or(StatusCode::FORBIDDEN)?
			.map_err(|_| StatusCode::FORBIDDEN)?;

		let authorization_ref = &*authorization;
		let uuid = {
			let record = query!("SELECT valid, player FROM tokens WHERE token = $1", authorization_ref)
				.fetch_one(database)
				.await?;

			match record.valid {
				true => match record.player {
					Some(uuid) => Ok(uuid),
					_ => Err(StatusCode::UNAUTHORIZED),
				},
				false => Err(StatusCode::UNAUTHORIZED),
			}?
		};

		query!("UPDATE players SET last_online = 'now' where uuid = $1", uuid)
			.execute(database)
			.await?;

		query!("UPDATE tokens SET used = 'now' where token = $1", authorization_ref)
			.execute(database)
			.await?;

		Ok(Self(uuid))
	}
}
