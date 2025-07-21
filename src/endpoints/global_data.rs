use crate::{errors::ApiError, ApiState};
use axum::{
	extract::{FromRequestParts, State},
	http::{self, request::Parts},
	Json,
};
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use regex::Regex;
use reqwest::{Client, StatusCode};
use serde::Serialize;
use serde_json::Value;
use sqlx::{query, PgPool};
use std::{fmt::Write, fs::read_to_string, sync::LazyLock};

const PROJECT_ID: &str = "p2rxzX0q";

#[derive(Clone)]
pub struct GlobalDataContainer {
	last_full_refresh: DateTime<Utc>,
	last_player_refresh: DateTime<Utc>,
	pub data: GlobalData,
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
				request_user_agents: DashMap::new(),
				gateway_user_agents: DashMap::new(),
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
	#[serde(skip)]
	pub request_user_agents: DashMap<String, u32>,
	#[serde(skip)]
	pub gateway_user_agents: DashMap<String, u32>,
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
			request_user_agents: self.request_user_agents.clone(),
			gateway_user_agents: self.gateway_user_agents.clone(),
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
			request_user_agents: self.request_user_agents.clone(),
			gateway_user_agents: self.gateway_user_agents.clone(),
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
		let request_user_agents = data_container.data.request_user_agents.clone();
		let gateway_user_agents = data_container.data.gateway_user_agents.clone();
		GlobalData {
			total_players: get_total_players(&database).await?,
			online_players: online_users.len() as u32,
			modrinth_data: fetch_modrinth_data(client).await?,
			notes: (cl_args.notes_file.as_ref())
				.map(|file| read_to_string(file).unwrap_or_else(|_| String::new()))
				.unwrap_or_else(String::new),
			request_user_agents,
			gateway_user_agents,
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

pub async fn metrics(
	State(ApiState {
		database,
		online_users,
		global_data,
		..
	}): State<ApiState>,
) -> Result<String, ApiError> {
	let lifetime_players = get_total_players(&database).await?;
	let online_players = online_users.len();

	let mut response = String::new();

	#[rustfmt::skip]
	#[expect(unused)]
	{
		writeln!(response, "# This endpoint is intended for internal use with Prometheus. It is not part of the documented stable API and may be");
		writeln!(response, "# removed without notice. The `/v1/global_data` endpoint should be preferred, see the following:");
		writeln!(response, "# https://github.com/AxolotlClient/AxolotlClient-API/blob/main/docs/api_documentation.md#get-global_data");
		writeln!(response, "");
		writeln!(response, "lifetime_players {lifetime_players}");
		writeln!(response, "online_players {online_players}");
		let data_container = global_data.read().await;
		let request_agents = data_container.data.request_user_agents.clone();
		let request_agents_count = request_agents.len();
		writeln!(response, "request_agents_count {request_agents_count}");
		for (agent, count) in request_agents {
			if let Some((mod_ver, minecraft_ver, note)) = parse_user_agent(agent) {
				writeln!(response, "request_count{{mod_version=\"{mod_ver}\", minecraft_version=\"{minecraft_ver}\", mod=\"{note}\"}} {count}");
			}
		}
		data_container.data.request_user_agents.clear();
		let gateway_agents = data_container.data.gateway_user_agents.clone();
		for (agent, count) in gateway_agents {
			if let Some((mod_ver, minecraft_ver, note)) = parse_user_agent(agent) {
				writeln!(response, "connections{{mod_version=\"{mod_ver}\", minecraft_version=\"{minecraft_ver}\", mod=\"{note}\"}} {count}");
			}
		}
	};

	Ok(response)
}

static SEMVER_REGEX: LazyLock<Regex> = LazyLock::new(|| {
	Regex::new(
		r#"(?P<major>0|[1-9]\d*)\.(?P<minor>0|[1-9]\d*)\.(?P<patch>0|[1-9]\d*)(?:-(?P<prerelease>(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+(?P<buildmetadata>[0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?"#,
	)
	.unwrap()
});
static MCVER_REGEX: LazyLock<Regex> = LazyLock::new(|| {
	Regex::new(
		"0\\.\\d+(\\.\\d+)?a?(_\\d+)?|\\d+\\.\\d+(\\.\\d+)?(-pre\\d+|Pre-[Rr]elease \\d+)?|\\d+\\.\\d+(\\.\\d+)?(-rc\\d+| [Rr]elease Candidate \\d+)?|\\d+w\\d+[a-z]|[a-c]\\d\\.\\d+(\\.\\d+)?[a-z]?(_\\d+)?[a-z]?|(Alpha|Beta) v?\\d+\\.\\d+(\\.\\d+)?[a-z]?(_\\d+)?[a-z]?|Inf?dev (0\\.31 )?\\d+(-\\d+)?|(rd|inf?)-\\d+|1\\.RV-Pre1|3D Shareware v1\\.34|23w13a_or_b|24w14potato|25w14craftmine|(.*[Ee]xperimental [Ss]napshot )(\\d+)",
	)
	.unwrap()
});
static OLD_1_UA: LazyLock<Regex> =
	LazyLock::new(|| Regex::new(r".*\((?:(AxolotlClient)/)(.+)(?:\+mc)?(.+)\) .*").unwrap());
static OLD_2_UA: LazyLock<Regex> =
	LazyLock::new(|| Regex::new(r".*\((?:(AxolotlClient)/)(.+) \(Minecraft .+\).*").unwrap());
static CURRENT_UA: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(AxolotlClient)/(.+) Minecraft/(.+)").unwrap());
static SNAPPER_UA: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(Snapper)/(.+)\+(.+)").unwrap());

fn parse_user_agent(agent: String) -> Option<(String, String, String)> {
	if agent.starts_with("Java-http-client") {
		return None;
	}
	for regex in [&OLD_1_UA, &OLD_2_UA, &CURRENT_UA, &SNAPPER_UA] {
		if regex.is_match(&agent) {
			let captures = regex.captures(&agent).unwrap();
			let mod_name_capture = captures.get(1);
			let mod_ver_capture = captures.get(2);
			let mc_ver_capture = captures.get(3);
			if mod_name_capture.is_none() || mod_ver_capture.is_none() || mc_ver_capture.is_none() {
				return None;
			}
			let mod_name = mod_name_capture.unwrap().as_str().to_string();
			let mod_ver = mod_ver_capture.unwrap().as_str();
			let mc_ver = mc_ver_capture.unwrap().as_str();
			if !SEMVER_REGEX.is_match(mod_ver) || !MCVER_REGEX.is_match(mc_ver) {
				return None;
			}
			return Some((mod_ver.to_string(), mc_ver.to_string(), mod_name));
		}
	}
	return None;
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

pub struct RequestUserAgentCounter;

impl FromRequestParts<ApiState> for RequestUserAgentCounter {
	type Rejection = ApiError;
	async fn from_request_parts(
		parts: &mut Parts,
		state: &ApiState,
	) -> Result<RequestUserAgentCounter, Self::Rejection> {
		if parts.uri.path().ends_with("metrics") {
			return Ok(Self);
		}
		let agent = parts
			.headers
			.get(http::header::USER_AGENT)
			.map(|v| v.to_str())
			.ok_or(StatusCode::BAD_REQUEST)?
			.map_err(|_| StatusCode::BAD_REQUEST)?
			.replace("\\", "")
			.replace("\"", "");

		let container = state.global_data.read().await;
		let agents = &container.data.request_user_agents;
		if agents.contains_key(&agent) {
			let prev = agents.get(&agent).map(|v| *v.value()).unwrap_or(0);
			agents.insert(agent, prev + 1);
		} else {
			agents.insert(agent, 1);
		}
		Ok(Self)
	}
}
