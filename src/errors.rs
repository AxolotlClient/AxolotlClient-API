use axum::{http::StatusCode, response::IntoResponse, response::Response};
use log::error;
use std::error::Error;

pub struct ApiError(Response);

impl IntoResponse for ApiError {
	fn into_response(self) -> Response {
		self.0
	}
}

impl From<StatusCode> for ApiError {
	fn from(value: StatusCode) -> Self {
		ApiError(value.into_response())
	}
}

impl From<reqwest::Error> for ApiError {
	fn from(error: reqwest::Error) -> Self {
		Self::handle_internal_error(error)
	}
}

impl From<sqlx::Error> for ApiError {
	fn from(error: sqlx::Error) -> Self {
		Self::handle_internal_error(error)
	}
}

impl ApiError {
	fn handle_internal_error<E: Error>(error: E) -> ApiError {
		error!("Unhandled internal error: {error}");
		StatusCode::INTERNAL_SERVER_ERROR.into()
	}
}
