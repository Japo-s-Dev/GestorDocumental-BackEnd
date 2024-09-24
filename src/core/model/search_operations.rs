use crate::core::ctx::Ctx;
use crate::core::model::base::DbBmc;
use crate::core::model::ModelManager;
use crate::core::model::Result;
use crate::utils::time::Rfc3339;
use modql::field::{Fields, HasFields};
use modql::filter::{FilterGroups, ListOptions};
use sea_query::{Condition, Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::Serialize;
use serde_with::serde_as;
use sqlx::postgres::PgRow;
use sqlx::types::time::OffsetDateTime;
use sqlx::{FromRow, Row};

use super::base::compute_list_options;

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct EventWithUsername {
	pub id: i64,
	pub archive_id: i64,
	pub user_id: i64,
	pub username: String,
	pub action: String,
	pub object: String,
	pub object_id: i64,
	#[serde_as(as = "Rfc3339")]
	pub timestamp: OffsetDateTime,
}

#[derive(Debug, Serialize, FromRow, Fields, Clone)]
pub struct IndexWithDatatype {
	id: i64,
	project_id: i64,
	required: bool,
	index_name: String,
	datatype_name: String,
}

#[allow(dead_code)]
pub trait SearchBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl SearchBy for EventWithUsername {}
impl SearchBy for IndexWithDatatype {}

#[derive(Iden)]
pub enum UserIden {
	#[iden = "user"]
	Table,
	Id,
	Username,
}

#[derive(Iden)]
pub enum EventIden {
	#[iden = "event"]
	Table,
	Id,
	UserId,
	Action,
	Object,
	ObjectId,
	Timestamp,
	ArchiveId,
}

#[derive(Iden)]
pub enum IndexIden {
	#[iden = "index"]
	Table,
	Id,
	ProjectId,
	Required,
	IndexName,
	DatatypeId,
}

#[derive(Iden)]
pub enum DatatypeIden {
	#[iden = "datatype"]
	Table,
	Id,
	DatatypeName,
}

pub struct SearchBmc;

impl DbBmc for SearchBmc {
	const TABLE: &'static str = "index";
}

impl SearchBmc {
	pub async fn get_events_with_filters<F>(
		_ctx: &Ctx,
		mm: &ModelManager,
		filters: Option<F>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<EventWithUsername>>
	where
		F: Into<FilterGroups>,
	{
		let db = mm.db();

		// Build the query using SeaQuery
		let mut query = Query::select();
		query
			.columns([
				(EventIden::Table, EventIden::Id),
				(EventIden::Table, EventIden::UserId),
				(EventIden::Table, EventIden::Action),
				(EventIden::Table, EventIden::Object),
				(EventIden::Table, EventIden::ObjectId),
				(EventIden::Table, EventIden::Timestamp),
				(EventIden::Table, EventIden::ArchiveId),
			])
			.column((UserIden::Table, UserIden::Username))
			.from(EventIden::Table)
			.inner_join(
				UserIden::Table,
				Expr::col((EventIden::Table, EventIden::UserId))
					.equals((UserIden::Table, UserIden::Id)),
			);

		// Apply filters if provided
		if let Some(filters) = filters {
			let filter_groups: FilterGroups = filters.into();
			let condition: Condition = filter_groups.try_into()?;
			query.cond_where(condition);
		}

		// Apply list options (pagination, order, limit)
		let list_options = compute_list_options(list_options)?;
		list_options.apply_to_sea_query(&mut query);

		// Build and bind the query
		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

		// Execute the query
		let rows = sqlx::query_with(&sql, values).fetch_all(db).await?;

		let events = rows
			.iter()
			.map(|row| EventWithUsername {
				id: row.get("id"),
				user_id: row.get("user_id"),
				archive_id: row.get("archive_id"),
				username: row.get("username"),
				action: row.get("action"),
				object: row.get("object"),
				object_id: row.get("object_id"),
				timestamp: row.get("timestamp"),
			})
			.collect();

		Ok(events)
	}

	pub async fn get_indexes_with_filters<F>(
		_ctx: &Ctx,
		mm: &ModelManager,
		filters: Option<F>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<IndexWithDatatype>>
	where
		F: Into<FilterGroups>,
	{
		let db = mm.db();

		let mut query = Query::select();
		query
			.columns([
				(IndexIden::Table, IndexIden::Id),
				(IndexIden::Table, IndexIden::ProjectId),
				(IndexIden::Table, IndexIden::Required),
				(IndexIden::Table, IndexIden::IndexName),
			])
			.column((DatatypeIden::Table, DatatypeIden::DatatypeName))
			.from(IndexIden::Table)
			.inner_join(
				DatatypeIden::Table,
				Expr::col((IndexIden::Table, IndexIden::DatatypeId))
					.equals((DatatypeIden::Table, DatatypeIden::Id)),
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

		let indexes = rows
			.iter()
			.map(|row| IndexWithDatatype {
				id: row.get("id"),
				project_id: row.get("project_id"),
				required: row.get("required"),
				index_name: row.get("index_name"),
				datatype_name: row.get("datatype_name"),
			})
			.collect();

		Ok(indexes)
	}
}
