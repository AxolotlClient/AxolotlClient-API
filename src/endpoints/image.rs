use axum::{
	body::Bytes,
	extract::{Path, Query, State},
	response::Html,
	Json,
};
use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};
use bytes::Buf;
use chrono::{DateTime, Utc};
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
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
	let png = PngInfo::create(&body).await;
	if png.is_none() {
		return Err(StatusCode::BAD_REQUEST)?;
	}
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

pub async fn get_view(
	State(ApiState { database, .. }): State<ApiState>,
	Path(id): Path<Id>,
) -> Result<Html<String>, ApiError> {
	let image = query!("SELECT player, filename, file, timestamp FROM images WHERE id = $1", id as _)
		.fetch_optional(&database)
		.await?
		.ok_or(StatusCode::NOT_FOUND)?;

	let filename = String::from_utf8(image.filename).unwrap();
	let base_url = "https://api.axolotlclient.com/v1/";
	let image_url = base_url.to_string() + "image/" + &id.to_string() + "/";

	Ok(Html(format!(
		r#"
		<html>
		<head>
		<title>{filename}</title>
		<link rel="alternate" type="application/json+oembed"
  href="{}oembed?format=json"
  title="{filename}" />
		<style>
			.title {{
				text-align: center;
			}}
			img {{
				width: 100%;
				max-height: 85%;
			}}
		</style>
		</head>
		<body>
		<div class="title">
	<h2>{filename}</h2>
		</div>
		<img src="{}raw">
		</body>
		</html>
	"#,
		&image_url, &image_url
	)))
}

#[derive(Serialize)]
pub struct OEmbed {
	version: &'static str,
	#[serde(rename(serialize = "type"))]
	_type: &'static str,
	title: String,
	url: String,
	width: i32,
	height: i32,
	provider_name: &'static str,
	provider_url: &'static str,
}

impl OEmbed {
	fn create(title: String, url: String, png: PngInfo) -> OEmbed {
		OEmbed {
			version: "1.0",
			_type: "photo",
			title,
			url,
			width: png.width,
			height: png.height,
			provider_name: "AxolotlClient",
			provider_url: "https://axolotlclient.com",
		}
	}
}

#[derive(Deserialize)]
pub struct OEmbedQuery {
	format: String,
}

pub async fn get_oembed(
	State(ApiState { database, .. }): State<ApiState>,
	Path(id): Path<Id>,
	Query(OEmbedQuery { format }): Query<OEmbedQuery>,
) -> Result<Json<OEmbed>, ApiError> {
	let image = query!("SELECT filename, file FROM images WHERE id = $1", id as _)
		.fetch_optional(&database)
		.await?
		.ok_or(StatusCode::NOT_FOUND)?;
	let png = PngInfo::create(&Bytes::from(image.file)).await;

	if png.is_none() {
		return Err(StatusCode::BAD_REQUEST)?;
	}

	let filename = String::from_utf8(image.filename).unwrap();

	let embed = OEmbed::create(
		filename,
		"https://api.axolotlclient.com/v1/images/".to_owned() + &id.to_string() + "/raw",
		png.unwrap(),
	);
	Ok(if format == "json" {
		Json(embed)
	} else {
		return Err(StatusCode::NOT_IMPLEMENTED)?;
	})
}

struct PngInfo {
	width: i32,
	height: i32,
}

impl PngInfo {
	async fn create(reader: &Bytes) -> Option<PngInfo> {
		let mut bytes = reader.clone();
		let header = bytes.get_u64();
		if header != 0x89504E470D0A1A0A {
			return None;
		}
		let ihdr_size = bytes.get_u32();
		if ihdr_size != 0x0D {
			return None;
		}
		let ihdr_type = bytes.get_u32();
		if ihdr_type != 0x49484452 {
			return None;
		}
		Some(PngInfo {
			width: bytes.get_i32(),
			height: bytes.get_i32(),
		})
	}
}
