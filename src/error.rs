use crate::model;
use axum::body::{boxed, BoxBody};
use axum::http::{Response, StatusCode};
use axum::response::IntoResponse;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
	// -- Config
	ConfigMissingEnv(&'static str),
	ConfigWrongFormat(&'static str),
	// -- Modules
	Model(model::Error),

	AxumError(axum::Error),
}

// region:    --- Froms
impl From<model::Error> for Error {
	fn from(val: model::Error) -> Self {
		Self::Model(val)
	}
}

impl From<axum::Error> for Error {
	fn from(err: axum::Error) -> Self {
		Self::AxumError(err)
	}
}
// endregion: --- Froms

// region:    --- Error Boilerplate
impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl std::error::Error for Error {}
// endregion: --- Error Boilerplate

impl IntoResponse for Error {
	fn into_response(self) -> Response<BoxBody> {
		match self {
			Error::AxumError(err) => {
				// Manually create a response from axum::Error
				let body = format!("Internal server error: {}", err);
				Response::builder()
					.status(StatusCode::INTERNAL_SERVER_ERROR)
					.header("Content-Type", "text/plain")
					.body(boxed(body))
					.unwrap()
			}
			Error::ConfigMissingEnv(msg) | Error::ConfigWrongFormat(msg) => {
				let body = format!("Configuration error: {}", msg);
				Response::builder()
					.status(StatusCode::INTERNAL_SERVER_ERROR)
					.header("Content-Type", "text/plain")
					.body(boxed(body))
					.unwrap()
			}
			Error::Model(err) => {
				let body = format!("Model error: {}", err);
				Response::builder()
					.status(StatusCode::INTERNAL_SERVER_ERROR)
					.header("Content-Type", "text/plain")
					.body(boxed(body))
					.unwrap()
			}
		}
	}
}
