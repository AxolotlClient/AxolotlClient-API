use axum::{http::StatusCode, response::IntoResponse, response::Response};
use garde::Report;
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

impl From<Report> for ApiError {
	fn from(value: Report) -> Self {
		ApiError((StatusCode::BAD_REQUEST, value.to_string()).into_response())
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

impl From<std::io::Error> for ApiError {
	fn from(error: std::io::Error) -> Self {
		Self::handle_internal_error(error)
	}
}

impl ApiError {
	fn handle_internal_error<E: Error>(error: E) -> ApiError {
		error!("Unhandled internal error: {error}");
		StatusCode::INTERNAL_SERVER_ERROR.into()
	}
}

pub struct TaskError;

impl TaskError {
	fn handle<E: Error>(error: E) -> TaskError {
		error!("Error while running taks: {error}");
		TaskError
	}
}

impl From<sqlx::Error> for TaskError {
	fn from(value: sqlx::Error) -> Self {
		Self::handle(value)
	}
}
