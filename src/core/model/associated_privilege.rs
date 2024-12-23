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
use super::idens::{AssociatedPrivilegeIden, CommonIden};

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct AssociatedPrivilege {
	pub id: i64,
	pub role_name: String,
	pub privilege_id: i64,
	pub is_enabled: bool,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct AssociatedPrivilegeForOp {
	pub role_name: String,
	pub privilege_id: i64,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct AssociatedPrivilegeForSearchByRole {
	pub role_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AssociatedPrivilegesForOp {
	pub role_name: String,
	pub ids: Vec<i64>,
}

#[allow(dead_code)]
pub trait AssociatedPrivilegeBy:
	HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send
{
}

impl AssociatedPrivilegeBy for AssociatedPrivilege {}
impl AssociatedPrivilegeBy for AssociatedPrivilegeForOp {}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct AssociatedPrivilegeFilter {
	pub id: Option<OpValsInt64>,
	pub role_name: Option<OpValsString>,
	pub privilege_id: Option<OpValsInt64>,
}

pub struct AssociatedPrivilegeBmc;

impl DbBmc for AssociatedPrivilegeBmc {
	const TABLE: &'static str = "assosiated_privilege";
	const TIMESTAMPED: bool = true;
	const SOFTDELETED: bool = true;
}

impl AssociatedPrivilegeBmc {
	pub async fn get(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
	) -> Result<AssociatedPrivilege> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn get_on_role_and_id(
		_ctx: &Ctx,
		mm: &ModelManager,
		role_name: &str,
		pid: i64,
	) -> Result<AssociatedPrivilege> {
		let db = mm.db();

		let mut query = Query::select();

		query
			.from(Self::table_ref())
			.columns(AssociatedPrivilege::field_idens())
			.and_where(Expr::col(AssociatedPrivilegeIden::RoleName).eq(role_name))
			.and_where(Expr::col(AssociatedPrivilegeIden::PrivilegeId).eq(pid));

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let association =
			sqlx::query_as_with::<_, AssociatedPrivilege, _>(&sql, values)
				.fetch_one(db)
				.await?;

		Ok(association)
	}

	pub async fn list_by_role_name(
		_ctx: &Ctx,
		mm: &ModelManager,
		role_name: &str,
	) -> Result<Vec<AssociatedPrivilege>> {
		let db = mm.db();

		let mut query = Query::select();
		query
			.from(Self::table_ref())
			.columns(AssociatedPrivilege::field_idens())
			.and_where(Expr::col(AssociatedPrivilegeIden::RoleName).eq(role_name))
			.and_where(Expr::col(CommonIden::IsDeleted).eq(false));

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let association =
			sqlx::query_as_with::<_, AssociatedPrivilege, _>(&sql, values)
				.fetch_all(db)
				.await?;

		Ok(association)
	}

	pub async fn list_enabled_permissions(
		_ctx: &Ctx,
		mm: &ModelManager,
		role_name: &str,
	) -> Result<Vec<AssociatedPrivilege>> {
		let db = mm.db();

		let mut query = Query::select();
		query
			.from(Self::table_ref())
			.columns(AssociatedPrivilege::field_idens())
			.and_where(Expr::col(AssociatedPrivilegeIden::RoleName).eq(role_name))
			.and_where(Expr::col(AssociatedPrivilegeIden::IsEnabled).eq(true))
			.and_where(Expr::col(CommonIden::IsDeleted).eq(false));

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let association =
			sqlx::query_as_with::<_, AssociatedPrivilege, _>(&sql, values)
				.fetch_all(db)
				.await?;

		Ok(association)
	}

	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		datatype_c: AssociatedPrivilegeForOp,
	) -> Result<i64> {
		let datatype_id = base::create::<Self, _>(ctx, mm, datatype_c).await?;

		Ok(datatype_id)
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filters: Option<Vec<AssociatedPrivilegeFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<ListResult<AssociatedPrivilege>> {
		base::list::<Self, _, _>(ctx, mm, filters, list_options).await
	}

	#[allow(unused)]
	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		datatype_u: AssociatedPrivilegeForOp,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, datatype_u).await
	}

	pub async fn enable(
		_ctx: &Ctx,
		mm: &ModelManager,
		role_name: &str,
		pid: i64,
	) -> Result<()> {
		let db = mm.db();

		let mut query = Query::update();
		query
			.table(Self::table_ref())
			.value(AssociatedPrivilegeIden::IsEnabled, true)
			.and_where(Expr::col(AssociatedPrivilegeIden::RoleName).eq(role_name))
			.and_where(Expr::col(AssociatedPrivilegeIden::PrivilegeId).eq(pid));

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
		role_name: &str,
		pid: i64,
	) -> Result<()> {
		let db = mm.db();

		let mut query = Query::update();
		query
			.table(Self::table_ref())
			.value(AssociatedPrivilegeIden::IsEnabled, false)
			.and_where(Expr::col(AssociatedPrivilegeIden::RoleName).eq(role_name))
			.and_where(Expr::col(AssociatedPrivilegeIden::PrivilegeId).eq(pid));

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

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
