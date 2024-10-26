use crate::core::ctx::Ctx;
use crate::core::model::associated_privilege::{
	AssociatedPrivilege, AssociatedPrivilegeBmc, AssociatedPrivilegeFilter,
	AssociatedPrivilegeForOp, AssociatedPrivilegeForSearchByRole,
};
use crate::core::model::base::ListResult;
use crate::core::model::ModelManager;
use crate::rpc::params::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
use crate::rpc::Result;

pub async fn create_associated_privilege(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<AssociatedPrivilegeForOp>,
) -> Result<AssociatedPrivilege> {
	let ParamsForCreate { data } = params;

	let id = AssociatedPrivilegeBmc::create(&ctx, &mm, data).await?;
	let association = AssociatedPrivilegeBmc::get(&ctx, &mm, id).await?;

	Ok(association)
}

pub async fn list_associated_privileges(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsList<AssociatedPrivilegeFilter>,
) -> Result<ListResult<AssociatedPrivilege>> {
	let associations =
		AssociatedPrivilegeBmc::list(&ctx, &mm, params.filters, params.list_options)
			.await?;

	Ok(associations)
}

pub async fn list_associated_privileges_by_role_name(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<AssociatedPrivilegeForSearchByRole>,
) -> Result<Vec<AssociatedPrivilege>> {
	let ParamsForCreate { data } = params;

	let associations =
		AssociatedPrivilegeBmc::list_by_role_name(&ctx, &mm, &data.role_name)
			.await?;

	Ok(associations)
}

pub async fn get_associated_privilege(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<AssociatedPrivilege> {
	let ParamsIded { id } = params;

	let association = AssociatedPrivilegeBmc::get(&ctx, &mm, id).await?;

	Ok(association)
}

pub async fn update_associated_privilege(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForUpdate<AssociatedPrivilegeForOp>,
) -> Result<AssociatedPrivilege> {
	let ParamsForUpdate { id, data } = params;

	AssociatedPrivilegeBmc::update(&ctx, &mm, id, data).await?;

	let association = AssociatedPrivilegeBmc::get(&ctx, &mm, id).await?;

	Ok(association)
}

pub async fn delete_associated_privilege(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<AssociatedPrivilege> {
	let ParamsIded { id } = params;

	let association = AssociatedPrivilegeBmc::get(&ctx, &mm, id).await?;
	AssociatedPrivilegeBmc::delete(&ctx, &mm, id).await?;

	Ok(association)
}
