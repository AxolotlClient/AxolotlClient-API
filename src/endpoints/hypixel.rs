use crate::ClArgs;
use crate::{errors::ApiError, extractors::Authentication, ApiState};
use axum::response::IntoResponse;
use axum::{body::Body, extract::State, response::Response, Json};
use chrono::Utc;
use log::warn;
use mini_moka::sync::{Cache, CacheBuilder};
use reqwest::{Client, StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};
use std::fs::read_to_string;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use uuid::Uuid;

const HYPIXEL_API_URL: &str = "https://api.hypixel.net/v2";

pub struct HypixelApiProxyState {
	cache: Cache<Uuid, Value>,
	ratelimits: RwLock<Ratelimits>,
}

struct Ratelimits {
	limit: u64,
	remaining: u64,
	reset: u64,
}

impl Default for HypixelApiProxyState {
	fn default() -> Self {
		Self {
			cache: CacheBuilder::new(10_000)
				.time_to_live(Duration::from_secs(2 * 24 * 60 * 60))
				.build(),
			ratelimits: RwLock::new(Ratelimits {
				limit: 10,
				remaining: 10,
				reset: 60,
			}),
		}
	}
}

#[derive(Deserialize, PartialEq, Eq, Hash)]
pub struct RequestDataType {
	request_type: RequestType,
	target_player: Uuid,
}

#[derive(Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
enum RequestType {
	NetworkLevel,
	BedwarsLevel,
	SkywarsExperience,
	BedwarsData,
}

pub async fn get(
	State(ApiState {
		cl_args,
		hypixel_api_state,
		client,
		..
	}): State<ApiState>,
	Authentication(_): Authentication,
	Json(request_data_type): Json<RequestDataType>,
) -> Result<Json<Value>, ApiError> {
	let hypixel_api_cache = &hypixel_api_state.cache;
	let player_data = match hypixel_api_cache.get(&request_data_type.target_player) {
		Some(value) => value,
		None => {
			let value = fetch_data(cl_args.clone(), &hypixel_api_state.clone(), &client, &request_data_type).await?;
			hypixel_api_cache.insert(request_data_type.target_player, value.clone());
			value
		}
	};
	let val = match request_data_type.request_type {
		RequestType::NetworkLevel => {
			let mut exp = player_data["networkExp"].as_f64().unwrap_or(0f64);
			exp += leveling::get_total_exp_to_full_level(player_data["networkLevel"].as_f64().unwrap_or(0f64) + 1f64);
			let level = leveling::get_exact_level(exp);
			json!({
				"network_level": level
			})
		}
		RequestType::BedwarsLevel => {
			let level = player_data["achievements"]["bedwars_level"].as_i64().unwrap_or(-1);
			json!({
				"bedwars_level": level
			})
		}
		RequestType::SkywarsExperience => {
			let level = player_data["stats"]["SkyWars"]["skywars_experience"]
				.as_i64()
				.unwrap_or(-1);
			json!({
				"skywars_experience": level
			})
		}
		RequestType::BedwarsData => {
			if let Some(bedwars) = player_data["stats"]["Bedwars"].as_object() {
				json!({
				"final_kills_bedwars": bedwars.get("final_kills_bedwars").map(|v| v.as_u64().unwrap()).unwrap_or(0),
				"final_deaths_bedwars": bedwars.get("final_deaths_bedwars").map(|v| v.as_u64().unwrap()).unwrap_or(0),
				"beds_broken_bedwars": bedwars.get("beds_broken_bedwars").map(|v| v.as_u64().unwrap()).unwrap_or(0),
				"deaths_bedwars": bedwars.get("deaths_bedwars").map(|v| v.as_u64().unwrap()).unwrap_or(0),
				"kills_bedwars": bedwars.get("kills_bedwars").map(|v| v.as_u64().unwrap()).unwrap_or(0),
				"losses_bedwars": bedwars.get("losses_bedwars").map(|v| v.as_u64().unwrap()).unwrap_or(0),
				"wins_bedwars": bedwars.get("wins_bedwars").map(|v| v.as_u64().unwrap()).unwrap_or(0),
				"winstreak": bedwars.get("winstreak").map(|v| v.as_u64().unwrap()).unwrap_or(0)
				})
			} else {
				return Err(StatusCode::NOT_FOUND)?;
			}
		}
	};

	Ok(Json(val))
}

