use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlb::{Fields, HasFields};
use sqlx::postgres::PgRow;
use sqlx::FromRow;
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Role {
	pub id: i64,
	pub role_name: String,
	pub description: String,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct RoleForOp {
	pub role_name: String,
	pub description: String,
}

pub trait RoleBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl RoleBy for Role {}
//impl RoleBy for RoleForInsert {}

pub struct RoleBmc;

impl DbBmc for RoleBmc {
	const TABLE: &'static str = "role";
}

impl RoleBmc {
	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Role> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn first_by_username<E>(
		_ctx: &Ctx,
		mm: &ModelManager,
		role_name: &str,
	) -> Result<Option<E>>
	where
		E: RoleBy,
	{
		let db = mm.db();

		let user = sqlb::select()
			.table(Self::TABLE)
			.and_where("role_name", "=", role_name)
			.fetch_optional::<_, E>(db)
			.await?;

		Ok(user)
	}

	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		role_c: RoleForOp,
	) -> Result<i64> {
		let role_id = base::create::<Self, _>(ctx, mm, role_c).await?;

		Ok(role_id)
	}

	pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Role>> {
		base::list::<Self, _>(ctx, mm).await
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
