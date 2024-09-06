use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlb::{Fields, HasFields};
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Value {
	pub id: i64,
	pub index_id: i64,
	pub project_id: i64,
	pub archive_id: i64,
	pub creation_date: OffsetDateTime,
	pub modified_date: OffsetDateTime,
	pub last_edit_user: i64,
	pub value: String,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct ValueForCreate {
	pub index_id: i64,
	pub project_id: i64,
	pub archive_id: i64,
	pub value: String,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct ValueForUpdate {
	pub value: String,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct ValueForInsertCreate {
	pub index_id: i64,
	pub project_id: i64,
	pub archive_id: i64,
	pub value: String,
	pub creation_date: OffsetDateTime,
	pub modified_date: OffsetDateTime,
	pub last_edit_user: i64,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct ValueForInsertUpdate {
	pub value: String,
	pub modified_date: OffsetDateTime,
	pub last_edit_user: i64,
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
		let values = ValueForInsertCreate {
			index_id: value_c.index_id,
			project_id: value_c.project_id,
			archive_id: value_c.archive_id,
			value: value_c.value,
			creation_date: OffsetDateTime::now_utc(),
			modified_date: OffsetDateTime::now_utc(),
			last_edit_user: ctx.user_id(),
		};

		let value_id = base::create::<Self, _>(ctx, mm, values).await?;

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
		let values = ValueForInsertUpdate {
			value: value_u.value,
			modified_date: OffsetDateTime::now_utc(),
			last_edit_user: ctx.user_id(),
		};

		base::update::<Self, _>(ctx, mm, id, values).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
