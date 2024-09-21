mod error;

use std::time::Duration;

pub use self::error::{Error, Result};

use crate::core::config::core_config;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
	let max_connections = if cfg!(test) { 1 } else { 5 };

	let db_url = core_config().DB_URL.clone();

	// let ssl_db_url = format!("{}?sslmode=require", db_url);

	PgPoolOptions::new()
		.max_connections(max_connections)
		.acquire_timeout(Duration::from_millis(10000))
		.connect(&db_url)
		.await
		.map_err(|ex| Error::FailedToCreatePool(ex.to_string()))
}
