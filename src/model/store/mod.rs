mod error;

use std::time::Duration;

pub use self::error::{Error, Result};

use crate::config;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use tracing::debug;

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
	PgPoolOptions::new()
		.max_connections(20)
		.acquire_timeout(Duration::from_millis(10000))
		.connect(&config().DB_URL)
		.await
		.map_err(|ex| Error::FailedToCreatePool(ex.to_string()))
}
