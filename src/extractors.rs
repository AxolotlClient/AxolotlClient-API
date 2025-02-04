use crate::{errors::ApiError, ApiState};
use axum::{
	extract::{FromRequestParts, OptionalFromRequestParts},
	http::{request::Parts, StatusCode},
};
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};
use sqlx::query;
use uuid::Uuid;

#[derive(Clone, Copy)]
pub struct Authentication(pub Uuid);

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
			.ok_or(StatusCode::UNAUTHORIZED)?
			.map_err(|_| StatusCode::UNAUTHORIZED)?;

		let mut transaction = database.begin().await?;
		let authorization_ref = &*authorization;
		let uuid = {
			let record = query!("SELECT valid, player FROM tokens WHERE token = $1", authorization_ref)
				.fetch_one(&mut *transaction)
				.await?;

			match record.valid {
				true => match record.player {
					Some(uuid) => Ok(uuid),
					_ => Err(StatusCode::UNAUTHORIZED),
				},
				false => Err(StatusCode::UNAUTHORIZED),
			}?
		};

		query!("UPDATE players SET last_online = LOCALTIMESTAMP where uuid = $1", uuid)
			.execute(&mut *transaction)
			.await?;

		query!("UPDATE tokens SET used = LOCALTIMESTAMP where token = $1", authorization_ref)
			.execute(&mut *transaction)
			.await?;

		transaction.commit().await?;

		Ok(Self(uuid))
	}
}

impl OptionalFromRequestParts<ApiState> for Authentication {
	type Rejection = ();

	async fn from_request_parts(parts: &mut Parts, state: &ApiState) -> Result<Option<Self>, Self::Rejection> {
		Ok(<Self as FromRequestParts<ApiState>>::from_request_parts(parts, state)
			.await
			.ok())
	}
}
