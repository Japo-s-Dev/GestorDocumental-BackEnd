use crate::core::ctx::Ctx;
use crate::core::model::structure_privilege::{
	StructurePrivilege, StructurePrivilegeBmc,
};
use crate::core::model::ModelManager;
use crate::rpc::params::ParamsIded;
use crate::rpc::Result;

pub async fn list_structure_privileges_by_user_id(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<Vec<StructurePrivilege>> {
	let ParamsIded { id } = params;

	let associations = StructurePrivilegeBmc::list_by_user_id(&ctx, &mm, id).await?;

	Ok(associations)
}

pub async fn get_structure_privilege(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<StructurePrivilege> {
	let ParamsIded { id } = params;

	let association = StructurePrivilegeBmc::get(&ctx, &mm, id).await?;

	Ok(association)
}

pub async fn enable_structure_privilege(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<StructurePrivilege> {
	let ParamsIded { id } = params;

	StructurePrivilegeBmc::enable(&ctx, &mm, id).await?;
	let association = StructurePrivilegeBmc::get(&ctx, &mm, id).await?;

	Ok(association)
}

pub async fn disable_structure_privilege(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<StructurePrivilege> {
	let ParamsIded { id } = params;

	StructurePrivilegeBmc::disable(&ctx, &mm, id).await?;
	let association = StructurePrivilegeBmc::get(&ctx, &mm, id).await?;

	Ok(association)
}
