use super::{ParamsForCreate, ParamsForUpdate, ParamsIded};
use crate::ctx::{self, Ctx};
use crate::model::value::{
	Value, ValueBmc, ValueBy, ValueForCreate, ValueForUpdate,
};
use crate::model::ModelManager;
use crate::web::Result;

pub async fn create_value(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<ValueForCreate>,
) -> Result<Value> {
	let ParamsForCreate { data } = params;

	let id = ValueBmc::create(&ctx, &mm, data).await?;
	let value = ValueBmc::get(&ctx, &mm, id).await?;

	Ok(value)
}

pub async fn list_values(ctx: Ctx, mm: ModelManager) -> Result<Vec<Value>> {
	let values = ValueBmc::list(&ctx, &mm).await?;

	Ok(values)
}

pub async fn get_value(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<Value> {
	let ParamsIded { id } = params;

	let value = ValueBmc::get(&ctx, &mm, id).await?;

	Ok(value)
}

pub async fn update_value(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForUpdate<ValueForUpdate>,
) -> Result<Value> {
	let ParamsForUpdate { id, data } = params;

	ValueBmc::update(&ctx, &mm, id, data).await?;

	let value = ValueBmc::get(&ctx, &mm, id).await?;

	Ok(value)
}

pub async fn delete_value(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<Value> {
	let ParamsIded { id } = params;

	let value = ValueBmc::get(&ctx, &mm, id).await?;
	ValueBmc::delete(&ctx, &mm, id).await?;

	Ok(value)
}
