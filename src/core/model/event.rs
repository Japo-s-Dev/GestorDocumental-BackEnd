use crate::core::ctx::Ctx;
use crate::core::model::base::{self, DbBmc};
use crate::core::model::idens::*;
use crate::core::model::modql_utils::time_to_sea_value;
use crate::core::model::ModelManager;
use crate::core::model::Result;
use crate::utils::time::Rfc3339;
use modql::field::{Fields, HasFields};
use modql::filter::{
	FilterGroups, FilterNodes, ListOptions, OpValsInt64, OpValsString, OpValsValue,
};
use sea_query::{Condition, Expr, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::postgres::PgRow;
use sqlx::types::time::OffsetDateTime;
use sqlx::{FromRow, Row};

use super::base::compute_list_options;

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Event {
	pub id: i64,
	pub user_id: i64,
	pub action: String,
	pub object: String,
	pub object_id: i64,

	#[serde_as(as = "Rfc3339")]
	pub timestamp: OffsetDateTime,
	pub old_data: Option<serde_json::Value>,
	pub new_data: Option<serde_json::Value>,
	pub additional_info: Option<serde_json::Value>,
}

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct EventWithUsername {
	pub id: i64,
	pub username: String,
	pub action: String,
	pub object: String,
	pub object_id: i64,

	#[serde_as(as = "Rfc3339")]
	pub timestamp: OffsetDateTime,
	pub old_data: Option<serde_json::Value>,
	pub new_data: Option<serde_json::Value>,
	pub additional_info: Option<serde_json::Value>,
}

#[allow(dead_code)]
pub trait EventBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl EventBy for Event {}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct EventFilter {
	id: Option<OpValsInt64>,
	username: Option<OpValsString>,
	action: Option<OpValsString>,
	object: Option<OpValsString>,
	object_id: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	timestamp: Option<OpValsValue>,
}

pub struct EventBmc;

impl DbBmc for EventBmc {
	const TABLE: &'static str = "event";
	const TIMESTAMPED: bool = false;
	const SOFTDELETED: bool = false;
}

impl EventBmc {
	#[allow(unused)]
	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Event> {
		base::get::<Self, _>(ctx, mm, id).await
	}
	/*
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		comment_c: CommentForOp,
	) -> Result<i64> {
		let comment_data = ArchiveCommentForOpInsert {
			archive_id: comment_c.archive_id,
			text: comment_c.text,
			user_id: ctx.user_id(),
		};

		let comment_id = base::create::<Self, _>(ctx, mm, comment_data).await?;

		Ok(comment_id)
	}
	*/
	pub async fn list<F>(
		_ctx: &Ctx,
		mm: &ModelManager,
		filters: Option<F>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<EventWithUsername>>
	where
		F: Into<FilterGroups>,
	{
		let db = mm.db();

		let mut query = Query::select();
		query
			.columns([
				(EventIden::Table, EventIden::Id),
				(EventIden::Table, EventIden::Action),
				(EventIden::Table, EventIden::Object),
				(EventIden::Table, EventIden::ObjectId),
				(EventIden::Table, EventIden::Timestamp),
				(EventIden::Table, EventIden::OldData),
				(EventIden::Table, EventIden::NewData),
				(EventIden::Table, EventIden::AdditionalInfo),
			])
			.column((UserIden::Table, UserIden::Username))
			.from(EventIden::Table)
			.inner_join(
				UserIden::Table,
				Expr::col((EventIden::Table, EventIden::UserId))
					.equals((UserIden::Table, UserIden::Id)),
			);

		if let Some(filters) = filters {
			let filter_groups: FilterGroups = filters.into();
			let condition: Condition = filter_groups.try_into()?;
			query.cond_where(condition);
		}

		let list_options = compute_list_options(list_options)?;
		list_options.apply_to_sea_query(&mut query);
		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

		let rows = sqlx::query_with(&sql, values).fetch_all(db).await?;

		let events = rows
			.iter()
			.map(|row| EventWithUsername {
				id: row.get("id"),
				username: row.get("username"),
				action: row.get("action"),
				object: row.get("object"),
				object_id: row.get("object_id"),
				timestamp: row.get("timestamp"),
				old_data: row.get("old_data"),
				new_data: row.get("new_data"),
				additional_info: row.get("additional_info"),
			})
			.collect();

		Ok(events)
	}

	/*
	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		comment_u: ArchiveCommentForOp,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, comment_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
	*/
}
