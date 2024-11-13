use crate::core::ctx::Ctx;
use crate::core::model::base::{self, DbBmc};
use crate::core::model::ModelManager;
use crate::core::model::Result;
use modql::field::{Fields, HasFields};
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::FromRow;

use super::base::ListResult;

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Privilege {
	pub id: i64,
	pub privilege_name: String,
	pub description: String,
}

#[allow(dead_code)]
pub trait PrivilegeBy:
	HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send
{
}

impl PrivilegeBy for Privilege {}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct PrivilegeFilter {
	id: Option<OpValsInt64>,

	privilege_name: Option<OpValsString>,
	description: Option<OpValsString>,
}

pub struct PrivilegeBmc;

impl DbBmc for PrivilegeBmc {
	const TABLE: &'static str = "privilege";
	const TIMESTAMPED: bool = false;
	const SOFTDELETED: bool = false;
	const SCHEMA: Option<&'static str> = Some("consts");
}

impl PrivilegeBmc {
	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Privilege> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filters: Option<Vec<PrivilegeFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<ListResult<Privilege>> {
		base::list::<Self, _, _>(ctx, mm, filters, list_options).await
	}
}
