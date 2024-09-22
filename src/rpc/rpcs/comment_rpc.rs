use crate::core::ctx::Ctx;
use crate::core::model::comment::{
	Comment, CommentBmc, CommentFilter, CommentForOp,
};
use crate::core::model::ModelManager;
use crate::rpc::params::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
use crate::rpc::Result;

pub async fn create_comment(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<CommentForOp>,
) -> Result<Comment> {
	let ParamsForCreate { data } = params;

	let id = CommentBmc::create(&ctx, &mm, data).await?;
	let comment = CommentBmc::get(&ctx, &mm, id).await?;

	Ok(comment)
}

pub async fn list_comments(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsList<CommentFilter>,
) -> Result<Vec<Comment>> {
	let archives =
		CommentBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

	Ok(archives)
}

pub async fn get_comment(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<Comment> {
	let ParamsIded { id } = params;

	let archive = CommentBmc::get(&ctx, &mm, id).await?;

	Ok(archive)
}
