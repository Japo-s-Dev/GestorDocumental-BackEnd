// region:    --- Modules

pub mod archive;
mod base;
pub mod bucket;
pub mod comment;
pub mod datatype;
pub mod document;
mod error;
pub mod event;
pub mod index;
pub mod modql_utils;
pub mod project;
pub mod role;
pub mod separator;
mod store;
pub mod user;
pub mod value;

use self::bucket::{get_s3_client, Bucket};
pub use self::error::{Error, Result};
use self::store::{new_db_pool, Db};

// endregion: --- Modules

#[derive(Clone)]
pub struct ModelManager {
	db: Db,
	pub bucket: Bucket,
}

impl ModelManager {
	pub async fn new() -> Result<Self> {
		let db = new_db_pool().await?;
		let bucket = get_s3_client().await?;
		// FIXME - TBC
		Ok(ModelManager { db, bucket })
	}
	//Regresa el pool de sqlx (Solo para la capa de Model)
	pub(in crate::core::model) fn db(&self) -> &Db {
		&self.db
	}
}
