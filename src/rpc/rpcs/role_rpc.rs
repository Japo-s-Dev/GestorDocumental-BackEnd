use crate::core::ctx::Ctx;
use crate::core::model::base::ListResult;
use crate::core::model::role::{Role, RoleBmc, RoleFilter, RoleForOp};
use crate::core::model::ModelManager;
use crate::rpc::params::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
use crate::rpc::Result;
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
) -> Result<ListResult<Role>> {
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
