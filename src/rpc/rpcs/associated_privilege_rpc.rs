use crate::core::ctx::Ctx;
use crate::core::model::associated_privilege::{
	AssociatedPrivilege, AssociatedPrivilegeBmc, AssociatedPrivilegeFilter,
	AssociatedPrivilegeForOp, AssociatedPrivilegeForSearchByRole,
	AssociatedPrivilegesForOp,
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

#[allow(unused)]
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

pub async fn enable_associated_privilege(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<AssociatedPrivilegesForOp>,
) -> Result<Vec<AssociatedPrivilege>> {
	let ParamsForCreate { data } = params;
	let mut enabled_privileges = Vec::new();

	for id in data.ids {
		AssociatedPrivilegeBmc::enable(&ctx, &mm, &data.role_name, id).await?;
		let association = AssociatedPrivilegeBmc::get_on_role_and_id(
			&ctx,
			&mm,
			&data.role_name,
			id,
		)
		.await?;
		enabled_privileges.push(association);
	}

	Ok(enabled_privileges)
}

pub async fn disable_associated_privilege(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<AssociatedPrivilegesForOp>,
) -> Result<Vec<AssociatedPrivilege>> {
	let ParamsForCreate { data } = params;
	let mut disabled_privileges = Vec::new();

	for id in data.ids {
		AssociatedPrivilegeBmc::disable(&ctx, &mm, &data.role_name, id).await?;
		let association = AssociatedPrivilegeBmc::get_on_role_and_id(
			&ctx,
			&mm,
			&data.role_name,
			id,
		)
		.await?;
		disabled_privileges.push(association);
	}

	Ok(disabled_privileges)
}

pub async fn list_enabled_privileges(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<AssociatedPrivilegeForSearchByRole>,
) -> Result<Vec<AssociatedPrivilege>> {
	let ParamsForCreate { data } = params;

	let permissions =
		AssociatedPrivilegeBmc::list_enabled_permissions(&ctx, &mm, &data.role_name)
			.await?;

	Ok(permissions)
}
