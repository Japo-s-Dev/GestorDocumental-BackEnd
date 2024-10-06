use crate::core::ctx::Ctx;
use crate::core::model::structure::{
	Structure, StructureBmc, StructureFilter, StructureForOp,
};
use crate::core::model::ModelManager;
use crate::rpc::params::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
use crate::rpc::Result;
pub async fn create_structure(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<StructureForOp>,
) -> Result<Structure> {
	let ParamsForCreate { data } = params;

	let id = StructureBmc::create(&ctx, &mm, data).await?;
	let structure = StructureBmc::get(&ctx, &mm, id).await?;

	Ok(structure)
}

pub async fn list_structures(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsList<StructureFilter>,
) -> Result<Vec<Structure>> {
	let structures =
		StructureBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

	Ok(structures)
}

pub async fn get_structure(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<Structure> {
	let ParamsIded { id } = params;

	let structure = StructureBmc::get(&ctx, &mm, id).await?;

	Ok(structure)
}

pub async fn update_structure(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForUpdate<StructureForOp>,
) -> Result<Structure> {
	let ParamsForUpdate { id, data } = params;

	StructureBmc::update(&ctx, &mm, id, data).await?;

	let structure = StructureBmc::get(&ctx, &mm, id).await?;

	Ok(structure)
}

pub async fn delete_structure(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<Structure> {
	let ParamsIded { id } = params;

	let structure = StructureBmc::get(&ctx, &mm, id).await?;
	StructureBmc::delete(&ctx, &mm, id).await?;

	Ok(structure)
}
