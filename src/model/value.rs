use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlb::{Fields, HasFields};
use sqlx::postgres::PgRow;
use sqlx::FromRow;

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Value {
	pub id: i64,
	pub datatype_name: String,
	pub project_id: i64,
	pub required: bool,
	pub index_name: String,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct ValueForCreate {
	pub datatype_name: String,
	pub project_id: i64,
	pub required: bool,
	pub index_name: String,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct ValueForUpdate {
	pub datatype_name: String,
	pub required: bool,
	pub index_name: String,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct ValueForInsert {
	pub datatype_name: String,
	pub required: bool,
	pub index_name: String,
}

pub trait ValueBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl ValueBy for Value {}
impl ValueBy for ValueForCreate {}
impl ValueBy for ValueForUpdate {}

pub struct ValueBmc;

impl DbBmc for ValueBmc {
	const TABLE: &'static str = "value";
}

impl ValueBmc {
	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Value> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		value_c: ValueForCreate,
	) -> Result<i64> {
		let value_id = base::create::<Self, _>(ctx, mm, value_c).await?;

		Ok(value_id)
	}

	pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Value>> {
		base::list::<Self, _>(ctx, mm).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		value_u: ValueForUpdate,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, value_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}

