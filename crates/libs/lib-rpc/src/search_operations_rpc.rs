use super::{ParamsForCreate, ParamsForUpdate, ParamsIded};
use crate::Result;
use lib_core::ctx::Ctx;
use lib_core::model::index::{IndexBmc, IndexWithDatatype};
use lib_core::model::ModelManager;

pub async fn get_project_fields(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<Vec<IndexWithDatatype>> {
	let ParamsIded { id } = params;

	let indexes = IndexBmc::get_indexes_by_project(&ctx, &mm, id).await?;

	Ok(indexes)
}
