use super::{ParamsForCreate, ParamsForUpdate, ParamsIded};
use crate::ctx::{self, Ctx};
use crate::model::role::{Role, RoleBmc, RoleForOp};
use crate::model::ModelManager;
use crate::web::Result;
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

pub async fn list_roles(ctx: Ctx, mm: ModelManager) -> Result<Vec<Role>> {
	let roles = RoleBmc::list(&ctx, &mm).await?;

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
