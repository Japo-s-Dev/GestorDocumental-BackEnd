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
pub struct Structure {
	pub id: i64,
	pub project_name: String,
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct StructureForOp {
	pub project_name: String,
}

#[allow(dead_code)]
pub trait StructureBy:
	HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send
{
}

impl StructureBy for Structure {}
impl StructureBy for StructureForOp {}

pub struct StructureBmc;

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct StructureFilter {
	id: Option<OpValsInt64>,
	project_name: Option<OpValsString>,
	cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	ctime: Option<OpValsValue>,
	mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	mtime: Option<OpValsValue>,
}

impl DbBmc for StructureBmc {
	const TABLE: &'static str = "structure";
}

impl StructureBmc {
	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Structure> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		structure_c: StructureForOp,
	) -> Result<i64> {
		let structure_id = base::create::<Self, _>(ctx, mm, structure_c).await?;

		Ok(structure_id)
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filters: Option<Vec<StructureFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<ListResult<Structure>> {
		base::list::<Self, _, _>(ctx, mm, filters, list_options).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		structure_u: StructureForOp,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, structure_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
