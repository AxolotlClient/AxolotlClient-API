use crate::endpoints::{
	brew_coffee, delete_account, delete_account_username, get_account, get_account_data, get_account_settings,
	get_authenticate, not_found, patch_account_settings, post_account_username, user,
};
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
		.route("/user/:uuid", get(user::get))
		// .route("/channel", post(post_channel))
		.route("/account", get(get_account).delete(delete_account))
		.route("/account/data", get(get_account_data))
		.route("/account/settings", get(get_account_settings).patch(patch_account_settings))
		.route("/account/:username", post(post_account_username).delete(delete_account_username))
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
