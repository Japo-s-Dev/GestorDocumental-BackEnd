use super::{ParamsForCreate, ParamsForUpdate, ParamsIded};
use crate::ctx::{self, Ctx};
use crate::model::separator::{
	self, Separator, SeparatorBmc, SeparatorForCreate, SeparatorForUpdate,
};
use crate::model::ModelManager;
use crate::web::Result;

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

pub async fn list_separators(ctx: Ctx, mm: ModelManager) -> Result<Vec<Separator>> {
	let separators = SeparatorBmc::list(&ctx, &mm).await?;

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
