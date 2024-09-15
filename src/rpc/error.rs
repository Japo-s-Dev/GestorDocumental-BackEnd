use crate::core::model;
use derive_more::From;
use serde::Serialize;
use serde_with::{serde_as, DisplayFromStr};

pub type Result<T> = core::result::Result<T, Error>;

#[serde_as]
#[derive(Debug, Serialize, From)]
pub enum Error {
	// -- Rpc
	RpcMethodUnknown(String),
	RpcMissingParams {
		rpc_method: String,
	},
	RpcFailJsonParams {
		rpc_method: String,
	},
	InvalidParams(String),
	RpcInvalidMethod {
		rpc_method: String,
		message: String,
	},
	// -- Document errors,
	RequestMissingFiles,

	InvalidFile,
	FileMissing,
	// -- Modules
	#[from]
	Model(model::Error),

	S3Error,
	SdkError(String),

	// -- External Modules
	#[from]
	SerdeJson(#[serde_as(as = "DisplayFromStr")] serde_json::Error),
}

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
