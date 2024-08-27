use super::{ParamsForCreate, ParamsForUpdate, ParamsIded};
use crate::ctx::{self, Ctx};
use crate::model::datatype::{Datatype, DatatypeBmc, DatatypeForOp};
use crate::model::ModelManager;
use crate::web::Result;
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

pub async fn list_datatypes(ctx: Ctx, mm: ModelManager) -> Result<Vec<Datatype>> {
	let datatypes = DatatypeBmc::list(&ctx, &mm).await?;

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
