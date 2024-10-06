use crate::core::ctx::Ctx;
use crate::core::model::archive_comment::{
	ArchiveComment, ArchiveCommentBmc, ArchiveCommentFilter, ArchiveCommentForOp,
};
use crate::core::model::ModelManager;
use crate::rpc::params::{ParamsForCreate, ParamsIded, ParamsList};
use crate::rpc::Result;

pub async fn create_comment(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<ArchiveCommentForOp>,
) -> Result<ArchiveComment> {
	let ParamsForCreate { data } = params;

	let id = ArchiveCommentBmc::create(&ctx, &mm, data).await?;
	let comment = ArchiveCommentBmc::get(&ctx, &mm, id).await?;

	Ok(comment)
}

pub async fn list_comments(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsList<ArchiveCommentFilter>,
) -> Result<Vec<ArchiveComment>> {
	let comments =
		ArchiveCommentBmc::list(&ctx, &mm, params.filters, params.list_options)
			.await?;

	Ok(comments)
}

pub async fn get_comment(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<ArchiveComment> {
	let ParamsIded { id } = params;

	let comment = ArchiveCommentBmc::get(&ctx, &mm, id).await?;

	Ok(comment)
}
