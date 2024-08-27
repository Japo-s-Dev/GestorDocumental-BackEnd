use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlb::{Fields, HasFields};
use sqlx::postgres::PgRow;
use sqlx::FromRow;

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Archive {
	pub id: i64,
	pub datatype_name: String,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct ArchiveForOp {
	pub datatype_name: String,
}

pub trait DatatypeBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl DatatypeBy for Datatype {}
impl DatatypeBy for DatatypeForOp {}

pub struct DatatypeBmc;

impl DbBmc for DatatypeBmc {
	const TABLE: &'static str = "datatype";
}

impl DatatypeBmc {
	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Datatype> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn first_by_role_name<E>(
		_ctx: &Ctx,
		mm: &ModelManager,
		role_name: &str,
	) -> Result<Option<E>>
	where
		E: DatatypeBy,
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
		role_c: DatatypeForOp,
	) -> Result<i64> {
		let role_id = base::create::<Self, _>(ctx, mm, role_c).await?;

		Ok(role_id)
	}

	pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Datatype>> {
		base::list::<Self, _>(ctx, mm).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		role_u: DatatypeForOp,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, role_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
