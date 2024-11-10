use crate::core::ctx::Ctx;
use crate::core::model::base::{self, DbBmc};
use crate::core::model::modql_utils::time_to_sea_value;
use crate::core::model::ModelManager;
use crate::core::model::Result;
use crate::utils::time::Rfc3339;
use modql::field::{Fields, HasFields};
use modql::filter::{
	FilterGroups, FilterNodes, ListOptions, OpValsInt64, OpValsString, OpValsValue,
};
use sea_query::{
	Alias, Asterisk, Condition, ConditionalStatement, Expr, PostgresQueryBuilder,
	Query,
};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::postgres::PgRow;
use sqlx::types::time::OffsetDateTime;
use sqlx::{FromRow, Row};

use super::base::{compute_list_options, ListResult};
use super::idens::{StructureIden, StructurePrivilegeIden};

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Structure {
	pub id: i64,
	pub project_name: String,
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct StructureForOp {
	pub project_name: String,
}

#[allow(dead_code)]
pub trait StructureBy:
	HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send
{
}

impl StructureBy for Structure {}
impl StructureBy for StructureForOp {}

pub struct StructureBmc;

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct StructureFilter {
	id: Option<OpValsInt64>,
	project_name: Option<OpValsString>,
	cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	ctime: Option<OpValsValue>,
	mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	mtime: Option<OpValsValue>,
}

impl DbBmc for StructureBmc {
	const TABLE: &'static str = "structure";
	const TIMESTAMPED: bool = true;
}

impl StructureBmc {
	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Structure> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		structure_c: StructureForOp,
	) -> Result<i64> {
		let structure_id = base::create::<Self, _>(ctx, mm, structure_c).await?;

		Ok(structure_id)
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filters: Option<Vec<StructureFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<ListResult<Structure>> {
		let db = mm.db();

		// Build the base query
		let mut base_query = Query::select();
		base_query
			.from(Self::table_ref())
			.columns(Structure::field_column_refs())
			.and_where(
				Expr::col(StructureIden::Id).in_subquery(
					Query::select()
						.from(StructurePrivilegeIden::Table)
						.column(StructurePrivilegeIden::ProjectId)
						.and_where(
							Expr::col(StructurePrivilegeIden::UserId)
								.eq(ctx.user_id()),
						)
						.take(),
				),
			);

		if let Some(filter) = filters {
			let filters: FilterGroups = filter.into();
			let cond: Condition = filters.try_into()?;
			base_query.cond_where(cond);
		}

		let mut count_query = Query::select();
		count_query.from(Self::table_ref());

		count_query.expr_as(Expr::col(Asterisk).count(), Alias::new("total_count"));

		let (count_sql, count_values) = count_query.build_sqlx(PostgresQueryBuilder);
		let total_count_row = sqlx::query_with(&count_sql, count_values)
			.fetch_one(db)
			.await?;

		let total_count: i64 = total_count_row.get("total_count");

		let list_options = compute_list_options(list_options)?;
		list_options.apply_to_sea_query(&mut base_query);

		let (sql, values) = base_query.build_sqlx(PostgresQueryBuilder);
		let entities = sqlx::query_as_with::<_, Structure, _>(&sql, values)
			.fetch_all(db)
			.await?;

		// Return the result
		Ok(ListResult {
			total_count: total_count as usize,
			items: entities,
		})
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		structure_u: StructureForOp,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, structure_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
