use axum::{http::StatusCode, response::IntoResponse, response::Response, Json};
use log::error;
use serde::{Serialize, Serializer};
use std::{borrow::Cow, error::Error};

/// An error returned by the API. Generally used to indicate a client error, any server side errors should be logged and
/// the classic vague Http 500 "Internal Server Error" given to the client.
#[derive(Serialize)]
pub struct ApiError {
	#[serde(serialize_with = "ApiError::serialize_status_code")]
	pub status_code: StatusCode,
	pub error_code: u16,
	pub description: Cow<'static, str>,
}

impl IntoResponse for ApiError {
	fn into_response(self) -> Response {
		IntoResponse::into_response((self.status_code, Json(self)))
	}
}

impl ApiError {
	fn serialize_status_code<S: Serializer>(status_code: &StatusCode, serializer: S) -> Result<S::Ok, S::Error> {
		serializer.serialize_u16(status_code.as_u16())
	}

	fn handle_internal_error<E: Error>(error: E) -> ApiError {
		error!("Unhandled internal error: {error}");
		Self::internal_server_error()
	}

	pub fn authentication_failed() -> ApiError {
		ApiError {
			status_code: StatusCode::UNAUTHORIZED,
			error_code: 1000,
			description: Cow::from("Authentication failed"),
		}
	}

	pub fn authentication_missing() -> ApiError {
		ApiError {
			status_code: StatusCode::UNAUTHORIZED,
			error_code: 1001,
			description: Cow::from("Access Token not provided"),
		}
	}

	pub fn authentication_corrupt() -> ApiError {
		ApiError {
			status_code: StatusCode::UNAUTHORIZED,
			error_code: 1002,
			description: Cow::from("Access Token is corrupt"),
		}
	}

	pub fn authentication_invalid() -> ApiError {
		ApiError {
			status_code: StatusCode::UNAUTHORIZED,
			error_code: 1003,
			description: Cow::from("Access Token is expired or revoked"),
		}
	}

	pub fn not_found(path: &str) -> ApiError {
		ApiError {
			status_code: StatusCode::NOT_FOUND,
			error_code: 404,
			description: format!("\"{path}\" not found").into(),
		}
	}

	pub fn im_a_teapot() -> ApiError {
		ApiError {
			status_code: StatusCode::IM_A_TEAPOT,
			error_code: 418,
			description: Cow::from("I'm a teapot"),
		}
	}

	pub fn internal_server_error() -> ApiError {
		ApiError {
			status_code: StatusCode::INTERNAL_SERVER_ERROR,
			error_code: 500,
			description: Cow::from("Internal Server Error"),
		}
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
