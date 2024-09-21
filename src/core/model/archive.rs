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

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Archive {
	pub id: i64,
	pub project_id: i64,
	#[serde_as(as = "Rfc3339")]
	pub creation_date: OffsetDateTime,
	#[serde_as(as = "Rfc3339")]
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

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct ArchiveForInsertCreate {
	pub project_id: i64,
	pub owner: i64,
	#[serde_as(as = "Rfc3339")]
	pub modified_date: OffsetDateTime,
	pub last_edit_user: i64,
	pub tag: String,
}

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct ArchiveForInsertUpdate {
	#[serde_as(as = "Rfc3339")]
	pub modified_date: OffsetDateTime,
	pub last_edit_user: i64,
	pub tag: String,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct ArchiveFilter {
	id: Option<OpValsInt64>,

	project_id: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	creation_date: Option<OpValsValue>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	modified_date: Option<OpValsValue>,
	owner: Option<OpValsInt64>,
	last_edit_user: Option<OpValsInt64>,
	tag: Option<OpValsString>,
}

#[allow(dead_code)]
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

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filters: Option<Vec<ArchiveFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<Archive>> {
		base::list::<Self, _, _>(ctx, mm, filters, list_options).await
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
