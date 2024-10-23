use crate::core::ctx::Ctx;
use crate::core::model::base::ListResult;
use crate::core::model::privilege::{Privilege, PrivilegeBmc, PrivilegeFilter};
use crate::core::model::ModelManager;
use crate::rpc::params::{ParamsIded, ParamsList};
use crate::rpc::Result;

pub async fn list_privileges(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsList<PrivilegeFilter>,
) -> Result<ListResult<Privilege>> {
	let indexes =
		PrivilegeBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

	Ok(indexes)
}

pub async fn get_privilege(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<Privilege> {
	let ParamsIded { id } = params;

	let index = PrivilegeBmc::get(&ctx, &mm, id).await?;

	Ok(index)
}
