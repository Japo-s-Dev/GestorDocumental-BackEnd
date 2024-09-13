use crate::Result;
use lib_core::ctx::Ctx;
use lib_core::model::user::{
	User, UserBmc, UserFilter, UserForCreate, UserForUpdate,
};
use lib_core::model::ModelManager;

use super::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};

pub async fn create_user(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<UserForCreate>,
) -> Result<User> {
	let ParamsForCreate { data } = params;

	let id = UserBmc::create(&ctx, &mm, data).await?;
	let user = UserBmc::get(&ctx, &mm, id).await?;

	Ok(user)
}

pub async fn list_users(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsList<UserFilter>,
) -> Result<Vec<User>> {
	let users =
		UserBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

	Ok(users)
}

pub async fn update_user(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForUpdate<UserForUpdate>,
) -> Result<User> {
	let ParamsForUpdate { id, data } = params;

	UserBmc::update(&ctx, &mm, id, data).await?;

	let user = UserBmc::get(&ctx, &mm, id).await?;

	Ok(user)
}

pub async fn delete_user(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<User> {
	let ParamsIded { id } = params;

	let user = UserBmc::get(&ctx, &mm, id).await?;
	UserBmc::delete(&ctx, &mm, id).await?;

	Ok(user)
}

pub async fn get_user(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<User> {
	let ParamsIded { id } = params;

	let user = UserBmc::get(&ctx, &mm, id).await?;

	Ok(user)
}
