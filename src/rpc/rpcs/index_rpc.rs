use crate::core::ctx::Ctx;
use crate::core::model::index::{
	Index, IndexBmc, IndexFilter, IndexForCreate, IndexForUpdate,
};
use crate::core::model::ModelManager;
use crate::rpc::params::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
use crate::rpc::Result;

pub async fn create_index(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<IndexForCreate>,
) -> Result<Index> {
	let ParamsForCreate { data } = params;

	let id = IndexBmc::create(&ctx, &mm, data).await?;
	let index = IndexBmc::get(&ctx, &mm, id).await?;

	Ok(index)
}

pub async fn list_indexes(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsList<IndexFilter>,
) -> Result<Vec<Index>> {
	let indexes =
		IndexBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

	Ok(indexes)
}

pub async fn get_index(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<Index> {
	let ParamsIded { id } = params;

	let index = IndexBmc::get(&ctx, &mm, id).await?;

	Ok(index)
}

pub async fn update_index(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForUpdate<IndexForUpdate>,
) -> Result<Index> {
	let ParamsForUpdate { id, data } = params;

	IndexBmc::update(&ctx, &mm, id, data).await?;

	let index = IndexBmc::get(&ctx, &mm, id).await?;

	Ok(index)
}

pub async fn delete_index(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<Index> {
	let ParamsIded { id } = params;

	let index = IndexBmc::get(&ctx, &mm, id).await?;
	IndexBmc::delete(&ctx, &mm, id).await?;

	Ok(index)
}
