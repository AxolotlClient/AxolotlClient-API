use crate::{errors::ApiError, ApiState};
use axum::{async_trait, extract::FromRequestParts, http::request::Parts, http::StatusCode};
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};
use serde::de::DeserializeOwned;
use sqlx::query;
use uuid::Uuid;

/// Basically a copy of Axum's Query except we handle errors our own way. Could have been a wrapper around Axum's Query
/// but its probably just simpler to not
#[derive(Clone, Copy)]
pub struct Query<T>(pub T);

#[async_trait]
impl<T: DeserializeOwned, S: Send + Sync> FromRequestParts<S> for Query<T> {
	type Rejection = ApiError;

	async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
		let query = parts.uri.query().unwrap_or_default();
		match serde_urlencoded::from_str(query) {
			Ok(value) => Ok(Self(value)),
			Err(error) => Err(ApiError {
				status_code: StatusCode::BAD_REQUEST,
				error_code: 400,
				description: error.to_string().into(),
			}),
		}
	}
}

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
			.ok_or(ApiError::authentication_missing())?
			.map_err(|_| ApiError::authentication_corrupt())?;

		let authorization_ref = &*authorization;
		let uuid = {
			let record = query!("SELECT valid, user AS 'uuid: Uuid' FROM tokens WHERE token = ?", authorization_ref)
				.fetch_one(database)
				.await?;

			match record.valid {
				true => match record.uuid {
					Some(uuid) => Ok(uuid),
					_ => Err(ApiError::authentication_invalid()),
				},
				false => Err(ApiError::authentication_invalid()),
			}?
		};

		let uuid_ref = uuid.as_ref();
		query!(
			"UPDATE users SET last_activity = CURRENT_TIMESTAMP where uuid = ?;\
			 UPDATE tokens SET used = CURRENT_TIMESTAMP where token = ?",
			uuid_ref,
			authorization_ref
		)
		.execute(database)
		.await?;

		Ok(Self(uuid))
	}
}
