use crate::core::ctx::Ctx;
use crate::core::model::base::ListResult;
use crate::core::model::document_comment::{
	DocumentComment, DocumentCommentBmc, DocumentCommentFilter, DocumentCommentForOp,
};
use crate::core::model::ModelManager;
use crate::rpc::params::{ParamsForCreate, ParamsIded, ParamsList};
use crate::rpc::Result;

pub async fn create_document_comment(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<DocumentCommentForOp>,
) -> Result<DocumentComment> {
	let ParamsForCreate { data } = params;

	let id = DocumentCommentBmc::create(&ctx, &mm, data).await?;
	let comment = DocumentCommentBmc::get(&ctx, &mm, id).await?;

	Ok(comment)
}

pub async fn list_document_comments(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsList<DocumentCommentFilter>,
) -> Result<ListResult<DocumentComment>> {
	let comments =
		DocumentCommentBmc::list(&ctx, &mm, params.filters, params.list_options)
			.await?;

	Ok(comments)
}

pub async fn get_document_comment(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<DocumentComment> {
	let ParamsIded { id } = params;

	let comment = DocumentCommentBmc::get(&ctx, &mm, id).await?;

	Ok(comment)
}
