use crate::core::ctx::Ctx;
use crate::core::model::base::{self, DbBmc};
use crate::core::model::modql_utils::time_to_sea_value;
use crate::core::model::ModelManager;
use crate::core::model::Result;
use crate::utils::time::Rfc3339;
use modql::field::{Fields, HasFields};
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsValue};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::postgres::PgRow;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

use super::base::ListResult;

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct DocumentComment {
	pub id: i64,
	pub document_id: i64,
	pub text: String,
	pub user_id: i64,
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct DocumentCommentForOp {
	pub text: String,
	pub document_id: i64,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct DocumentCommentForOpInsert {
	pub text: String,
	pub document_id: i64,
	pub user_id: i64,
}

#[allow(dead_code)]
pub trait DocumentCommentBy:
	HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send
{
}

impl DocumentCommentBy for DocumentComment {}
impl DocumentCommentBy for DocumentCommentForOp {}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct DocumentCommentFilter {
	id: Option<OpValsInt64>,

	document_id: Option<OpValsInt64>,
	user_id: Option<OpValsInt64>,
	cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	ctime: Option<OpValsValue>,
	mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	mtime: Option<OpValsValue>,
}

pub struct DocumentCommentBmc;

impl DbBmc for DocumentCommentBmc {
	const TABLE: &'static str = "document_comment";
}

impl DocumentCommentBmc {
	pub async fn get(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
	) -> Result<DocumentComment> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		comment_c: DocumentCommentForOp,
	) -> Result<i64> {
		let comment_data = DocumentCommentForOpInsert {
			document_id: comment_c.document_id,
			text: comment_c.text,
			user_id: ctx.user_id(),
		};

		let comment_id = base::create::<Self, _>(ctx, mm, comment_data).await?;

		Ok(comment_id)
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filters: Option<Vec<DocumentCommentFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<ListResult<DocumentComment>> {
		base::list::<Self, _, _>(ctx, mm, filters, list_options).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		comment_u: DocumentCommentForOp,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, comment_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
