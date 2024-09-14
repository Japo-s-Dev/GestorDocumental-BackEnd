use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::Result;
use modql::field::{Fields, HasFields};
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsString};
use serde::{Deserialize, Serialize};
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Document {
	pub id: i64,
	pub archive_id: i64,
	pub name: String,
	pub doc_type: String,
	pub creation_date: i64,
	pub modified_date: i64,
	pub owner: i64,
	pub last_edit_user: i64,
	pub url: String,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct DocumentForCreate {
	pub archive_id: i64,
	pub name: String,
	pub doc_type: String,
	pub url: String,
}

#[derive(Clone, Fields, FromRow, Debug, Deserialize)]
pub struct DocumentForUpdate {
	pub name: String,
	pub doc_type: String,
	pub url: String,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct DocumentForCreateInsert {
	pub archive_id: i64,
	pub name: String,
	pub doc_type: String,
	pub modified_date: i64,
	pub owner: i64,
	pub last_edit_user: i64,
	pub url: String,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct DocumentForUpdateInsert {
	pub name: String,
	pub modified_date: i64,
	pub last_edit_user: i64,
}

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
	name: Option<OpValsString>,
	doc_type: Option<OpValsString>,
	creation_date: Option<OpValsInt64>,
	modified_date: Option<OpValsInt64>,
	owner: Option<OpValsInt64>,
	last_edit_user: Option<OpValsInt64>,
	url: Option<OpValsString>,
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
			name: document_c.name,
			doc_type: document_c.doc_type,
			modified_date: OffsetDateTime::unix_timestamp(OffsetDateTime::now_utc()),
			owner: ctx.user_id(),
			last_edit_user: ctx.user_id(),
			url: document_c.url,
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
			modified_date: OffsetDateTime::unix_timestamp(OffsetDateTime::now_utc()),
			last_edit_user: ctx.user_id(),
		};

		base::update::<Self, _>(ctx, mm, id, document).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}