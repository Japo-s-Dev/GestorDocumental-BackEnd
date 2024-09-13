use super::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
use crate::Result;
use lib_core::ctx::Ctx;
use lib_core::model::role::{Role, RoleBmc, RoleFilter, RoleForOp};
use lib_core::model::ModelManager;
pub async fn create_role(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<RoleForOp>,
) -> Result<Role> {
	let ParamsForCreate { data } = params;

	let id = RoleBmc::create(&ctx, &mm, data).await?;
	let role = RoleBmc::get(&ctx, &mm, id).await?;

	Ok(role)
}

pub async fn list_roles(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsList<RoleFilter>,
) -> Result<Vec<Role>> {
	let roles =
		RoleBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

	Ok(roles)
}

pub async fn get_role(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<Role> {
	let ParamsIded { id } = params;

	let role = RoleBmc::get(&ctx, &mm, id).await?;

	Ok(role)
}

pub async fn update_role(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForUpdate<RoleForOp>,
) -> Result<Role> {
	let ParamsForUpdate { id, data } = params;

	RoleBmc::update(&ctx, &mm, id, data).await?;

	let role = RoleBmc::get(&ctx, &mm, id).await?;

	Ok(role)
}

pub async fn delete_role(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<Role> {
	let ParamsIded { id } = params;

	let role = RoleBmc::get(&ctx, &mm, id).await?;
	RoleBmc::delete(&ctx, &mm, id).await?;

	Ok(role)
}
