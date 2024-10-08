use crate::core::ctx::Ctx;
use crate::core::model::base::DbBmc;
use crate::core::model::ModelManager;
use crate::core::model::{Error, Result};
use crate::utils::time::Rfc3339;
use modql::field::{Fields, HasFields};
use modql::filter::FilterNodes;
use modql::filter::{FilterGroups, ListOptions};
use sea_query::Alias;
use sea_query::JoinType;
use sea_query::{Condition, Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::postgres::PgRow;
use sqlx::types::time::OffsetDateTime;
use sqlx::{FromRow, Row};
use tracing::debug;

use super::archive::Archive;
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

#[derive(Deserialize, Default, Debug, FilterNodes)]
pub struct ArchiveIndexFilter {
	index_id: i64,
	value: String,
	operator: String,
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

#[derive(Iden)]
pub enum ValueIden {
	#[iden = "value"]
	Table,
	IndexId,
	ArchiveId,
	Value,
}

#[derive(Iden)]
pub enum ArchiveIden {
	#[iden = "archive"]
	Table,
	Id,
	ProjectId,
	Owner,
	LastEditUser,
	Tag,
	Cid,
	Ctime,
	Mid,
	Mtime,
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

	pub async fn search_archives(
		_ctx: &Ctx,
		mm: &ModelManager,
		filters: Option<Vec<ArchiveIndexFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<Archive>> {
		let db = mm.db();

		let mut query = Query::select();
		query
			.columns([
				(ArchiveIden::Table, ArchiveIden::Id),
				(ArchiveIden::Table, ArchiveIden::ProjectId),
				(ArchiveIden::Table, ArchiveIden::Owner),
				(ArchiveIden::Table, ArchiveIden::LastEditUser),
				(ArchiveIden::Table, ArchiveIden::Tag),
				(ArchiveIden::Table, ArchiveIden::Cid),
				(ArchiveIden::Table, ArchiveIden::Ctime),
				(ArchiveIden::Table, ArchiveIden::Mid),
				(ArchiveIden::Table, ArchiveIden::Mtime),
			])
			.from(ArchiveIden::Table);

		if let Some(filters) = filters {
			use std::collections::HashMap;

			// Group filters by index_id
			let mut filters_by_index: HashMap<i64, Vec<&ArchiveIndexFilter>> =
				HashMap::new();

			for filter in &filters {
				filters_by_index
					.entry(filter.index_id)
					.or_insert_with(Vec::new)
					.push(filter);
			}

			let mut index = 0;
			for (index_id, filter_group) in filters_by_index {
				let alias = Alias::new(&format!("v{}", index));
				index += 1;

				let mut on_condition = Condition::all()
					.add(
						Expr::col((alias.clone(), ValueIden::ArchiveId))
							.equals((ArchiveIden::Table, ArchiveIden::Id)),
					)
					.add(
						Expr::col((alias.clone(), ValueIden::IndexId)).eq(index_id),
					);

				// Variables to hold Gte, Lte, and Eq conditions for the same index
				let mut gte_value: Option<String> = None;
				let mut lte_value: Option<String> = None;
				let mut eq_value: Option<String> = None;

				// Process each filter in the group
				for filter in filter_group {
					match filter.operator.as_str() {
						"Gte" => {
							gte_value = Some(filter.value.clone());
						}
						"Lte" => {
							lte_value = Some(filter.value.clone());
						}
						"Eq" => {
							eq_value = Some(filter.value.clone());
						}
						_ => {
							return Err(Error::UnsupportedOperator(
								filter.operator.clone(),
							));
						}
					}
				}

				// Apply Eq condition if it exists (precedence over Gte and Lte)
				if let Some(eq) = &eq_value {
					on_condition = on_condition.add(
						Expr::col((alias.clone(), ValueIden::Value)).eq(eq.clone()),
					);
				}
				// Apply BETWEEN if both Gte and Lte exist
				else if let (Some(gte), Some(lte)) = (&gte_value, &lte_value) {
					on_condition = on_condition.add(
						Expr::col((alias.clone(), ValueIden::Value))
							.between(gte.clone(), lte.clone()),
					);
				}
				// Apply Gte or Lte individually if only one exists
				else {
					if let Some(gte) = &gte_value {
						on_condition = on_condition.add(
							Expr::col((alias.clone(), ValueIden::Value))
								.gte(gte.clone()),
						);
					}
					if let Some(lte) = &lte_value {
						on_condition = on_condition.add(
							Expr::col((alias.clone(), ValueIden::Value))
								.lte(lte.clone()),
						);
					}
				}

				// Join the alias table based on the conditions
				query.join_as(
					JoinType::InnerJoin,
					ValueIden::Table,
					alias.clone(),
					on_condition,
				);
			}
		}

		// Apply list options like sorting and limits
		let list_options = compute_list_options(list_options)?;
		list_options.apply_to_sea_query(&mut query);

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

		// Optionally, print the generated SQL for debugging
		debug!("Generated SQL: {}", sql);

		let archives = sqlx::query_as_with::<_, Archive, _>(&sql, values)
			.fetch_all(db)
			.await?;

		Ok(archives)
	}

	/*
	pub async fn search_archives<F>(
		_ctx: &Ctx,
		mm: &ModelManager,
		filters: Option<F>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<Archive>>
	where
		F: Into<FilterGroups>,
	{
		if let Some(filters) = filters {
			let filter_groups: FilterGroups = filters.into();
			// Print the entire filter groups for inspection
			//
			let condition: Condition = filter_groups.try_into()?;
			dbg!(condition);
		}

		todo!()
		let db = mm.db();

		let mut query = Query::select();
		query
			.columns([
				(ArchiveIden::Table, ArchiveIden::Id),
				(ArchiveIden::Table, ArchiveIden::ProjectId),
				(ArchiveIden::Table, ArchiveIden::Owner),
				(ArchiveIden::Table, ArchiveIden::LastEditUser),
				(ArchiveIden::Table, ArchiveIden::Tag),
				(ArchiveIden::Table, ArchiveIden::Cid),
				(ArchiveIden::Table, ArchiveIden::Ctime),
				(ArchiveIden::Table, ArchiveIden::Mid),
				(ArchiveIden::Table, ArchiveIden::Mtime),
			])
			.from(ArchiveIden::Table)
			.inner_join(
				ValueIden::Table,
				Expr::col((ArchiveIden::Table, ArchiveIden::Id))
					.equals((ValueIden::Table, ValueIden::ArchiveId)),
			);

		if let Some(filters) = filters {
			let filter_groups: FilterGroups = filters.into();
			let condition: Condition = filter_groups.try_into()?;
			query.cond_where(condition);
		}

		let list_options = compute_list_options(list_options)?;
		list_options.apply_to_sea_query(&mut query);

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);

		let archives = sqlx::query_as_with::<_, Archive, _>(&sql, values)
			.fetch_all(db)
			.await?;

		Ok(archives)
	}
	*/
}
