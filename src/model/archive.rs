use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlb::{Fields, HasFields};
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Archive {
	pub id: i64,
	pub project_id: i64,
	pub creation_date: OffsetDateTime,
	pub modified_date: OffsetDateTime,
	pub owner: i64,
	pub last_edit_user: i64,
	pub tag: String,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct ArchiveForCreate {
	pub project_id: i64,
	pub tag: String,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct ArchiveForUpdate {
	pub tag: String,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct ArchiveForInsertCreate {
	pub project_id: i64,
	pub owner: i64,
	pub modified_date: OffsetDateTime,
	pub last_edit_user: i64,
	pub tag: String,
}
#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct ArchiveForInsertUpdate {
	pub modified_date: OffsetDateTime,
	pub last_edit_user: i64,
	pub tag: String,
}

pub trait ArchiveBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl ArchiveBy for Archive {}
impl ArchiveBy for ArchiveForCreate {}
impl ArchiveBy for ArchiveForUpdate {}
impl ArchiveBy for ArchiveForInsertCreate {}
impl ArchiveBy for ArchiveForInsertUpdate {}

pub struct ArchiveBmc;

impl DbBmc for ArchiveBmc {
	const TABLE: &'static str = "archive";
}

impl ArchiveBmc {
	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Archive> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		archive_op: ArchiveForCreate,
	) -> Result<i64> {
		let archive_insert = ArchiveForInsertCreate {
			modified_date: OffsetDateTime::now_utc(),
			owner: ctx.user_id(),
			project_id: archive_op.project_id,
			last_edit_user: ctx.user_id(),
			tag: archive_op.tag,
		};

		let archive_id = base::create::<Self, _>(ctx, mm, archive_insert).await?;

		Ok(archive_id)
	}

	pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Archive>> {
		base::list::<Self, _>(ctx, mm).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		archive_op: ArchiveForUpdate,
	) -> Result<()> {
		let archive_insert = ArchiveForInsertUpdate {
			modified_date: OffsetDateTime::now_utc(),
			last_edit_user: ctx.user_id(),
			tag: archive_op.tag,
		};

		base::update::<Self, _>(ctx, mm, id, archive_insert).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
