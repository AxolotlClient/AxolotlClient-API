use std::env::var;

use axum::{body::Body, extract::State, response::Response, Json};
use mini_moka::sync::{Cache, CacheBuilder};
use reqwest::{Client, RequestBuilder, StatusCode};
use serde::Deserialize;
use serde_json::{json, Value};
use std::time::Duration;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{errors::ApiError, extractors::Authentication, ApiState};

const HYPIXEL_API_URL: &str = "https://api.hypixel.net/v2";

fn get_api_key() -> Result<String, ApiError> {
	Ok(var("HYPIXEL_API_KEY").map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?)
}

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
			let limits = &hypixel_api_state.ratelimits;
			let mut guard = limits.try_write().map_err(|_| StatusCode::TOO_MANY_REQUESTS)?;

			if guard.remaining < 2 {
				let response: Response = Response::builder()
					.header("RateLimit-Reset", guard.reset)
					.status(StatusCode::TOO_MANY_REQUESTS)
					.body(Body::empty())
					.unwrap();
				return Err(response)?;
			}
			let response = get_request(client, "/player")?
				.query(&[("uuid", request_data_type.target_player.to_string())])
				.send()
				.await?;
			let limit = response
				.headers()
				.get("RateLimit-Limit")
				.unwrap()
				.to_str()
				.unwrap()
				.parse::<u64>()
				.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
			let remaining = response
				.headers()
				.get("RateLimit-Remaining")
				.unwrap()
				.to_str()
				.unwrap()
				.parse::<u64>()
				.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
			let reset = response
				.headers()
				.get("RateLimit-Reset")
				.unwrap()
				.to_str()
				.unwrap()
				.parse::<u64>()
				.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

			guard.limit = limit;
			guard.remaining = remaining;
			guard.reset = reset;

			drop(guard);
			response.json::<Value>().await?["player"].clone()
		}
	};
	hypixel_api_cache.insert(request_data_type.target_player, player_data.clone());
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
			let bedwars = player_data["stats"]["Bedwars"].as_object().unwrap();

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
		}
	};

	Ok(Json(val))
}

fn get_request(client: Client, route: &str) -> Result<RequestBuilder, ApiError> {
	Ok(client
		.get(HYPIXEL_API_URL.to_string() + route)
		.header("API-Key", get_api_key()?))
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
