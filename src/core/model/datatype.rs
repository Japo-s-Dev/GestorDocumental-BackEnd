use crate::core::ctx::Ctx;
use crate::core::model::base::{self, DbBmc};
use crate::core::model::ModelManager;
use crate::core::model::Result;
use modql::field::{Fields, HasFields};
use modql::filter::{FilterNodes, ListOptions, OpValsInt64};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::postgres::PgRow;
use sqlx::FromRow;

use super::base::ListResult;

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Datatype {
	pub id: i64,
	pub datatype_name: String,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct DatatypeForOp {
	pub datatype_name: String,
}

#[allow(dead_code)]
pub trait DatatypeBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl DatatypeBy for Datatype {}
impl DatatypeBy for DatatypeForOp {}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct DatatypeFilter {
	id: Option<OpValsInt64>,

	datatype_name: Option<OpValsInt64>,
}

pub struct DatatypeBmc;

impl DbBmc for DatatypeBmc {
	const TABLE: &'static str = "datatype";
	const TIMESTAMPED: bool = false;
	const SOFTDELETED: bool = false;
	const SCHEMA: Option<&'static str> = Some("consts");
}

impl DatatypeBmc {
	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Datatype> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		datatype_c: DatatypeForOp,
	) -> Result<i64> {
		let datatype_id = base::create::<Self, _>(ctx, mm, datatype_c).await?;

		Ok(datatype_id)
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filters: Option<Vec<DatatypeFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<ListResult<Datatype>> {
		base::list::<Self, _, _>(ctx, mm, filters, list_options).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		datatype_u: DatatypeForOp,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, datatype_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
