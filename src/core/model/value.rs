use crate::core::ctx::Ctx;
use crate::core::model::base::{self, DbBmc};
use crate::core::model::modql_utils::time_to_sea_value;
use crate::core::model::ModelManager;
use crate::core::model::Result;
use crate::utils::time::Rfc3339;
use modql::field::{Fields, HasFields};
use modql::filter::{
	FilterNodes, ListOptions, OpValsInt64, OpValsString, OpValsValue,
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::postgres::PgRow;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

use super::base::ListResult;

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Value {
	pub id: i64,
	pub index_id: i64,
	pub project_id: i64,
	pub archive_id: i64,
	pub last_edit_user: i64,
	pub value: String,
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
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

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct ValueForInsertCreate {
	pub index_id: i64,
	pub project_id: i64,
	pub archive_id: i64,
	pub value: String,
	pub last_edit_user: i64,
}

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct ValueForInsertUpdate {
	pub value: String,
	pub last_edit_user: i64,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct ValueFilter {
	id: Option<OpValsInt64>,
	index_id: Option<OpValsInt64>,
	project_id: Option<OpValsInt64>,
	archive_id: Option<OpValsInt64>,
	last_edit_user: Option<OpValsInt64>,
	value: Option<OpValsString>,
	cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	ctime: Option<OpValsValue>,
	mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	mtime: Option<OpValsValue>,
}

#[allow(dead_code)]
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
	) -> Result<ListResult<Value>> {
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
			last_edit_user: ctx.user_id(),
		};

		base::update::<Self, _>(ctx, mm, id, values).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
