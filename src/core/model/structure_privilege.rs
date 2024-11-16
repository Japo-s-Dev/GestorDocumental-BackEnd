use crate::core::ctx::Ctx;
use crate::core::model::base::{self, DbBmc};
use crate::core::model::ModelManager;
use crate::core::model::{Error, Result};
use modql::field::{Fields, HasFields};
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::postgres::PgRow;
use sqlx::FromRow;

use super::base::ListResult;
use super::idens::StructurePrivilegeIden;

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct StructurePrivilege {
	pub id: i64,
	pub user_id: i64,
	pub project_id: i64,
	pub is_enabled: bool,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct StructurePrivilegeForOp {
	pub user_id: i64,
	pub project_id: i64,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct StructurePrivilegeForSearchByUser {
	pub user_id: i64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StructuresForAct {
	pub user_id: i64,
	pub ids: Vec<i64>,
}

#[allow(dead_code)]
pub trait StructurePrivilegeBy:
	HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send
{
}

impl StructurePrivilegeBy for StructurePrivilege {}
impl StructurePrivilegeBy for StructurePrivilegeForOp {}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct StructurePrivilegeFilter {
	pub id: Option<OpValsInt64>,
	pub role_name: Option<OpValsString>,
	pub privilege_id: Option<OpValsInt64>,
}

pub struct StructurePrivilegeBmc;

impl DbBmc for StructurePrivilegeBmc {
	const TABLE: &'static str = "structure_privilege";
	const TIMESTAMPED: bool = true;
	const SOFTDELETED: bool = false;
}

impl StructurePrivilegeBmc {
	#[allow(unused)]
	pub async fn get(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
	) -> Result<StructurePrivilege> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn get_on_user_and_project_id(
		_ctx: &Ctx,
		mm: &ModelManager,
		user_id: i64,
		pid: i64,
	) -> Result<StructurePrivilege> {
		let db = mm.db();

		let mut query = Query::select();

		query
			.from(Self::table_ref())
			.columns(StructurePrivilege::field_idens())
			.and_where(Expr::col(StructurePrivilegeIden::UserId).eq(user_id))
			.and_where(Expr::col(StructurePrivilegeIden::ProjectId).eq(pid));

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let association =
			sqlx::query_as_with::<_, StructurePrivilege, _>(&sql, values)
				.fetch_one(db)
				.await?;

		Ok(association)
	}

	pub async fn list_by_user_id(
		_ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
	) -> Result<Vec<StructurePrivilege>> {
		let db = mm.db();

		let mut query = Query::select();
		query
			.from(Self::table_ref())
			.columns(StructurePrivilege::field_idens())
			.and_where(Expr::col(StructurePrivilegeIden::UserId).eq(id));

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let role = sqlx::query_as_with::<_, StructurePrivilege, _>(&sql, values)
			.fetch_all(db)
			.await?;

		Ok(role)
	}

	#[allow(unused)]
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		datatype_c: StructurePrivilegeForOp,
	) -> Result<i64> {
		let datatype_id = base::create::<Self, _>(ctx, mm, datatype_c).await?;

		Ok(datatype_id)
	}

	#[allow(unused)]
	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filters: Option<Vec<StructurePrivilegeFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<ListResult<StructurePrivilege>> {
		base::list::<Self, _, _>(ctx, mm, filters, list_options).await
	}

	#[allow(unused)]
	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		datatype_u: StructurePrivilegeForOp,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, datatype_u).await
	}

	#[allow(unused)]
	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}

	pub async fn enable(
		_ctx: &Ctx,
		mm: &ModelManager,
		user_id: i64,
		pid: i64,
	) -> Result<()> {
		let db = mm.db();

		let mut query = Query::update();
		query
			.table(Self::table_ref())
			.value(StructurePrivilegeIden::IsEnabled, true)
			.and_where(Expr::col(StructurePrivilegeIden::UserId).eq(user_id))
			.and_where(Expr::col(StructurePrivilegeIden::ProjectId).eq(pid));

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let count = sqlx::query_with(&sql, values)
			.execute(db)
			.await?
			.rows_affected();

		if count == 0 {
			Err(Error::EntityNotFound {
				entity: Self::TABLE,
				id: pid,
			})
		} else {
			Ok(())
		}
	}

	pub async fn disable(
		_ctx: &Ctx,
		mm: &ModelManager,
		user_id: i64,
		pid: i64,
	) -> Result<()> {
		let db = mm.db();

		let mut query = Query::update();
		query
			.table(Self::table_ref())
			.value(StructurePrivilegeIden::IsEnabled, false)
			.and_where(Expr::col(StructurePrivilegeIden::UserId).eq(user_id))
			.and_where(Expr::col(StructurePrivilegeIden::ProjectId).eq(pid));

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let count = sqlx::query_with(&sql, values)
			.execute(db)
			.await?
			.rows_affected();

		if count == 0 {
			Err(Error::EntityNotFound {
				entity: Self::TABLE,
				id: pid,
			})
		} else {
			Ok(())
		}
	}
}
