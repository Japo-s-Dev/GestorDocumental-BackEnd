use crate::{crypt, model, web};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
	// -- Rpc
	RpcMethodUnknown(String),
	RpcMisingParams { rpc_method: String },
	RpcFailJsonParams { rpc_method: String },
	InvalidParams(String),
	RpcInvalidMethod { rpc_method: String, message: String },
	// -- Document errors,
	RequestMissingFiles,

	// -- Login
	LoginFailUsernameNotFound,
	LoginFailUserHasNoPwd { user_id: i64 },
	LoginFialPwdNotMatching { user_id: i64 },
	LoginFailUserHasNoRole { user_id: i64 },

	// -- CtxExtError
	CtxExt(web::mw_auth::CtxExtError),

	// -- Modules
	Model(model::Error),
	Crypt(crypt::Error),

	// -- External modules
	SerdeJson(String),

	// -- MultiPart
	MultipartError,

	S3Error,
	SdkError(String),

	BadRequest(String),
}

impl From<model::Error> for Error {
	fn from(val: model::Error) -> Self {
		Self::Model(val)
	}
}

impl From<crypt::Error> for Error {
	fn from(val: crypt::Error) -> Self {
		Self::Crypt(val)
	}
}

impl From<serde_json::Error> for Error {
	fn from(val: serde_json::Error) -> Self {
		Self::SerdeJson(val.to_string())
	}
}

impl From<axum::extract::multipart::MultipartError> for Error {
	fn from(val: axum::extract::multipart::MultipartError) -> Self {
		Self::MultipartError
	}
}

impl From<aws_sdk_s3::Error> for Error {
	fn from(val: aws_sdk_s3::Error) -> Self {
		Self::S3Error
	}
}

impl<R, E> From<aws_smithy_runtime_api::client::result::SdkError<R, E>> for Error
where
	E: std::fmt::Debug,
	R: std::fmt::Debug,
{
	fn from(err: aws_smithy_runtime_api::client::result::SdkError<R, E>) -> Self {
		Error::SdkError(format!("{:?}", err))
	}
}

// region:    --- Axum IntoResponse
impl IntoResponse for Error {
	fn into_response(self) -> Response {
		debug!(" {:<12} - model::Error {self:?}", "INTO_RES");

		// Create a placeholder Axum reponse.
		let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

		// Insert the Error into the reponse.
		response.extensions_mut().insert(self);

		response
	}
}
// endregion: --- Axum IntoResponse

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

// region:    --- Client Error

/// From the root error to the http status code and ClientError
impl Error {
	pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
		use web::Error::*;

		#[allow(unreachable_patterns)]
		match self {
			// -- Login
			LoginFailUsernameNotFound
			| LoginFailUserHasNoPwd { .. }
			| LoginFialPwdNotMatching { .. } => {
				(StatusCode::FORBIDDEN, ClientError::LOGIN_FAIL)
			}

			// -- Auth
			CtxExt(_) => (StatusCode::FORBIDDEN, ClientError::NO_AUTH),

			// -- Model
			Model(model::Error::EntityNotFound { entity, id }) => (
				StatusCode::BAD_REQUEST,
				ClientError::ENTITY_NOT_FOUND { entity, id: *id },
			),

			// -- Fallback.
			_ => (
				StatusCode::INTERNAL_SERVER_ERROR,
				ClientError::SERVICE_ERROR,
			),
		}
	}
}

#[derive(Debug, Serialize, strum_macros::AsRefStr)]
#[serde(tag = "message", content = "detail")]
#[allow(non_camel_case_types)]
pub enum ClientError {
	LOGIN_FAIL,
	NO_AUTH,
	ENTITY_NOT_FOUND { entity: &'static str, id: i64 },
	SERVICE_ERROR,
}
// endregion: --- Client Error
