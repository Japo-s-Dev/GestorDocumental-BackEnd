use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlb::{Fields, HasFields};
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use time::OffsetDateTime;

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Document {
	pub id: i64,
	pub archive_id: i64,
	pub name: String,
	pub doc_type: String,
	pub creation_date: OffsetDateTime,
	pub modified_date: OffsetDateTime,
	pub owner: i64,
	pub last_edit_user: i64,
	pub url: String,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct DocumentForCreate {
	pub archive_id: i64,
	pub name: String,
	pub doc_type: String,
	pub owner: i64,
	pub last_edit_user: i64,
	pub url: String,
}

#[derive(Clone, Fields, FromRow, Debug, Deserialize)]
pub struct DocumentForUpdate {
	pub name: String,
	pub last_edit_user: i64,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct DocumentForCreateInsert {
	pub archive_id: i64,
	pub name: String,
	pub doc_type: String,
	pub modified_date: OffsetDateTime,
	pub owner: i64,
	pub last_edit_user: i64,
	pub url: String,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct DocumentForUpdateInsert {
	pub name: String,
	pub modified_date: OffsetDateTime,
	pub last_edit_user: i64,
}

pub trait DocumentBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl DocumentBy for Document {}
impl DocumentBy for DocumentForUpdate {}
impl DocumentBy for DocumentForCreate {}
impl DocumentBy for DocumentForUpdateInsert {}
impl DocumentBy for DocumentForCreateInsert {}

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
			modified_date: OffsetDateTime::now_utc(),
			owner: document_c.owner,
			last_edit_user: document_c.last_edit_user,
			url: document_c.url,
		};

		let document_id = base::create::<Self, _>(ctx, mm, document).await?;

		Ok(document_id)
	}

	pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Document>> {
		base::list::<Self, _>(ctx, mm).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		document_u: DocumentForUpdate,
	) -> Result<()> {
		let document = DocumentForUpdateInsert {
			name: document_u.name,
			modified_date: OffsetDateTime::now_utc(),
			last_edit_user: document_u.last_edit_user,
		};

		base::update::<Self, _>(ctx, mm, id, document).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
