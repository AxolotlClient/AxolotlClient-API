use crate::{errors::ApiError, ApiState};
use axum::{extract::State, Json};
use chrono::{DateTime, Utc};
use reqwest::{Client, StatusCode};
use serde::Serialize;
use serde_json::Value;
use sqlx::{query, PgPool};
use std::fs::read_to_string;

const PROJECT_ID: &str = "p2rxzX0q";

#[derive(Clone)]
pub struct GlobalDataContainer {
	last_full_refresh: DateTime<Utc>,
	last_player_refresh: DateTime<Utc>,
	data: GlobalData,
}

impl Default for GlobalDataContainer {
	fn default() -> Self {
		GlobalDataContainer {
			last_full_refresh: DateTime::from_timestamp_millis(0).unwrap(),
			last_player_refresh: DateTime::from_timestamp_millis(0).unwrap(),
			data: GlobalData {
				total_players: 0,
				online_players: 0,
				modrinth_data: ModrinthData {
					latest_version: String::new(),
				},
				notes: String::new(),
			},
		}
	}
}

#[derive(Serialize)]
pub struct GlobalData {
	total_players: u32,
	online_players: u32,
	#[serde(flatten)]
	modrinth_data: ModrinthData,
	#[serde(skip_serializing_if = "String::is_empty")]
	notes: String,
}

#[derive(Serialize)]
pub struct ModrinthData {
	latest_version: String,
}

impl GlobalData {
	fn with_players(&self, total: u32, online: u32) -> GlobalData {
		GlobalData {
			total_players: total,
			online_players: online,
			modrinth_data: self.modrinth_data.clone(),
			notes: self.notes.clone(),
		}
	}
}

impl Clone for GlobalData {
	fn clone(&self) -> Self {
		Self {
			total_players: self.total_players,
			online_players: self.online_players,
			modrinth_data: self.modrinth_data.clone(),
			notes: self.notes.clone(),
		}
	}
}

impl Clone for ModrinthData {
	fn clone(&self) -> Self {
		Self {
			latest_version: self.latest_version.clone(),
		}
	}
}

pub async fn get(
	State(ApiState {
		database,
		cl_args,
		online_users,
		client,
		global_data,
		..
	}): State<ApiState>,
) -> Result<Json<GlobalData>, ApiError> {
	let now = Utc::now();
	let data_container = global_data.read().await;
	let full_refresh = now.signed_duration_since(data_container.last_full_refresh).num_days() >= 1;
	if !full_refresh
		&& now
			.signed_duration_since(data_container.last_player_refresh)
			.num_minutes()
			< 2
	{
		let cloned = data_container.data.clone();
		drop(data_container);
		return Ok(Json(cloned));
	}
	let data = if full_refresh {
		GlobalData {
			total_players: get_total_players(&database).await?,
			online_players: online_users.len() as u32,
			modrinth_data: fetch_modrinth_data(client).await?,
			notes: (cl_args.notes_file.as_ref())
				.map(|file| read_to_string(file).unwrap_or_else(|_| String::new()))
				.unwrap_or_else(String::new),
		}
	} else {
		data_container
			.data
			.with_players(get_total_players(&database).await?, online_users.len() as u32)
	};
	drop(data_container);

	let mut container = global_data.write().await;
	if full_refresh {
		container.last_full_refresh = now;
	}
	container.last_player_refresh = now;
	container.data = data.clone();
	drop(container);

	Ok(Json(data))
}

async fn get_total_players(database: &PgPool) -> Result<u32, ApiError> {
	Ok(query!("SELECT reltuples AS estimate FROM pg_class where relname = 'players'")
		.fetch_one(database)
		.await?
		.estimate as u32)
}

async fn fetch_modrinth_data(client: Client) -> Result<ModrinthData, ApiError> {
	let response = client
		.get("https://api.modrinth.com/v2/project/".to_string() + PROJECT_ID + "/version")
		.send()
		.await?;

	let json: Value = response.json().await?;

	let latest = json
		.as_array()
		.map(|a| a.first().unwrap())
		.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
	Ok(ModrinthData {
		latest_version: latest
			.get("version_number")
			.map(|v| v.as_str().unwrap())
			.ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
			.split("+")
			.collect::<Vec<&str>>()
			.first()
			.unwrap()
			.to_string(),
	})
}
