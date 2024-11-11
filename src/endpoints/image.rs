use axum::{
	body::Bytes,
	extract::{Path, State},
	Json,
};
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use serde::Serialize;
use sqlx::{query, PgPool};
use uuid::Uuid;

use crate::{
	errors::{ApiError, TaskError},
	extractors::Authentication,
	id::Id,
	ApiState,
};

#[derive(Serialize)]
pub struct SharedImage {
	uploader: Uuid,
	filename: String,
	file: String,
	shared_at: DateTime<Utc>,
}

pub async fn get(
	State(ApiState { database, .. }): State<ApiState>,
	Path(id): Path<Id>,
) -> Result<Json<SharedImage>, ApiError> {
	let image = query!("SELECT player, filename, file, timestamp FROM images WHERE id = $1", id as _)
		.fetch_optional(&database)
		.await?
		.ok_or(StatusCode::NOT_FOUND)?;

	let filename = String::from_utf8(image.filename).unwrap();
	Ok(Json(SharedImage {
		uploader: image.player,
		filename,
		file: STANDARD_NO_PAD.encode(image.file),
		shared_at: image.timestamp.and_utc(),
	}))
}

pub async fn get_raw(
	State(ApiState { database, .. }): State<ApiState>,
	Path(id): Path<Id>,
) -> Result<Vec<u8>, ApiError> {
	let image = query!("SELECT player, filename, file, timestamp FROM images WHERE id = $1", id as _)
		.fetch_optional(&database)
		.await?
		.ok_or(StatusCode::NOT_FOUND)?;
	Ok(image.file)
}

pub async fn post(
	State(ApiState { database, .. }): State<ApiState>,
	Authentication(uuid): Authentication,
	Path(filename): Path<String>,
	body: Bytes,
) -> Result<String, ApiError> {
	let id = Id::new();
	query!(
		"INSERT INTO images (id, player, filename, file) VALUES ($1, $2, $3, $4)",
		&id as _,
		uuid,
		filename.as_bytes(),
		&body.to_vec()
	)
	.execute(&database)
	.await?;

	Ok(id.to_string())
}

pub async fn evict_expired(database: &PgPool) -> Result<(), TaskError> {
	query!("DELETE FROM images WHERE (LOCALTIMESTAMP - timestamp) > '1 week'")
		.execute(database)
		.await?;
	Ok(())
}
