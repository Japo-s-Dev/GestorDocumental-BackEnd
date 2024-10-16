use std::collections::HashMap;

use crate::core::ctx::Ctx;
use crate::core::model::base::DbBmc;
use crate::core::model::ModelManager;
use crate::core::model::{Error, Result};
use crate::utils::time::Rfc3339;
use modql::field::{Fields, HasFields};
use modql::filter::FilterNodes;
use modql::filter::{FilterGroups, ListOptions};
use sea_query::{Condition, Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::postgres::PgRow;
use sqlx::types::time::OffsetDateTime;
use sqlx::{FromRow, QueryBuilder, Row};
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

#[derive(Deserialize, Default)]
pub struct Listoptions {
	pub order_bys: Option<String>, // e.g., "!id,name"
	pub limit: Option<i64>,
	pub offset: Option<i64>,
}

#[derive(Deserialize, Default, Debug, FilterNodes)]
pub struct ArchiveIndexFilter {
	index_id: i64,
	value: String,
	operator: String,
	datatype_id: i64,
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
		list_options: Option<Listoptions>,
	) -> Result<Vec<Archive>> {
		let db = mm.db();

		// Unwrap filters and list options or use default values.
		let filters = filters.unwrap_or_default();
		let list_options = list_options.unwrap_or_default();

		// Initialize the query builder with the SELECT clause.
		let mut query_builder = QueryBuilder::new("SELECT ");

		// Define fields to select from the archive table.
		let archive_fields = [
			"id",
			"project_id",
			"owner",
			"last_edit_user",
			"tag",
			"cid",
			"ctime",
			"mid",
			"mtime",
		];

		// Build the SELECT clause.
		for (i, field) in archive_fields.iter().enumerate() {
			if i > 0 {
				query_builder.push(", ");
			}
			query_builder.push(format!("\"archive\".\"{}\"", field));
		}

		// Group filters by index_id and ensure datatype consistency.
		let mut filters_by_index: HashMap<i64, Vec<&ArchiveIndexFilter>> =
			HashMap::new();
		let mut index_datatype_ids: HashMap<i64, i64> = HashMap::new();

		for filter in &filters {
			filters_by_index
				.entry(filter.index_id)
				.or_insert_with(Vec::new)
				.push(filter);

			// Ensure that the datatype_id is consistent for each index_id.
			if let Some(&existing_datatype_id) =
				index_datatype_ids.get(&filter.index_id)
			{
				if existing_datatype_id != filter.datatype_id {
					return Err(Error::UnsupportedDatatype(filter.index_id));
				}
			} else {
				index_datatype_ids.insert(filter.index_id, filter.datatype_id);
			}
		}

		// Add value columns to SELECT clause.
		for (n, (&index_id, _)) in filters_by_index.iter().enumerate() {
			query_builder.push(", ");
			query_builder
				.push(format!("\"v{n}\".\"value\" AS \"v{index_id}_value\""));
		}

		// FROM clause.
		query_builder.push(" FROM \"archive\"");

		// Build INNER JOINs for each index_id.
		let mut join_index = 0;

		for (&index_id, index_filters) in &filters_by_index {
			let alias = format!("v{}", join_index);
			let datatype_id = index_datatype_ids
				.get(&index_id)
				.ok_or_else(|| Error::UnsupportedDatatype(index_id))?;

			query_builder.push(format!(
            " INNER JOIN \"value\" AS \"{}\" ON \"{}\".\"archive_id\" = \"archive\".\"id\" AND \"{}\".\"index_id\" = ",
            alias, alias, alias
        ));
			query_builder.push_bind(index_id);

			// Collect conditions for this index_id.
			let mut eq_values = Vec::new();
			let mut gte_value = None;
			let mut lte_value = None;

			for filter in index_filters {
				match filter.operator.as_str() {
					"Eq" => eq_values.push(&filter.value),
					"Gte" => gte_value = Some(&filter.value),
					"Lte" => lte_value = Some(&filter.value),
					_ => return Err(Error::UnsupportedDatatype(index_id)),
				}
			}

			// Adjust casting based on datatype_id and cast both sides.
			match datatype_id {
				3 => {
					// TEXT
					if !eq_values.is_empty() {
						for value in eq_values {
							query_builder.push(" AND ");
							query_builder
								.push(format!("\"{}\".\"value\" = ", alias));
							query_builder.push_bind(value);
						}
					}
					if gte_value.is_some() || lte_value.is_some() {
						return Err(Error::UnsupportedDatatype(index_id));
					}
				}
				4 => {
					// NUMERIC
					if !eq_values.is_empty() {
						for value in eq_values {
							query_builder.push(" AND ");
							query_builder.push(format!(
								"CAST(\"{}\".\"value\" AS NUMERIC) = CAST(",
								alias
							));
							query_builder.push_bind(value);
							query_builder.push(" AS NUMERIC)");
						}
					}
					if gte_value.is_some() && lte_value.is_some() {
						query_builder.push(" AND CAST(\"");
						query_builder.push(&alias);
						query_builder.push("\".\"value\" AS NUMERIC) BETWEEN CAST(");
						query_builder.push_bind(gte_value.unwrap());
						query_builder.push(" AS NUMERIC) AND CAST(");
						query_builder.push_bind(lte_value.unwrap());
						query_builder.push(" AS NUMERIC)");
					} else {
						if let Some(value) = gte_value {
							query_builder.push(" AND CAST(\"");
							query_builder.push(&alias);
							query_builder.push("\".\"value\" AS NUMERIC) >= CAST(");
							query_builder.push_bind(value);
							query_builder.push(" AS NUMERIC)");
						}
						if let Some(value) = lte_value {
							query_builder.push(" AND CAST(\"");
							query_builder.push(&alias);
							query_builder.push("\".\"value\" AS NUMERIC) <= CAST(");
							query_builder.push_bind(value);
							query_builder.push(" AS NUMERIC)");
						}
					}
				}
				5 => {
					// TIMESTAMP stored as mm-dd-yyyy
					let date_format = "YYYY-MM-DD";
					if !eq_values.is_empty() {
						for value in eq_values {
							query_builder.push(" AND ");
							query_builder.push(format!(
								"TO_DATE(\"{}\".\"value\", '{}') = TO_DATE(",
								alias, date_format
							));
							query_builder.push_bind(value);
							query_builder.push(format!(", '{}')", date_format));
						}
					}
					if gte_value.is_some() && lte_value.is_some() {
						query_builder.push(" AND TO_DATE(\"");
						query_builder.push(&alias);
						query_builder.push(format!(
							"\".\"value\", '{}') BETWEEN TO_DATE(",
							date_format
						));
						query_builder.push_bind(gte_value.unwrap());
						query_builder
							.push(format!(", '{}') AND TO_DATE(", date_format));
						query_builder.push_bind(lte_value.unwrap());
						query_builder.push(format!(", '{}')", date_format));
					} else {
						if let Some(value) = gte_value {
							query_builder.push(" AND TO_DATE(\"");
							query_builder.push(&alias);
							query_builder.push(format!(
								"\".\"value\", '{}') >= TO_DATE(",
								date_format
							));
							query_builder.push_bind(value);
							query_builder.push(format!(", '{}')", date_format));
						}
						if let Some(value) = lte_value {
							query_builder.push(" AND TO_DATE(\"");
							query_builder.push(&alias);
							query_builder.push(format!(
								"\".\"value\", '{}') <= TO_DATE(",
								date_format
							));
							query_builder.push_bind(value);
							query_builder.push(format!(", '{}')", date_format));
						}
					}
				}
				_ => return Err(Error::UnsupportedDatatype(index_id)),
			}

			join_index += 1;
		}

		if let Some(order_bys) = &list_options.order_bys {
			let order_by_fields = order_bys.split(',').collect::<Vec<&str>>();
			query_builder.push(" ORDER BY ");

			for (i, field) in order_by_fields.iter().enumerate() {
				let (order_field, order_direction) = if field.starts_with('!') {
					(&field[1..], "DESC")
				} else {
					(field.as_ref(), "ASC")
				};

				if !archive_fields.contains(&order_field) {
					return Err(Error::UnsupportedDatatype(1));
				}

				if i > 0 {
					query_builder.push(", ");
				}

				query_builder.push(format!(
					"\"archive\".\"{}\" {}",
					order_field, order_direction
				));
			}
		} else {
			// Default ordering if none provided.
			query_builder.push(" ORDER BY \"archive\".\"id\" ASC");
		}

		if let Some(limit) = list_options.limit {
			query_builder.push(" LIMIT ");
			query_builder.push_bind(limit);
		}

		if let Some(offset) = list_options.offset {
			query_builder.push(" OFFSET ");
			query_builder.push_bind(offset);
		}

		// Build and execute the query.
		let query = query_builder.build();

		let archives = query
			.map(|row: sqlx::postgres::PgRow| Archive {
				id: row.get("id"),
				project_id: row.get("project_id"),
				owner: row.get("owner"),
				last_edit_user: row.get("last_edit_user"),
				tag: row.get("tag"),
				cid: row.get("cid"),
				ctime: row.get("ctime"),
				mid: row.get("mid"),
				mtime: row.get("mtime"),
			})
			.fetch_all(db)
			.await?;

		Ok(archives)
	}
}
