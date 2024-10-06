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
use sea_query::{Expr, Iden, PostgresQueryBuilder, Query};
use sea_query_binder::SqlxBinder;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::postgres::PgRow;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Document {
	pub id: i64,
	pub separator_id: i64,
	pub archive_id: i64,
	pub name: String,
	pub doc_type: String,
	pub owner: i64,
	pub last_edit_user: i64,
	pub key: String,
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct DocumentForRequest {
	pub separator_id: i64,
	pub name: Option<String>,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct DocumentForCreate {
	pub separator_id: i64,
	pub archive_id: i64,
	pub name: String,
	pub doc_type: String,
	pub key: String,
}

#[derive(Clone, Fields, FromRow, Debug, Deserialize)]
pub struct DocumentForUpdate {
	pub archive_id: i64,
	pub separator_id: i64,
	pub name: String,
	pub doc_type: String,
	pub key: String,
}

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct DocumentForCreateInsert {
	pub separator_id: i64,
	pub archive_id: i64,
	pub name: String,
	pub doc_type: String,
	pub owner: i64,
	pub last_edit_user: i64,
	pub key: String,
}

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct DocumentForUpdateInsert {
	pub name: String,
	pub last_edit_user: i64,
	pub separator_id: i64,
	pub archive_id: i64,
	pub key: String,
	pub doc_type: String,
}

#[allow(dead_code)]
pub trait DocumentBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl DocumentBy for Document {}
impl DocumentBy for DocumentForUpdate {}
impl DocumentBy for DocumentForCreate {}
impl DocumentBy for DocumentForUpdateInsert {}
impl DocumentBy for DocumentForCreateInsert {}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct DocumentFilter {
	id: Option<OpValsInt64>,
	archive_id: Option<OpValsInt64>,
	separator_id: Option<OpValsInt64>,
	name: Option<OpValsString>,
	doc_type: Option<OpValsString>,
	owner: Option<OpValsInt64>,
	last_edit_user: Option<OpValsInt64>,
	url: Option<OpValsString>,
	cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	ctime: Option<OpValsValue>,
	mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	mtime: Option<OpValsValue>,
}

#[derive(Iden)]
enum DocumentIden {
	ArchiveId,
}

pub struct DocumentBmc;

impl DbBmc for DocumentBmc {
	const TABLE: &'static str = "document";
}

impl DocumentBmc {
	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Document> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		document_c: DocumentForCreate,
	) -> Result<i64> {
		let document = DocumentForCreateInsert {
			archive_id: document_c.archive_id,
			separator_id: document_c.separator_id,
			name: document_c.name,
			doc_type: document_c.doc_type,
			owner: ctx.user_id(),
			last_edit_user: ctx.user_id(),
			key: document_c.key,
		};

		let document_id = base::create::<Self, _>(ctx, mm, document).await?;

		Ok(document_id)
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filters: Option<Vec<DocumentFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<Document>> {
		base::list::<Self, _, _>(ctx, mm, filters, list_options).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		document_u: DocumentForUpdate,
	) -> Result<()> {
		let document = DocumentForUpdateInsert {
			name: document_u.name,
			last_edit_user: ctx.user_id(),
			archive_id: document_u.archive_id,
			separator_id: document_u.separator_id,
			key: document_u.key,
			doc_type: document_u.doc_type,
		};

		base::update::<Self, _>(ctx, mm, id, document).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}

	pub async fn get_documents_by_archive<E>(
		_ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
	) -> Result<Vec<E>>
	where
		E: DocumentBy,
	{
		let db = mm.db();

		let mut query = Query::select();
		query
			.from(Self::table_ref())
			.columns(E::field_column_refs())
			.and_where(Expr::col(DocumentIden::ArchiveId).eq(id));

		let (sql, values) = query.build_sqlx(PostgresQueryBuilder);
		let entities = sqlx::query_as_with::<_, E, _>(&sql, values)
			.fetch_all(db)
			.await?;

		Ok(entities)
	}
}
