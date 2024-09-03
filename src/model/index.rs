use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlb::{Fields, HasFields};
use sqlx::postgres::PgRow;
use sqlx::FromRow;
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Index {
	pub id: i64,
	pub datatype_name: String,
	pub project_id: i64,
	pub required: bool,
	pub index_name: String,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct IndexForCreate {
	pub datatype_name: String,
	pub project_id: i64,
	pub required: bool,
	pub index_name: String,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct IndexForUpdate {
	pub datatype_name: String,
	pub required: bool,
	pub index_name: String,
}

pub trait IndexBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl IndexBy for Index {}
impl IndexBy for IndexForCreate {}
impl IndexBy for IndexForUpdate {}

pub struct IndexBmc;

impl DbBmc for IndexBmc {
	const TABLE: &'static str = "index";
}

impl IndexBmc {
	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Index> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		index_c: IndexForCreate,
	) -> Result<i64> {
		let index_id = base::create::<Self, _>(ctx, mm, index_c).await?;

		Ok(index_id)
	}

	pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Index>> {
		base::list::<Self, _>(ctx, mm).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		index_u: IndexForUpdate,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, index_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
