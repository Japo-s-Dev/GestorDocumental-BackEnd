use crate::core::ctx::Ctx;
use crate::core::model::ModelManager;
use crate::core::model::{Error, Result};
use crate::utils::time::now_utc;
use modql::field::{Field, Fields, HasFields};
use modql::filter::{FilterGroups, ListOptions};
use modql::SIden;
use sea_query::Asterisk;
use sea_query::{
	Alias, Condition, Expr, Iden, IntoIden, PostgresQueryBuilder, Query, TableRef,
};
use sea_query_binder::SqlxBinder;
use serde::Serialize;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};

use super::idens::CommonIden;

const LIST_LIMIT_DEFAULT: i64 = 1000;
const LIST_LIMIT_MAX: i64 = 5000;

#[derive(Iden)]
pub enum TimestampIden {
	Cid,
	Ctime,
	Mid,
	Mtime,
}

#[derive(Serialize)]
pub struct ListResult<E> {
	pub total_count: usize,
	pub items: Vec<E>,
}

pub trait DbBmc {
	const TABLE: &'static str;
	const SCHEMA: Option<&'static str> = None;
	const TIMESTAMPED: bool;
	const SOFTDELETED: bool;

	fn table_ref() -> TableRef {
		match Self::SCHEMA {
			Some(schema) => TableRef::SchemaTable(
				SIden(schema).into_iden(),
				SIden(Self::TABLE).into_iden(),
			),
			None => TableRef::Table(SIden(Self::TABLE).into_iden()),
		}
	}
}

pub fn compute_list_options(
	list_options: Option<ListOptions>,
) -> Result<ListOptions> {
	if let Some(mut list_options) = list_options {
		if let Some(limit) = list_options.limit {
			if limit > LIST_LIMIT_MAX {
				return Err(Error::ListLimitOverMax {
					max: LIST_LIMIT_MAX,
					actual: limit,
				});
			}
		} else {
			list_options.limit = Some(LIST_LIMIT_DEFAULT);
		}

		Ok(list_options)
	} else {
		Ok(ListOptions {
			limit: Some(LIST_LIMIT_DEFAULT),
			offset: None,
			order_bys: Some("id".into()),
		})
	}
}

pub async fn create<MC, E>(ctx: &Ctx, mm: &ModelManager, data: E) -> Result<i64>
where
	MC: DbBmc,
	E: HasFields,
{
	let db = mm.db();

	// -- Prep data
	let mut fields = data.not_none_fields();
	if MC::TIMESTAMPED {
		add_timestamps_for_create(&mut fields, ctx.user_id());
	}
	let (columns, sea_values) = fields.for_sea_insert();

	// -- Build query
	let mut query = Query::insert();
	query
		.into_table(MC::table_ref())
		.columns(columns)
		.values(sea_values)?
		.returning(Query::returning().columns([CommonIden::Id]));

	// -- Exec query
	let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
	let (id,) = sqlx::query_as_with::<_, (i64,), _>(&sql, values)
		.fetch_one(db)
		.await?;

	Ok(id)
}

pub async fn get<MC, E>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
where
	MC: DbBmc,
	E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
	E: HasFields,
{
	let db = mm.db();

	let mut query = Query::select();
	query
		.from(MC::table_ref())
		.columns(E::field_column_refs())
		.and_where(Expr::col(CommonIden::Id).eq(id));

	let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
	let entity = sqlx::query_as_with(&sql, values)
		.fetch_optional(db)
		.await?
		.ok_or(Error::EntityNotFound {
			entity: MC::TABLE,
			id,
		})?;

	Ok(entity)
}

pub async fn list<MC, E, F>(
	_ctx: &Ctx,
	mm: &ModelManager,
	filter: Option<F>,
	list_options: Option<ListOptions>,
) -> Result<ListResult<E>>
where
	MC: DbBmc,
	F: Into<FilterGroups>,
	E: for<'r> FromRow<'r, PgRow> + Unpin + Send,
	E: HasFields,
{
	let db = mm.db();

	// Build the base query
	let mut base_query = Query::select();
	base_query
		.from(MC::table_ref())
		.columns(E::field_column_refs());

	if let Some(filter) = filter {
		let filters: FilterGroups = filter.into();
		let cond: Condition = filters.try_into()?;
		base_query.cond_where(cond);
	}

	if MC::SOFTDELETED {
		base_query.and_where(Expr::col(CommonIden::IsDeleted).eq(false));
	}

	// Clone the base query for counting
	// Build a separate count query
	let mut count_query = Query::select();
	count_query.from(MC::table_ref());

	// Modify the count query to select COUNT(*)
	count_query.expr_as(Expr::col(Asterisk).count(), Alias::new("total_count"));

	// Build and execute the count query
	let (count_sql, count_values) = count_query.build_sqlx(PostgresQueryBuilder);
	let total_count_row = sqlx::query_with(&count_sql, count_values)
		.fetch_one(db)
		.await?;

	// Use the `get` method to retrieve the total count
	let total_count: i64 = total_count_row.get("total_count");

	// Apply limit and offset to the base query
	let list_options = compute_list_options(list_options)?;
	list_options.apply_to_sea_query(&mut base_query);

	// Build and execute the original query
	let (sql, values) = base_query.build_sqlx(PostgresQueryBuilder);
	let entities = sqlx::query_as_with::<_, E, _>(&sql, values)
		.fetch_all(db)
		.await?;

	// Return the result
	Ok(ListResult {
		total_count: total_count as usize,
		items: entities,
	})
}

