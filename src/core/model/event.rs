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
pub struct Event {
	pub id: i64,
	pub archive_id: i64,
	pub user_id: i64,
	pub action: String,
	pub object: String,
	pub object_id: i64,
	#[serde_as(as = "Rfc3339")]
	pub timestamp: OffsetDateTime,
}

#[allow(dead_code)]
pub trait EventBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl EventBy for Event {}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct EventFilter {
	id: Option<OpValsInt64>,

	archive_id: Option<OpValsInt64>,
	user_id: Option<OpValsInt64>,
	action: Option<OpValsString>,
	object: Option<OpValsString>,
	object_id: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	timestamp: Option<OpValsValue>,
}

pub struct EventBmc;

impl DbBmc for EventBmc {
	const TABLE: &'static str = "event";
}

impl EventBmc {
	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filters: Option<Vec<EventFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<Event>> {
		base::list::<Self, _, _>(ctx, mm, filters, list_options).await
	}
}
