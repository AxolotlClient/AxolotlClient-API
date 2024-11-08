use crate::endpoints::{account, brew_coffee, get_authenticate, not_found, user, channel};
use crate::gateway::gateway;
use axum::{routing::get, routing::post, serve, Router};
use dashmap::DashMap;
use dotenvy::dotenv;
use endpoints::user::Activity;
use env_logger::Env;
use log::info;
use reqwest::Client;
use sqlx::{migrate, PgPool};
use tokio::sync::mpsc::UnboundedSender;
use std::{env::var, sync::Arc};
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
	pub socket_sender: Arc<DashMap<Uuid, UnboundedSender<String>>>
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

	let router = Router::new()
		.route("/authenticate", get(get_authenticate))
		.route("/gateway", get(gateway))
		.route("/user/:uuid", get(user::get).post(user::post))
		.route("/channel", post(channel::post))
		.route("/channel/:id", get(channel::get).patch(channel::patch))
		.route("/account", get(account::get).delete(account::delete))
		.route("/account/data", get(account::get_data))
		.route("/account/settings", get(account::get_settings).patch(account::patch_settings))
		.route("/account/:username", post(account::post_username).delete(account::delete_username))
		.route("/brew_coffee", get(brew_coffee).post(brew_coffee))
		.fallback(not_found)
		.with_state(ApiState {
			database,
			client: Client::new(),
			online_users: Default::default(),
			socket_sender: Default::default()
		});

	let listener = tokio::net::TcpListener::bind("[::]:8000").await?;
	serve(listener, router).await?;
	Ok(())
}
