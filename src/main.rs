use crate::endpoints::{
	account, brew_coffee, channel, get_authenticate,
	global_data::{self, GlobalDataContainer},
	image, not_found,
	user::{self, Activity},
};
use crate::gateway::gateway;
use axum::{routing::get, routing::post, serve, Router};
use dashmap::DashMap;
use env_logger::Env;
use log::info;
use reqwest::Client;
use sqlx::{migrate, PgPool};
use std::borrow::Cow;
use std::time::Duration;
use std::{env::var, sync::Arc};
use tokio::sync::mpsc::UnboundedSender;
use tokio::time::{interval, MissedTickBehavior};
use uuid::Uuid;

mod endpoints;
mod errors;
mod extractors;
mod gateway;
mod id;

#[derive(Clone)]
pub struct ApiState {
	pub database: PgPool,
	pub client: Client,
	pub online_users: Arc<DashMap<Uuid, Option<Activity>>>,
	pub socket_sender: Arc<DashMap<Uuid, UnboundedSender<String>>>,
	pub global_data: Cow<'static, GlobalDataContainer>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	env_logger::init_from_env(Env::default().default_filter_or("info"));

	info!("Starting AxolotlClient-Api... ({})", env!("CARGO_PKG_VERSION"));

	let database = {
		let database_url = match var("DATABASE_URL") {
			Ok(url) => url,
			Err(e) => panic!("Failed to read database url: {}", e),
		};
		PgPool::connect(&database_url).await?
	};

	migrate!().run(&database).await?;

	let db = database.clone();
	tokio::spawn(async move {
		let mut interval = interval(Duration::from_secs(1 * 24 * 60));
		interval.set_missed_tick_behavior(MissedTickBehavior::Skip);
		let tasks = [image::evict_expired];
		loop {
			let _ = interval.tick().await;
			for task in tasks {
				let _ = task(&db).await;
			}
		}
	});

	let router = Router::new()
		.route("/global_data", get(global_data::get))
		.route("/authenticate", get(get_authenticate))
		.route("/gateway", get(gateway))
		.route("/user/:uuid", get(user::get).post(user::post))
		.route("/channels", get(account::get_channels))
		.route("/channel", post(channel::post))
		.route("/channel/:id", get(channel::get).post(channel::post_channel).patch(channel::patch))
		.route("/channel/:id/messages", get(channel::get_messages))
		.route("/account", get(account::get).delete(account::delete))
		.route("/account/activity", post(account::post_activity))
		.route("/account/data", get(account::get_data))
		.route("/account/settings", get(account::get_settings).patch(account::patch_settings))
		.route("/account/username/:username", post(account::post_username).delete(account::delete_username))
		.route("/account/relations/friends", get(account::get_friends))
		.route("/account/relations/blocked", get(account::get_blocked))
		.route("/account/relations/requests", get(account::get_requests))
		.route("/image/:id", get(image::get).post(image::post))
		.route("/image/:id/raw", get(image::get_raw))
		.route("/brew_coffee", get(brew_coffee).post(brew_coffee))
		.fallback(not_found)
		.with_state(ApiState {
			database,
			client: Client::builder()
				.user_agent(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")))
				.build()?,
			online_users: Default::default(),
			socket_sender: Default::default(),
			global_data: Default::default(),
		});

	let listener = tokio::net::TcpListener::bind("[::]:8000").await?;
	serve(listener, router).await?;
	Ok(())
}