pub async fn update<MC, E>(
	ctx: &Ctx,
	mm: &ModelManager,
	id: i64,
	data: E,
) -> Result<()>
where
	MC: DbBmc,
	E: HasFields,
{
	let db = mm.db();

	let mut fields = data.not_none_fields();
	if MC::TIMESTAMPED {
		add_timestamps_for_update(&mut fields, ctx.user_id());
	}
	let fields = fields.for_sea_update();

	let mut query = Query::update();
	query
		.table(MC::table_ref())
		.values(fields)
		.and_where(Expr::col(CommonIden::Id).eq(id));

	let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
	let count = sqlx::query_with(&sql, values)
		.execute(db)
		.await?
		.rows_affected();

	if count == 0 {
		Err(Error::EntityNotFound {
			entity: MC::TABLE,
			id,
		})
	} else {
		Ok(())
	}
}
async fn soft_delete<MC>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()>
where
	MC: DbBmc,
{
	let db = mm.db();

	let mut query = Query::update();
	query
		.table(MC::table_ref())
		.value(CommonIden::IsDeleted, true)
		.and_where(Expr::col(CommonIden::Id).eq(id));

	let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
	let count = sqlx::query_with(&sql, values)
		.execute(db)
		.await?
		.rows_affected();

	if count == 0 {
		Err(Error::EntityNotFound {
			entity: MC::TABLE,
			id,
		})
	} else {
		Ok(())
	}
}

async fn phisical_delete<MC>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()>
where
	MC: DbBmc,
{
	let db = mm.db();

	let mut query = Query::delete();
	query
		.from_table(MC::table_ref())
		.and_where(Expr::col(CommonIden::Id).eq(id));

	let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
	let count = sqlx::query_with(&sql, values)
		.execute(db)
		.await?
		.rows_affected();

	if count == 0 {
		Err(Error::EntityNotFound {
			entity: MC::TABLE,
			id,
		})
	} else {
		Ok(())
	}
}

pub async fn delete<MC>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()>
where
	MC: DbBmc,
{
	if MC::SOFTDELETED {
		soft_delete::<MC>(ctx, mm, id).await
	} else {
		phisical_delete::<MC>(ctx, mm, id).await
	}
}

pub async fn restore<MC>(_ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()>
where
	MC: DbBmc,
{
	if MC::SOFTDELETED {
		let db = mm.db();

		let mut query = Query::update();
		query
			.table(MC::table_ref())
			.value(CommonIden::IsDeleted, false)
			.and_where(Expr::col(CommonIden::Id).eq(id));

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let count = sqlx::query_with(&sql, values)
			.execute(db)
			.await?
			.rows_affected();

		if count == 0 {
			Err(Error::EntityNotFound {
				entity: MC::TABLE,
				id,
			})
		} else {
			Ok(())
		}
	} else {
		Err(Error::UnrecoverableItem {
			entity: MC::TABLE,
			id,
		})
	}
}

pub fn add_timestamps_for_create(fields: &mut Fields, user_id: i64) {
	let now = now_utc();
	fields.push(Field::new(TimestampIden::Cid.into_iden(), user_id.into()));
	fields.push(Field::new(TimestampIden::Ctime.into_iden(), now.into()));

	fields.push(Field::new(TimestampIden::Mid.into_iden(), user_id.into()));
	fields.push(Field::new(TimestampIden::Mtime.into_iden(), now.into()));
}

/// Update the timestamps info only for update.
/// (.e.g., only mid, mtime will be udpated)
pub fn add_timestamps_for_update(fields: &mut Fields, user_id: i64) {
	let now = now_utc();
	fields.push(Field::new(TimestampIden::Mid.into_iden(), user_id.into()));
	fields.push(Field::new(TimestampIden::Mtime.into_iden(), now.into()));
}
