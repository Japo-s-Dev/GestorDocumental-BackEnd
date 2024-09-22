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
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::postgres::PgRow;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

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

#[derive(Iden)]
enum RoleIden {
	RoleName,
}

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

	pub async fn first_by_role_name<E>(
		_ctx: &Ctx,
		mm: &ModelManager,
		role_name: &str,
	) -> Result<Option<E>>
	where
		E: RoleBy,
	{
		let db = mm.db();

		let mut query = Query::select();
		query
			.from(Self::table_ref())
			.columns(E::field_idens())
			.and_where(Expr::col(RoleIden::RoleName).eq(role_name));

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let role = sqlx::query_as_with::<_, E, _>(&sql, values)
			.fetch_optional(db)
			.await?;

		Ok(role)
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
	) -> Result<Vec<Role>> {
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
