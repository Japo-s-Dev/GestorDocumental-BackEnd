use serde::Serialize;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, Serialize)]
pub enum Error {
	FailedToCreateClient(String),
	AwsSdkConfig,
}

impl core::fmt::Display for Error {
	fn fmt(
		&self,
		fmt: &mut core::fmt::Formatter,
	) -> core::result::Result<(), core::fmt::Error> {
		write!(fmt, "{self:?}")
	}
}

impl From<aws_sdk_s3::Error> for Error {
	fn from(val: aws_sdk_s3::Error) -> Self {
		Self::AwsSdkConfig
	}
}

impl std::error::Error for Error {}
