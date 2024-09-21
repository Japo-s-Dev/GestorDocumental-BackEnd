use crate::core::ctx::Ctx;
use crate::core::model::base::{self, DbBmc};
use crate::core::model::ModelManager;
use crate::core::model::Result;
use modql::field::{Fields, HasFields};
use modql::filter::{
	FilterNodes, ListOptions, OpValsBool, OpValsInt64, OpValsString,
};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Index {
	pub id: i64,
	pub datatype_id: i64,
	pub project_id: i64,
	pub required: bool,
	pub index_name: String,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct IndexForCreate {
	pub datatype_id: i64,
	pub project_id: i64,
	pub required: bool,
	pub index_name: String,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct IndexForUpdate {
	pub datatype_id: i64,
	pub required: bool,
	pub index_name: String,
}

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct IndexWithDatatype {
	id: i64,
	project_id: i64,
	required: bool,
	index_name: String,
	datatype_name: String,
}

#[allow(dead_code)]
pub trait IndexBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl IndexBy for Index {}
impl IndexBy for IndexForCreate {}
impl IndexBy for IndexForUpdate {}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct IndexFilter {
	id: Option<OpValsInt64>,
	datatype_id: Option<OpValsInt64>,
	project_id: Option<OpValsInt64>,
	required: Option<OpValsBool>,
	index_name: Option<OpValsString>,
}

pub struct IndexBmc;

impl DbBmc for IndexBmc {
	const TABLE: &'static str = "index";
}

impl IndexBmc {
	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Index> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		index_c: IndexForCreate,
	) -> Result<i64> {
		let index_id = base::create::<Self, _>(ctx, mm, index_c).await?;

		Ok(index_id)
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filters: Option<Vec<IndexFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<Index>> {
		base::list::<Self, _, _>(ctx, mm, filters, list_options).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		index_u: IndexForUpdate,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, index_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}

	pub async fn get_indexes_by_project(
		_ctx: &Ctx,
		mm: &ModelManager,
		project_id: i64,
	) -> Result<Vec<IndexWithDatatype>> {
		let db = mm.db();

		let rows = sqlx::query(
            r#"select i.id, i.project_id, i.required, i.index_name, d.datatype_name from public.index i
            join public.datatype d on i.datatype_id = d.id
            where i.project_id = $1;"#
        )
            .bind(project_id)
            .fetch_all(db)
            .await?;

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
