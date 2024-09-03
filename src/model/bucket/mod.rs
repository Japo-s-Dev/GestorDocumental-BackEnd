mod error;

use aws_config::{meta::region::RegionProviderChain, BehaviorVersion};
use aws_sdk_s3::Client;
use std::time::Duration;
use tokio::sync::OnceCell;
use tracing::debug;

pub use self::error::{Error, Result};

static S3_CLIENT: OnceCell<Client> = OnceCell::const_new();

pub type Bucket = Client;

async fn new_s3_client() -> Result<Client> {
	let config = aws_config::load_defaults(BehaviorVersion::v2024_03_28()).await;

	let client = Client::new(&config);

	debug!("S3 client created successfully");

	Ok(client)
}

pub async fn get_s3_client() -> Result<Bucket> {
	S3_CLIENT
		.get_or_try_init(new_s3_client)
		.await
		.map(|client| client.clone())
		.map_err(|_| {
			Error::FailedToCreateClient("Failed to retrieve S3 client".to_string())
		})
}
