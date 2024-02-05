use crate::endpoints::{
	authenticate, brew_coffee, delete_user, get_user, get_user_data, get_user_public, get_user_settings, not_found,
	patch_user_settings,
};
use axum::{routing::delete, routing::get, serve, Router};
use reqwest::Client;
use sqlx::{migrate, SqlitePool};
use std::env::var;

mod endpoints;
mod errors;
mod extractors;

#[derive(Clone)]
pub struct ApiState {
	pub database: SqlitePool,
	pub client: Client,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	env_logger::init();

	let database = {
		let database_url = var("SERVER_DATABASE_URL")?;
		SqlitePool::connect(&database_url).await?
	};

	migrate!().run(&database).await?;

	let router = Router::new()
		.route("/authenticate", get(authenticate))
		.route("/user/:uuid", get(get_user_public))
		.route("/account", delete(delete_user))
		.route("/account/user", get(get_user))
		.route("/account/settings", get(get_user_settings).patch(patch_user_settings))
		.route("/account/data", get(get_user_data))
		.route("/brew_coffee", get(brew_coffee).post(brew_coffee))
		.fallback(not_found)
		.with_state(ApiState {
			database,
			client: Client::new(),
		});

	let listener = tokio::net::TcpListener::bind("[::]:8000").await?;
	serve(listener, router).await?;
	Ok(())
}
