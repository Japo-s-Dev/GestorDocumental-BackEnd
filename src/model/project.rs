use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlb::{Fields, HasFields};
use sqlx::postgres::PgRow;
use sqlx::FromRow;

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Project {
	pub id: i64,
	pub project_name: String,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct ProjectForOp {
	pub project_name: String,
}

pub trait ProjectBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl ProjectBy for Project {}
impl ProjectBy for ProjectForOp {}

pub struct ProjectBmc;

impl DbBmc for ProjectBmc {
	const TABLE: &'static str = "project";
}

impl ProjectBmc {
	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Project> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		project_c: ProjectForOp,
	) -> Result<i64> {
		let project_id = base::create::<Self, _>(ctx, mm, project_c).await?;

		Ok(project_id)
	}

	pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Project>> {
		base::list::<Self, _>(ctx, mm).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		project_u: ProjectForOp,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, project_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
