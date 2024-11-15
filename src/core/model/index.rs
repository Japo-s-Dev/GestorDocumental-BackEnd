use crate::core::ctx::Ctx;
use crate::core::model::base::{self, DbBmc};
use crate::core::model::modql_utils::time_to_sea_value;
use crate::core::model::ModelManager;
use crate::core::model::Result;
use crate::utils::time::Rfc3339;
use modql::field::{Fields, HasFields};
use modql::filter::{
	FilterNodes, ListOptions, OpValsBool, OpValsInt64, OpValsString, OpValsValue,
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::postgres::PgRow;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

use super::base::ListResult;

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Index {
	pub id: i64,
	pub datatype_id: i64,
	pub project_id: i64,
	pub required: bool,
	pub index_name: String,
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct IndexForCreate {
	pub datatype_id: i64,
	pub project_id: i64,
	pub required: bool,
	pub index_name: String,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct IndexForUpdate {
	pub datatype_id: i64,
	pub required: bool,
	pub index_name: String,
}

#[allow(dead_code)]
pub trait IndexBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl IndexBy for Index {}
impl IndexBy for IndexForCreate {}
impl IndexBy for IndexForUpdate {}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct IndexFilter {
	id: Option<OpValsInt64>,
	datatype_id: Option<OpValsInt64>,
	project_id: Option<OpValsInt64>,
	required: Option<OpValsBool>,
	index_name: Option<OpValsString>,
	cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	ctime: Option<OpValsValue>,
	mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	mtime: Option<OpValsValue>,
}

pub struct IndexBmc;

impl DbBmc for IndexBmc {
	const TABLE: &'static str = "index";
	const TIMESTAMPED: bool = false;
	const SOFTDELETED: bool = true;
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

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filters: Option<Vec<IndexFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<ListResult<Index>> {
		base::list::<Self, _, _>(ctx, mm, filters, list_options).await
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
