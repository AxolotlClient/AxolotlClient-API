use crate::endpoints::{account, brew_coffee, get_authenticate, not_found, user};
use crate::gateway::gateway;
use axum::{routing::get, routing::post, serve, Router};
use dashmap::DashMap;
use endpoints::user::Activity;
use reqwest::Client;
use sqlx::{migrate, PgPool};
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
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	env_logger::init();

	let database = {
		let database_url = var("SERVER_DATABASE_URL")?;
		PgPool::connect(&database_url).await?
	};

	migrate!().run(&database).await?;

	let router = Router::new()
		.route("/authenticate", get(get_authenticate))
		.route("/gateway", get(gateway))
		.route("/user/:uuid", get(user::get).post(user::post))
		// .route("/channel", post(post_channel))
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
		});

	let listener = tokio::net::TcpListener::bind("[::]:8000").await?;
	serve(listener, router).await?;
	Ok(())
}
