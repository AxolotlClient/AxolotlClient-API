use crate::{ApiState, ClArgs, errors::ApiError, extractors::Authentication};
use axum::{Json, body::Body, extract::State, response::IntoResponse, response::Response};
use chrono::Utc;
use log::warn;
use mini_moka::sync::{Cache, CacheBuilder};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{
	collections::{HashMap, VecDeque},
	fs::read_to_string,
	sync::Arc,
	time::Duration,
};
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

impl HypixelApiProxyState {
	pub fn new(cache_limit_bytes: u64) -> Self {
		Self {
			cache: CacheBuilder::new(cache_limit_bytes)
				.weigher(|_, value| {
					let mut size = size_of::<Uuid>() as u32 + size_of::<Value>() as u32;
					let mut recursive_search = VecDeque::from([value]);
					while let Some(value) = recursive_search.pop_front() {
						match value {
							Value::String(string) => size += string.capacity() as u32,
							Value::Array(vec) => {
								size += (vec.capacity() * size_of::<Value>()) as u32;
								recursive_search.extend(vec);
							}
							Value::Object(map) => {
								size += map.keys().map(String::len).sum::<usize>() as u32;
								size += ((size_of::<String>() + size_of::<Value>()) * map.len()) as u32; // Capacity isn't available?
								recursive_search.extend(map.values());
							}
							_ => {}
						};
					}
					size
				})
				.time_to_live(Duration::from_secs(1 * 24 * 60 * 60))
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
	PlayerData,
}

#[derive(Serialize)]
struct PlayerData {
	name: String,
	bedwars: BedwarsData,
	skywars: SkywarsData,
	duels: DuelsData,
	rank: String,
	rank_formatted: String,
	level: f64,
	karma: i64,
}

impl PlayerData {
	fn of(player: Value) -> PlayerData {
		let bedwars = BedwarsData::of(&player);
		let skywars = SkywarsData::of(&player);
		let duels = DuelsData::of(&player);
		let rank: Rank = Self::get_rank(&player);
		let mut network_exp = player["networkExp"].as_f64().unwrap_or(0f64);
		network_exp += leveling::get_total_exp_to_full_level(player["networkLevel"].as_f64().unwrap_or(0f64) + 1f64);
		let network_level = leveling::get_exact_level(network_exp);
		PlayerData {
			name: player["displayname"].as_str().unwrap_or_default().to_owned(),
			bedwars,
			skywars,
			duels,
			rank: rank.clone().to_string(),
			rank_formatted: rank
				.to_string_formatted(player["rankPlusColor"].as_str(), player["monthlyPlusColor"].as_str()),
			level: network_level,
			karma: player["karma"].as_i64().unwrap_or_default(),
		}
	}

	fn get_rank(player: &Value) -> Rank {
		let keys = ["monthlyPackageRank", "newPackageRank", "packageRank"];
		let rank = player["rank"].as_str().unwrap_or("NORMAL");
		if rank == "NORMAL" {
			let mut highest: Rank = Rank::Normal;
			for current in keys {
				if let Some(r) = player[current].as_str().map(|s| Rank::of(s)) {
					if r > highest {
						highest = r;
					}
				}
			}
			return highest;
		} else {
			Rank::of(rank)
		}
	}
}

#[derive(PartialEq, PartialOrd, Clone)]
enum Rank {
	Normal = 1,
	Vip = 2,
	VipPlus = 3,
	Mvp = 4,
	MvpPlus = 5,
	Superstar = 6,
	Admin = 100,
	GameMaster = 95,
	Moderator = 90,
	Helper = 80,
	JrHelper = 70,
	Youtuber = 60,
}

impl Rank {
	fn of(name: &str) -> Rank {
		match name {
			"NONE" | "NORMAL" => Rank::Normal,
			"VIP" => Rank::Vip,
			"VIP_PLUS" => Rank::VipPlus,
			"MVP" => Rank::Mvp,
			"MVP_PLUS" => Rank::MvpPlus,
			"SUPERSTAR" => Rank::Superstar,
			"ADMIN" => Rank::Admin,
			"GAME_MASTER" => Rank::GameMaster,
			"MODERATOR" => Rank::Moderator,
			"HELPER" => Rank::Helper,
			"JR_HELPER" => Rank::JrHelper,
			"YOUTUBER" => Rank::Youtuber,
			&_ => Rank::Normal,
		}
	}

	fn to_string(&self) -> String {
		match self {
			Rank::Normal => "NORMAL",
			Rank::Vip => "VIP",
			Rank::VipPlus => "VIP_PLUS",
			Rank::Mvp => "MVP",
			Rank::MvpPlus => "MVP_PLUS",
			Rank::Superstar => "SUPERSTAR",
			Rank::Admin => "ADMIN",
			Rank::GameMaster => "GAME_MASTER",
			Rank::Moderator => "MODERATOR",
			Rank::Helper => "HELPER",
			Rank::JrHelper => "JR_HELPER",
			Rank::Youtuber => "YOUTUBER",
		}
		.to_owned()
	}

	fn to_string_formatted(&self, plus_color: Option<&str>, superstar_color: Option<&str>) -> String {
		match self {
			Rank::Normal => "§7".to_owned(),
			Rank::Vip => "§a[VIP]".to_owned(),
			Rank::VipPlus => "§a[VIP§6+§a]".to_owned(),
			Rank::Mvp => "§b[MVP]".to_owned(),
			Rank::MvpPlus => {
				let plus = colors::Code::of_option(plus_color).unwrap_or(colors::Code::Red);
				"§b[MVP".to_owned() + plus.get_code() + "+§b]"
			}
			Rank::Superstar => {
				let plus = plus_color
					.and_then(|s| colors::Code::of(s))
					.unwrap_or(colors::Code::Red);
				let color = superstar_color
					.and_then(|s| colors::Code::of(s))
					.unwrap_or(colors::Code::Gold);
				color.get_code().to_owned() + "[MVP" + plus.get_code() + "++" + color.get_code() + "]"
			}
			Rank::Admin => "§c[ADMIN]".to_owned(),
			Rank::GameMaster => "§2[GM]".to_owned(),
			Rank::Moderator => "§2[MOD]".to_owned(),
			Rank::Helper => "§9[HELPER]".to_owned(),
			Rank::JrHelper => "§9[JR HELPER]".to_owned(),
			Rank::Youtuber => "§c[§fYOUTUBE§c]".to_owned(),
		}
	}
}

#[derive(Serialize)]
struct BedwarsData {
	level: i64,
	all: BedwarsGameData,
	core: CombinedBedwarsGameData,
	solo: BedwarsGameData,
	doubles: BedwarsGameData,
	trios: BedwarsGameData,
	fours: BedwarsGameData,
	four_v_four: BedwarsGameData,
	dreams: CombinedBedwarsGameData,
	castle: BedwarsGameData,
	doubles_lucky: BedwarsGameData,
	fours_lucky: BedwarsGameData,
	doubles_ultimate: BedwarsGameData,
	fours_ultimate: BedwarsGameData,
	doubles_armed: BedwarsGameData,
	fours_armed: BedwarsGameData,
	doubles_rush: BedwarsGameData,
	fours_rush: BedwarsGameData,
	doubles_swap: BedwarsGameData,
	fours_swap: BedwarsGameData,
}

impl BedwarsData {
	fn of(val: &Value) -> BedwarsData {
		let bedwars_stats = &val["stats"]["Bedwars"];
		let solo = BedwarsGameData::of("eight_one_", &bedwars_stats);
		let doubles = BedwarsGameData::of("eight_two_", &bedwars_stats);
		let trios = BedwarsGameData::of("four_three_", &bedwars_stats);
		let fours = BedwarsGameData::of("four_four_", &bedwars_stats);
		let four_v_four = BedwarsGameData::of("two_four_", &bedwars_stats);
		let castle = BedwarsGameData::of("castle_", &bedwars_stats);
		let doubles_lucky = BedwarsGameData::of("eight_two_lucky_", &bedwars_stats);
		let fours_lucky = BedwarsGameData::of("four_four_lucky_", &bedwars_stats);
		let doubles_ultimate = BedwarsGameData::of("eight_two_ultimate_", &bedwars_stats);
		let fours_ultimate = BedwarsGameData::of("four_four_ultimate_", &bedwars_stats);
		let doubles_armed = BedwarsGameData::of("eight_two_armed_", &bedwars_stats);
		let fours_armed = BedwarsGameData::of("four_four_armed_", &bedwars_stats);
		let doubles_rush = BedwarsGameData::of("eight_two_rush_", &bedwars_stats);
		let fours_rush = BedwarsGameData::of("four_four_rush_", &bedwars_stats);
		let doubles_swap = BedwarsGameData::of("eight_two_swap_", &bedwars_stats);
		let fours_swap = BedwarsGameData::of("four_four_swap_", &bedwars_stats);
		BedwarsData {
			level: val["achievements"]["bedwars_level"].as_i64().unwrap_or(-1),
			all: BedwarsGameData::of("", &bedwars_stats),
			core: CombinedBedwarsGameData {
				kills: solo.kills + doubles.kills + trios.kills + fours.kills,
				deaths: solo.deaths + doubles.deaths + trios.deaths + fours.deaths,
				wins: solo.wins + doubles.wins + trios.wins + fours.wins,
				losses: solo.losses + doubles.losses + trios.losses + fours.losses,
				final_kills: solo.final_kills + doubles.final_kills + trios.final_kills + fours.final_kills,
				final_deaths: solo.final_deaths + doubles.final_deaths + trios.final_deaths + fours.final_deaths,
				beds_broken: solo.beds_broken + doubles.beds_broken + trios.beds_broken + fours.beds_broken,
				beds_lost: solo.beds_lost + doubles.beds_lost + trios.beds_lost + fours.beds_lost,
			},
			solo,
			doubles,
			trios,
			fours,
			four_v_four,
			dreams: CombinedBedwarsGameData {
				kills: castle.kills
					+ doubles_lucky.kills
					+ fours_lucky.kills
					+ doubles_ultimate.kills
					+ fours_ultimate.kills
					+ doubles_armed.kills
					+ fours_armed.kills
					+ doubles_rush.kills
					+ fours_rush.kills
					+ doubles_swap.kills
					+ fours_swap.kills,
				deaths: castle.deaths
					+ doubles_lucky.deaths
					+ fours_lucky.deaths
					+ doubles_ultimate.deaths
					+ fours_ultimate.deaths
					+ doubles_armed.deaths
					+ fours_armed.deaths
					+ doubles_rush.deaths
					+ fours_rush.deaths
					+ doubles_swap.deaths
					+ fours_swap.deaths,
				wins: castle.wins
					+ doubles_lucky.wins
					+ fours_lucky.wins
					+ doubles_ultimate.wins
					+ fours_ultimate.wins
					+ doubles_armed.wins
					+ fours_armed.wins
					+ doubles_rush.wins
					+ fours_rush.wins
					+ doubles_swap.wins
					+ fours_swap.wins,
				losses: castle.losses
					+ doubles_lucky.losses
					+ fours_lucky.losses
					+ doubles_ultimate.losses
					+ fours_ultimate.losses
					+ doubles_armed.losses
					+ fours_armed.losses
					+ doubles_rush.losses
					+ fours_rush.losses
					+ doubles_swap.losses
					+ fours_swap.losses,
				final_kills: castle.final_kills
					+ doubles_lucky.final_kills
					+ fours_lucky.final_kills
					+ doubles_ultimate.final_kills
					+ fours_ultimate.final_kills
					+ doubles_armed.final_kills
					+ fours_armed.final_kills
					+ doubles_rush.final_kills
					+ fours_rush.final_kills
					+ doubles_swap.final_kills
					+ fours_swap.final_kills,
				final_deaths: castle.final_deaths
					+ doubles_lucky.final_deaths
					+ fours_lucky.final_deaths
					+ doubles_ultimate.final_deaths
					+ fours_ultimate.final_deaths
					+ doubles_armed.final_deaths
					+ fours_armed.final_deaths
					+ doubles_rush.final_deaths
					+ fours_rush.final_deaths
					+ doubles_swap.final_deaths
					+ fours_swap.final_deaths,
				beds_broken: castle.beds_broken
					+ doubles_lucky.beds_broken
					+ fours_lucky.beds_broken
					+ doubles_ultimate.beds_broken
					+ fours_ultimate.beds_broken
					+ doubles_armed.beds_broken
					+ fours_armed.beds_broken
					+ doubles_rush.beds_broken
					+ fours_rush.beds_broken
					+ doubles_swap.beds_broken
					+ fours_swap.beds_broken,
				beds_lost: castle.beds_lost
					+ doubles_lucky.beds_lost
					+ fours_lucky.beds_lost
					+ doubles_ultimate.beds_lost
					+ fours_ultimate.beds_lost
					+ doubles_armed.beds_lost
					+ fours_armed.beds_lost
					+ doubles_rush.beds_lost
					+ fours_rush.beds_lost
					+ doubles_swap.beds_lost
					+ fours_swap.beds_lost,
			},
			castle,
			doubles_lucky,
			fours_lucky,
			doubles_ultimate,
			fours_ultimate,
			doubles_armed,
			fours_armed,
			doubles_rush,
			fours_rush,
			doubles_swap,
			fours_swap,
		}
	}
}

#[derive(Serialize)]
struct BedwarsGameData {
	kills: u64,
	deaths: u64,
	wins: u64,
	losses: u64,
	winstreak: u64,
	final_kills: u64,
	final_deaths: u64,
	beds_broken: u64,
	beds_lost: u64,
}

#[derive(Serialize)]
struct CombinedBedwarsGameData {
	kills: u64,
	deaths: u64,
	wins: u64,
	losses: u64,
	final_kills: u64,
	final_deaths: u64,
	beds_broken: u64,
	beds_lost: u64,
}

impl BedwarsGameData {
	fn of(prefix: &str, bedwars_stats: &Value) -> BedwarsGameData {
		BedwarsGameData {
			kills: bedwars_stats[prefix.to_owned() + "kills_bedwars"]
				.as_u64()
				.unwrap_or_default(),
			deaths: bedwars_stats[prefix.to_owned() + "deaths_bedwars"]
				.as_u64()
				.unwrap_or_default(),
			wins: bedwars_stats[prefix.to_owned() + "wins_bedwars"]
				.as_u64()
				.unwrap_or_default(),
			losses: bedwars_stats[prefix.to_owned() + "losses_bedwars"]
				.as_u64()
				.unwrap_or_default(),
			winstreak: bedwars_stats[prefix.to_owned() + "winstreak"]
				.as_u64()
				.unwrap_or_default(),
			final_kills: bedwars_stats[prefix.to_owned() + "final_kills_bedwars"]
				.as_u64()
				.unwrap_or_default(),
			final_deaths: bedwars_stats[prefix.to_owned() + "final_deaths_bedwars"]
				.as_u64()
				.unwrap_or_default(),
			beds_broken: bedwars_stats[prefix.to_owned() + "beds_broken_bedwars"]
				.as_u64()
				.unwrap_or_default(),
			beds_lost: bedwars_stats[prefix.to_owned() + "beds_lost_bedwars"]
				.as_u64()
				.unwrap_or_default(),
		}
	}
}

#[derive(Serialize)]
struct SkywarsData {
	level: String,
	exp: u64,
	all: SkywarsGameData,
	core: SkywarsGameData,
	solo: SkywarsModeData,
	team: SkywarsModeData,
	mega: SkywarsMegaModeData,
	ranked: SkywarsGameData,
	winstreak: u64,
}

impl SkywarsData {
	fn of(val: &Value) -> SkywarsData {
		let skywars_stats = &val["stats"]["SkyWars"];
		let solo = SkywarsModeData::of(skywars_stats, "_solo");
		let team = SkywarsModeData::of(skywars_stats, "_team");
		let mega = SkywarsMegaModeData::of(skywars_stats);
		let core = SkywarsGameData {
			kills: solo.normal.kills + solo.insane.kills + team.normal.kills + team.insane.kills,
			deaths: solo.normal.deaths + solo.insane.deaths + team.normal.deaths + team.insane.deaths,
			wins: solo.normal.wins + solo.insane.wins + team.normal.wins + team.insane.wins,
			losses: solo.normal.losses + solo.insane.losses + team.normal.losses + team.insane.losses,
		};
		let ranked = SkywarsGameData::of(skywars_stats, "_ranked_normal");
		SkywarsData {
			level: skywars_stats["levelFormatted"]
				.as_str()
				.map(|x| x.to_owned())
				.unwrap_or_default(),
			exp: skywars_stats["skywars_experience"].as_u64().unwrap_or_default(),
			all: SkywarsGameData {
				kills: core.kills + mega.normal.kills + mega.doubles.kills + ranked.kills,
				deaths: core.deaths + mega.normal.deaths + mega.doubles.deaths + ranked.deaths,
				wins: core.wins + mega.normal.wins + mega.doubles.wins + ranked.wins,
				losses: core.losses + mega.normal.losses + mega.doubles.losses + ranked.losses,
			},
			core,
			solo,
			team,
			mega,
			ranked,
			winstreak: skywars_stats["win_streak"].as_u64().unwrap_or_default(),
		}
	}
}

#[derive(Serialize)]
struct SkywarsModeData {
	normal: SkywarsGameData,
	insane: SkywarsGameData,
}

impl SkywarsModeData {
	fn of(skywars_stats: &Value, suffix: &str) -> SkywarsModeData {
		SkywarsModeData {
			normal: SkywarsGameData::of(skywars_stats, (suffix.to_owned() + "_normal").as_str()),
			insane: SkywarsGameData::of(skywars_stats, (suffix.to_owned() + "_insane").as_str()),
		}
	}
}

#[derive(Serialize)]
struct SkywarsMegaModeData {
	normal: SkywarsGameData,
	doubles: SkywarsGameData,
}

impl SkywarsMegaModeData {
	fn of(skywars_stats: &Value) -> SkywarsMegaModeData {
		SkywarsMegaModeData {
			normal: SkywarsGameData::of(skywars_stats, "_mega_normal"),
			doubles: SkywarsGameData::of(skywars_stats, "_mega_doubles"),
		}
	}
}

#[derive(Serialize)]
struct SkywarsGameData {
	kills: u64,
	deaths: u64,
	wins: u64,
	losses: u64,
}

impl SkywarsGameData {
	fn of(skywars_stats: &Value, suffix: &str) -> SkywarsGameData {
		SkywarsGameData {
			kills: skywars_stats["kills".to_owned() + suffix].as_u64().unwrap_or_default(),
			deaths: skywars_stats["deaths".to_owned() + suffix].as_u64().unwrap_or_default(),
			wins: skywars_stats["wins".to_owned() + suffix].as_u64().unwrap_or_default(),
			losses: skywars_stats["losses".to_owned() + suffix].as_u64().unwrap_or_default(),
		}
	}
}

#[derive(Serialize)]
struct DuelsData {
	modes: HashMap<String, DuelsGameData>,
}
impl DuelsData {
	fn of(player: &Value) -> DuelsData {
		let mut modes = HashMap::new();
		if let Some(obj) = player["stats"]["Duels"].as_object() {
			let mut mode_names = Vec::new();
			for name in obj.keys() {
				if name.starts_with("current_winstreak_mode_") {
					mode_names.push(name.strip_prefix("current_winstreak_mode_").unwrap());
				}
			}
			for name in mode_names {
				modes.insert(
					name.to_owned(),
					DuelsGameData {
						kills: obj
							.get(&(name.to_owned() + "_kills"))
							.map(|o| o.as_u64().unwrap_or_default())
							.unwrap_or(0),
						deaths: obj
							.get(&(name.to_owned() + "_deaths"))
							.map(|o| o.as_u64().unwrap_or_default())
							.unwrap_or(0),
						wins: obj
							.get(&(name.to_owned() + "_wins"))
							.map(|o| o.as_u64().unwrap_or_default())
							.unwrap_or(0),
						losses: obj
							.get(&(name.to_owned() + "_losses"))
							.map(|o| o.as_u64().unwrap_or_default())
							.unwrap_or(0),
						winstreak: obj
							.get(&("current_winstreak_mode_".to_owned() + name))
							.map(|o| o.as_u64().unwrap_or_default())
							.unwrap_or(0),
					},
				);
			}
		}
		DuelsData { modes }
	}
}

#[derive(Serialize)]
struct DuelsGameData {
	kills: u64,
	deaths: u64,
	wins: u64,
	losses: u64,
	winstreak: u64,
}

mod colors {

	pub enum Code {
		Black,
		DarkBlue,
		DarkGreen,
		DarkAqua,
		DarkRed,
		DarkPurple,
		Gold,
		Gray,
		DarkGray,
		Blue,
		Green,
		Aqua,
		Red,
		LightPurple,
		Yellow,
		White,

		Bold,
		Strikethrough,
		Underline,
		Italic,

		Reset,
		Magic,
	}

	impl Code {
		pub fn of_option(name: Option<&str>) -> Option<Code> {
			name.and_then(Code::of)
		}

		pub fn of(name: &str) -> Option<Code> {
			Some(match name {
				"BLACK" => Code::Black,
				"DARK_BLUE" => Code::DarkBlue,
				"DARK_GREEN" => Code::DarkGreen,
				"DARK_AQUA" => Code::DarkAqua,
				"DARK_RED" => Code::DarkRed,
				"DARK_PURPLE" => Code::DarkPurple,
				"GOLD" => Code::Gold,
				"GRAY" => Code::Gray,
				"DARK_GRAY" => Code::DarkGray,
				"BLUE" => Code::Blue,
				"GREEN" => Code::Green,
				"AQUA" => Code::Aqua,
				"RED" => Code::Red,
				"LIGHT_PURPLE" => Code::LightPurple,
				"YELLOW" => Code::Yellow,
				"WHITE" => Code::White,
				"MAGIC" => Code::Magic,
				"BOLD" => Code::Bold,
				"STRIKETHROUGH" => Code::Strikethrough,
				"UNDERLINE" => Code::Underline,
				"ITALIC" => Code::Italic,
				"RESET" => Code::Reset,
				&_ => return None,
			})
		}

		pub fn get_code(&self) -> &str {
			match self {
				Code::Black => "§0",
				Code::DarkBlue => "§1",
				Code::DarkGreen => "§2",
				Code::DarkAqua => "§3",
				Code::DarkRed => "§4",
				Code::DarkPurple => "§5",
				Code::Gold => "§6",
				Code::Gray => "§7",
				Code::DarkGray => "§8",
				Code::Blue => "§9",
				Code::Green => "§a",
				Code::Aqua => "§b",
				Code::Red => "§c",
				Code::LightPurple => "§d",
				Code::Yellow => "§e",
				Code::White => "§f",
				Code::Bold => "§l",
				Code::Strikethrough => "§m",
				Code::Underline => "§n",
				Code::Italic => "§o",
				Code::Reset => "§r",
				Code::Magic => "§k",
			}
		}
	}
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
				.as_f64()
				.unwrap_or(-1.0);
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
		RequestType::PlayerData => serde_json::to_value(PlayerData::of(player_data))?,
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
		Some(api_key) => api_key,
		None => match &cl_args.hypixel.hypixel_api_key_file {
			Some(file) => &read_to_string(file)
				.map(|s| s.trim().to_string())
				.map_err(|e| ApiError::from(e).into_response())?,
			None => unreachable!("clap should ensure that a url or url file is provided"),
		},
	};

	let response = client
		.get(HYPIXEL_API_URL.to_string() + "/player")
		.query(&[("uuid", request_data_type.target_player)])
		.header("api-key", api_key)
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