async fn fetch_data(
	cl_args: Arc<ClArgs>,
	hypixel_api_state: &Arc<HypixelApiProxyState>,
	client: &Client,
	request_data_type: &RequestDataType,
) -> Result<Value, Response> {
	let limits = &hypixel_api_state.ratelimits;
	let mut guard = limits.write().await;

	if let Some(value) = hypixel_api_state.cache.get(&request_data_type.target_player) {
		drop(guard);
		return Ok(value);
	}

	if guard.remaining <= 2 && Utc::now().timestamp() - guard.reset as i64 > 0 {
		guard.remaining = guard.limit;
	}
	if guard.remaining < 2 {
		let response: Response = Response::builder()
			.header("RateLimit-Reset", guard.reset)
			.status(StatusCode::TOO_MANY_REQUESTS)
			.body(Body::empty())
			.unwrap();
		drop(guard);
		return Err(response);
	}

	let api_key = match &cl_args.hypixel.hypixel_api_key {
		Some(api_key) => &api_key,
		None => match &cl_args.hypixel.hypixel_api_key_file {
			Some(file) => &read_to_string(file).map_err(|e| -> axum::http::Response<Body> {
				warn!("Failed to read hypixel API key file!");
				ApiError::from(e).into_response()
			})?,
			None => unreachable!("clap should ensure that a url or url file is provided"),
		},
	};

	let response = client
		.get(HYPIXEL_API_URL.to_string() + "/player")
		.header("API-Key", api_key)
		.query(&[("uuid", request_data_type.target_player.to_string())])
		.send()
		.await
		.map_err(|e| {
			warn!("Failed to request player data from hypixel!");
			ApiError::from(e).into_response()
		})?;
	let limit = response
		.headers()
		.get("RateLimit-Limit")
		.unwrap()
		.to_str()
		.unwrap()
		.parse::<u64>()
		.map_err(|_| {
			warn!("Failed to read 'RateLimit-Limit' header from hypixel's response!");
			ApiError::from(StatusCode::INTERNAL_SERVER_ERROR).into_response()
		})?;
	let remaining = response
		.headers()
		.get("RateLimit-Remaining")
		.unwrap()
		.to_str()
		.unwrap()
		.parse::<u64>()
		.map_err(|_| {
			warn!("Failed to read 'RateLimit-Remaining' header from hypixel's response!");
			ApiError::from(StatusCode::INTERNAL_SERVER_ERROR).into_response()
		})?;
	let reset = response
		.headers()
		.get("RateLimit-Reset")
		.unwrap()
		.to_str()
		.unwrap()
		.parse::<u64>()
		.map_err(|_| {
			warn!("Failed to read 'RateLimit-Reset' header from hypixel's response!");
			ApiError::from(StatusCode::INTERNAL_SERVER_ERROR).into_response()
		})?;

	guard.limit = limit;
	guard.remaining = remaining;
	guard.reset = Utc::now().timestamp() as u64 + reset;

	drop(guard);
	let data = response.json::<Value>().await.map_err(|e| {
		warn!("Failed to extract player data from hypixel's response");
		ApiError::from(e).into_response()
	})?["player"]
		.clone();
	Ok(data)
}

// Ported to rust from
// https://github.com/HypixelDev/PublicAPI/blob/68778c2235bb861667771887c57404c46ac23b50/hypixel-api-core/src/main/java/net/hypixel/api/util/ILeveling.java
// Original License: MIT
mod leveling {

	const BASE: f64 = 10_000f64;
	const GROWTH: f64 = 2_500f64;

	/* Constants to generate the total amount of XP to complete a level */
	const HALF_GROWTH: f64 = 0.5 * GROWTH;

	/* Constants to look up the level from the total amount of XP */
	const REVERSE_PQ_PREFIX: f64 = -(BASE - 0.5 * GROWTH) / GROWTH;
	const REVERSE_CONST: f64 = REVERSE_PQ_PREFIX * REVERSE_PQ_PREFIX;
	const GROWTH_DIVIDES_2: f64 = 2f64 / GROWTH;

	pub fn get_exact_level(exp: f64) -> f64 {
		get_level(exp) + get_percentage_to_next_level(exp)
	}

	pub fn get_level(exp: f64) -> f64 {
		f64::max(1f64, f64::floor(1f64 + REVERSE_PQ_PREFIX + f64::sqrt(REVERSE_CONST + GROWTH_DIVIDES_2 * (exp))))
	}

	fn get_percentage_to_next_level(exp: f64) -> f64 {
		let lv = get_level(exp);
		let x0 = get_total_exp_to_level(lv);
		(exp - x0) / (get_total_exp_to_level(lv + 1f64) - x0)
	}

	fn get_total_exp_to_level(level: f64) -> f64 {
		let lv = f64::floor(level);
		let x0 = get_total_exp_to_full_level(lv);
		if level == lv {
			x0
		} else {
			(get_total_exp_to_full_level(lv + 1f64) - x0) * (level % 1f64) + x0
		}
	}

	pub fn get_total_exp_to_full_level(level: f64) -> f64 {
		(HALF_GROWTH * (level - 2f64) + BASE) * (level - 1f64)
	}
}
