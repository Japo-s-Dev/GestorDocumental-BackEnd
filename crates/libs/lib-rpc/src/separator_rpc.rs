use super::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
use crate::Result;
use lib_core::ctx::Ctx;
use lib_core::model::separator::{
	Separator, SeparatorBmc, SeparatorFilter, SeparatorForCreate, SeparatorForUpdate,
};
use lib_core::model::ModelManager;

pub async fn create_separator(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<SeparatorForCreate>,
) -> Result<Separator> {
	let ParamsForCreate { data } = params;

	let id = SeparatorBmc::create(&ctx, &mm, data).await?;
	let separator = SeparatorBmc::get(&ctx, &mm, id).await?;

	Ok(separator)
}

pub async fn list_separators(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsList<SeparatorFilter>,
) -> Result<Vec<Separator>> {
	let separators =
		SeparatorBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

	Ok(separators)
}

pub async fn get_separator(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<Separator> {
	let ParamsIded { id } = params;

	let separator = SeparatorBmc::get(&ctx, &mm, id).await?;

	Ok(separator)
}

pub async fn update_separator(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForUpdate<SeparatorForUpdate>,
) -> Result<Separator> {
	let ParamsForUpdate { id, data } = params;

	SeparatorBmc::update(&ctx, &mm, id, data).await?;

	let separator = SeparatorBmc::get(&ctx, &mm, id).await?;

	Ok(separator)
}

pub async fn delete_separator(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<Separator> {
	let ParamsIded { id } = params;

	let separator = SeparatorBmc::get(&ctx, &mm, id).await?;
	SeparatorBmc::delete(&ctx, &mm, id).await?;

	Ok(separator)
}
