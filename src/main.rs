use crate::endpoints::global_data::{self, GlobalDataContainer, RequestUserAgentCounter};
use crate::endpoints::user::{self, Activity};
use crate::endpoints::{account, brew_coffee, channel, get_authenticate, image, not_found};
use crate::gateway::gateway;
use axum::extract::DefaultBodyLimit;
use axum::routing::any;
use axum::{routing::get, routing::post, serve, Router};
use clap::{Args, Parser};
use dashmap::DashMap;
use endpoints::hypixel::{self, HypixelApiProxyState};
use env_logger::Env;
use log::info;
use reqwest::Client;
use sqlx::{migrate, postgres::PgConnectOptions, PgPool};
use std::time::{Duration, Instant};
use std::{fs::read_to_string, path::PathBuf, str::FromStr, sync::Arc};
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::RwLock;
use tokio::time::{interval, MissedTickBehavior};
use uuid::Uuid;

mod endpoints;
mod errors;
mod extractors;
mod gateway;
mod id;

#[derive(Parser)]
#[command(version)]
pub struct ClArgs {
	#[group(flatten)]
	pub postgres: PostgreSQL,

	#[group(flatten)]
	pub hypixel: Hypixel,

	#[arg(long)]
	pub notes_file: Option<PathBuf>,

	#[arg(long)]
	pub domain_name: Option<String>,

	#[arg(long, default_value = "1073741824")]
	pub cache_limit_bytes: u64,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
pub struct PostgreSQL {
	/// Postgres Connection Url, see: <https://docs.rs/sqlx/latest/sqlx/postgres/struct.PgConnectOptions.html>
	#[arg(long)]
	pub postgres_url: Option<PgConnectOptions>,

	/// File containing a Postgres Connection Url, see: <https://docs.rs/sqlx/latest/sqlx/postgres/struct.PgConnectOptions.html>
	#[arg(long)]
	pub postgres_url_file: Option<PathBuf>,
}

#[derive(Args)]
#[group(required = false, multiple = false)]
pub struct Hypixel {
	/// Hypixel API Key
	#[arg(long)]
	pub hypixel_api_key: Option<String>,

	/// File containing a Hypixel API Key
	#[arg(long)]
	pub hypixel_api_key_file: Option<PathBuf>,
}

#[derive(Clone)]
pub struct ApiState {
	pub database: PgPool,
	pub cl_args: Arc<ClArgs>,
	pub client: Client,
	pub online_users: Arc<DashMap<Uuid, Option<Activity>>>,
	pub socket_sender: Arc<DashMap<Uuid, UnboundedSender<String>>>,
	pub global_data: Arc<RwLock<GlobalDataContainer>>,
	pub hypixel_api_state: Arc<HypixelApiProxyState>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let start_time = Instant::now();

	let cl_args = Arc::new(ClArgs::parse());

	env_logger::init_from_env(Env::default().default_filter_or("info"));

	info!("AxolotlClient-Api v{} ({})", env!("CARGO_PKG_VERSION"), env!("GIT_HASH"));

	let database = {
		let postgres_url = match &cl_args.postgres.postgres_url {
			Some(postgres_url) => postgres_url.clone(),
			None => match &cl_args.postgres.postgres_url_file {
				Some(file) => PgConnectOptions::from_str(&read_to_string(file)?)?,
				None => unreachable!("clap should ensure that a url or url file is provided"),
			},
		};

		PgPool::connect_with(postgres_url.application_name("axolotl_client-api")).await?
	};

	migrate!().run(&database).await?;

	let state = ApiState {
		database,
		hypixel_api_state: Arc::new(HypixelApiProxyState::new(cl_args.cache_limit_bytes)),
		cl_args,
		client: Client::builder()
			.user_agent(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")))
			.build()?,
		online_users: Default::default(),
		socket_sender: Default::default(),
		global_data: Default::default(),
	};

	let task_state = state.clone();
	tokio::spawn(async move {
		let mut interval = interval(Duration::from_secs(1 * 24 * 60 * 60));
		interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
		let tasks = [image::evict_expired];
		loop {
			let _ = interval.tick().await;
			for task in tasks {
				let _ = task(&task_state).await;
			}
		}
	});

	let router = Router::new()
		.route("/global_data", get(global_data::get))
		.route("/metrics", get(global_data::metrics))
		.route("/authenticate", get(get_authenticate))
		.route("/gateway", any(gateway))
		.route("/user/{uuid}", get(user::get).post(user::post))
		.route("/user/{uuid}/images", get(user::get_images))
		.route("/channels", get(account::get_channels))
		.route("/channels/invites", get(account::get_channel_invites).post(account::post_channel_invite))
		.route("/channel", post(channel::post))
		.route(
			"/channel/{id}",
			get(channel::get)
				.post(channel::post_channel)
				.patch(channel::patch)
				.delete(channel::delete),
		)
		.route("/channel/{id}/messages", get(channel::get_messages))
		.route("/channel/{id}/remove", post(channel::remove_user))
		.route("/account", get(account::get).delete(account::delete))
		.route("/account/activity", post(account::post_activity))
		.route("/account/data", get(account::get_data))
		.route("/account/settings", get(account::get_settings).patch(account::patch_settings))
		.route("/account/username/{username}", post(account::post_username).delete(account::delete_username))
		.route("/account/relations/friends", get(account::get_friends))
		.route("/account/relations/blocked", get(account::get_blocked))
		.route("/account/relations/requests", get(account::get_requests))
		.route(
			"/image/{id}",
			get(image::get)
				.post(image::post)
				.layer(DefaultBodyLimit::max(1024 * 1024 * 8)),
		)
		.route("/image/{id}/raw", get(image::get_raw))
		.route("/image/{id}/view", get(image::get_view))
		.route("/image/{id}/oembed", get(image::get_oembed))
		.route("/hypixel", get(hypixel::get))
		//.route("/report/:message", post(channel::report_message))
		.route("/brew_coffee", get(brew_coffee).post(brew_coffee))
		.layer(axum::middleware::from_extractor_with_state::<RequestUserAgentCounter, ApiState>(state.clone()))
		.fallback(not_found)
		.with_state(state);

	let listener = tokio::net::TcpListener::bind("[::]:8000").await?;

	info!("Ready {:.0?}", Instant::now() - start_time);

	serve(listener, router).await?;
	Ok(())
}
