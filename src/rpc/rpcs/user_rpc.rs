use crate::core::ctx::Ctx;
use crate::core::model::base::ListResult;
use crate::core::model::user::{
	User, UserBmc, UserFilter, UserForCreate, UserForUpdate, UserForUpdatePwd,
};
use crate::core::model::ModelManager;
use crate::rpc::params::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
use crate::rpc::Result;

const READ_USERS: i64 = 1;
const WRITE_USERS: i64 = 2;

pub async fn create_user(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<UserForCreate>,
) -> Result<User> {
	//check_permission(&ctx, WRITE_USERS)?;

	let ParamsForCreate { data } = params;

	let id = UserBmc::create(&ctx, &mm, data).await?;
	let user = UserBmc::get(&ctx, &mm, id).await?;

	Ok(user)
}

pub async fn list_users(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsList<UserFilter>,
) -> Result<ListResult<User>> {
	//check_permission(&ctx, READ_USERS)?;

	let users =
		UserBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

	Ok(users)
}

pub async fn update_user(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForUpdate<UserForUpdate>,
) -> Result<User> {
	//check_permission(&ctx, WRITE_USERS)?;

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
	//check_permission(&ctx, WRITE_USERS)?;

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
	//check_permission(&ctx, READ_USERS)?;

	let ParamsIded { id } = params;

	let user = UserBmc::get(&ctx, &mm, id).await?;

	Ok(user)
}

pub async fn update_pwd(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForUpdate<UserForUpdatePwd>,
) -> Result<User> {
	//check_permission(&ctx, WRITE_USERS)?;

	let ParamsForUpdate { id, data } = params;

	UserBmc::update_pwd(&ctx, &mm, id, &data.pwd_clear).await?;

	let user = UserBmc::get(&ctx, &mm, id).await?;

	Ok(user)
}
