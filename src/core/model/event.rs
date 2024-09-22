use crate::core::ctx::Ctx;
use crate::core::model::base::{self, DbBmc};
use crate::core::model::ModelManager;
use crate::core::model::Result;
use crate::utils::time::Rfc3339;
use modql::field::{Fields, HasFields};
use modql::filter::{FilterNodes, ListOptions, OpValsInt64};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::postgres::PgRow;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Event {
	pub id: i64,
	pub archive_id: i64,
	pub user_id: i64,
	pub action: String,
	pub object: String,
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct EventForOp {
	pub text: String,
}

#[allow(dead_code)]
pub trait EventBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl EventBy for Event {}
impl EventBy for EventForOp {}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct EventFilter {
	id: Option<OpValsInt64>,

	datatype_name: Option<OpValsInt64>,
}

pub struct EventBmc;

impl DbBmc for EventBmc {
	const TABLE: &'static str = "event";
}

impl EventBmc {
	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Event> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		event_c: EventForOp,
	) -> Result<i64> {
		let event_id = base::create::<Self, _>(ctx, mm, event_c).await?;

		Ok(event_id)
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filters: Option<Vec<EventFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<Event>> {
		base::list::<Self, _, _>(ctx, mm, filters, list_options).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		event_u: EventForOp,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, event_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
