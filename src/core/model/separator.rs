use crate::core::ctx::Ctx;
use crate::core::model::base::{self, DbBmc};
use crate::core::model::ModelManager;
use crate::core::model::Result;
use modql::field::{Fields, HasFields};
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::FromRow;

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Separator {
	pub id: i64,
	pub name: String,
	pub parent_id: Option<i64>,
	pub archive_id: i64,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct SeparatorForCreate {
	pub name: String,
	pub parent_id: Option<i64>,
	pub archive_id: i64,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct SeparatorForUpdate {
	pub name: String,
}

pub trait SeparatorBy:
	HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send
{
}

impl SeparatorBy for Separator {}
impl SeparatorBy for SeparatorForCreate {}
impl SeparatorBy for SeparatorForUpdate {}

pub struct SeparatorBmc;

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct SeparatorFilter {
	id: Option<OpValsInt64>,

	name: Option<OpValsString>,
	parent_id: Option<OpValsInt64>,
	archive_id: Option<OpValsInt64>,
}

impl DbBmc for SeparatorBmc {
	const TABLE: &'static str = "separator";
}

impl SeparatorBmc {
	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Separator> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		separator_c: SeparatorForCreate,
	) -> Result<i64> {
		let separator_id = base::create::<Self, _>(ctx, mm, separator_c).await?;

		Ok(separator_id)
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filters: Option<Vec<SeparatorFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<Separator>> {
		base::list::<Self, _, _>(ctx, mm, filters, list_options).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		separator_u: SeparatorForUpdate,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, separator_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
