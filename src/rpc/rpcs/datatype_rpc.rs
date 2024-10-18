use crate::core::ctx::Ctx;
use crate::core::model::base::ListResult;
use crate::core::model::datatype::{
	Datatype, DatatypeBmc, DatatypeFilter, DatatypeForOp,
};
use crate::core::model::ModelManager;
use crate::rpc::params::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
use crate::rpc::Result;
pub async fn create_datatype(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<DatatypeForOp>,
) -> Result<Datatype> {
	let ParamsForCreate { data } = params;

	let id = DatatypeBmc::create(&ctx, &mm, data).await?;
	let datatype = DatatypeBmc::get(&ctx, &mm, id).await?;

	Ok(datatype)
}

pub async fn list_datatypes(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsList<DatatypeFilter>,
) -> Result<ListResult<Datatype>> {
	let datatypes =
		DatatypeBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

	Ok(datatypes)
}

pub async fn get_datatype(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<Datatype> {
	let ParamsIded { id } = params;

	let datatypes = DatatypeBmc::get(&ctx, &mm, id).await?;

	Ok(datatypes)
}

pub async fn update_datatype(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForUpdate<DatatypeForOp>,
) -> Result<Datatype> {
	let ParamsForUpdate { id, data } = params;

	DatatypeBmc::update(&ctx, &mm, id, data).await?;

	let datatype = DatatypeBmc::get(&ctx, &mm, id).await?;

	Ok(datatype)
}

pub async fn delete_datatype(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<Datatype> {
	let ParamsIded { id } = params;

	let datatype = DatatypeBmc::get(&ctx, &mm, id).await?;
	DatatypeBmc::delete(&ctx, &mm, id).await?;

	Ok(datatype)
}
