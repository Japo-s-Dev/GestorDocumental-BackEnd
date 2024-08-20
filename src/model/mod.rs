// region:    --- Modules

pub mod assigned_role;
mod base;
mod bucket;
mod error;
pub mod role;
mod store;
pub mod user;

pub use self::error::{Error, Result};
use self::store::{new_db_pool, Db};

// endregion: --- Modules

#[derive(Clone)]
pub struct ModelManager {
	db: Db,
}

impl ModelManager {
	pub async fn new() -> Result<Self> {
		let db = new_db_pool().await?;
		// FIXME - TBC
		Ok(ModelManager { db })
	}
	//Regresa el pool de sqlx (Solo para la capa de Model)
	pub(in crate::model) fn db(&self) -> &Db {
		&self.db
	}
}
