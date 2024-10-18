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
pub struct Role {
	pub id: i64,
	pub role_name: String,
	pub description: String,
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct RoleForOp {
	pub role_name: String,
	pub description: String,
}

#[allow(dead_code)]
pub trait RoleBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl RoleBy for Role {}
impl RoleBy for RoleForOp {}

pub struct RoleBmc;

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct RoleFilter {
	id: Option<OpValsInt64>,

	role_name: Option<OpValsString>,
	cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	ctime: Option<OpValsValue>,
	mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	mtime: Option<OpValsValue>,
}

impl DbBmc for RoleBmc {
	const TABLE: &'static str = "role";
}

impl RoleBmc {
	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Role> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		role_c: RoleForOp,
	) -> Result<i64> {
		let role_id = base::create::<Self, _>(ctx, mm, role_c).await?;

		Ok(role_id)
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filters: Option<Vec<RoleFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<ListResult<Role>> {
		base::list::<Self, _, _>(ctx, mm, filters, list_options).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		role_u: RoleForOp,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, role_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
