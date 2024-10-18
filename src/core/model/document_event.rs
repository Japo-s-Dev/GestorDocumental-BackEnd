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
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::postgres::PgRow;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

use super::base::ListResult;

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct DocumentEvent {
	pub id: i64,
	pub document_id: i64,
	pub user_id: i64,
	pub action: String,
	pub object: String,
	pub object_id: i64,
	#[serde_as(as = "Rfc3339")]
	pub timestamp: OffsetDateTime,
}

#[allow(dead_code)]
pub trait DocumentEventBy:
	HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send
{
}

impl DocumentEventBy for DocumentEvent {}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct DocumentEventFilter {
	id: Option<OpValsInt64>,

	document_id: Option<OpValsInt64>,
	user_id: Option<OpValsInt64>,
	action: Option<OpValsString>,
	object: Option<OpValsString>,
	object_id: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	timestamp: Option<OpValsValue>,
}

pub struct DocumentEventBmc;

impl DbBmc for DocumentEventBmc {
	const TABLE: &'static str = "document_event";
}

impl DocumentEventBmc {
	pub async fn get(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
	) -> Result<DocumentEvent> {
		base::get::<Self, _>(ctx, mm, id).await
	}
	/*
	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		comment_c: CommentForOp,
	) -> Result<i64> {
		let comment_data = ArchiveCommentForOpInsert {
			archive_id: comment_c.archive_id,
			text: comment_c.text,
			user_id: ctx.user_id(),
		};

		let comment_id = base::create::<Self, _>(ctx, mm, comment_data).await?;

		Ok(comment_id)
	}
	*/
	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filters: Option<Vec<DocumentEventFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<ListResult<DocumentEvent>> {
		base::list::<Self, _, _>(ctx, mm, filters, list_options).await
	}
	/*
	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		comment_u: ArchiveCommentForOp,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, comment_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
	*/
}
