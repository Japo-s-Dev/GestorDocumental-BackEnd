use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::Result;
use modql::field::{Fields, HasFields};
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Value {
	pub id: i64,
	pub index_id: i64,
	pub project_id: i64,
	pub archive_id: i64,
	pub creation_date: i64,
	pub modified_date: i64,
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
	pub creation_date: i64,
	pub modified_date: i64,
	pub last_edit_user: i64,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct ValueForInsertUpdate {
	pub value: String,
	pub modified_date: i64,
	pub last_edit_user: i64,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct ValueFilter {
	id: Option<OpValsInt64>,
	index_id: Option<OpValsInt64>,
	project_id: Option<OpValsInt64>,
	archive_id: Option<OpValsInt64>,
	creation_date: Option<OpValsInt64>,
	modified_date: Option<OpValsInt64>,
	last_edit_user: Option<OpValsInt64>,
	value: Option<OpValsString>,
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
			creation_date: OffsetDateTime::unix_timestamp(OffsetDateTime::now_utc()),
			modified_date: OffsetDateTime::unix_timestamp(OffsetDateTime::now_utc()),
			last_edit_user: ctx.user_id(),
		};

		let value_id = base::create::<Self, _>(ctx, mm, values).await?;

		Ok(value_id)
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filters: Option<Vec<ValueFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<Value>> {
		base::list::<Self, _, _>(ctx, mm, filters, list_options).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		value_u: ValueForUpdate,
	) -> Result<()> {
		let values = ValueForInsertUpdate {
			value: value_u.value,
			modified_date: OffsetDateTime::unix_timestamp(OffsetDateTime::now_utc()),
			last_edit_user: ctx.user_id(),
		};

		base::update::<Self, _>(ctx, mm, id, values).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
