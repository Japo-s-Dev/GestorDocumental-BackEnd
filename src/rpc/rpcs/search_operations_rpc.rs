use crate::core::ctx::Ctx;
use crate::core::model::index::{IndexBmc, IndexWithDatatype};
use crate::core::model::ModelManager;
use crate::rpc::params::ParamsIded;
use crate::rpc::Result;

pub async fn get_project_fields(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<Vec<IndexWithDatatype>> {
	let ParamsIded { id } = params;

	let indexes = IndexBmc::get_indexes_by_project(&ctx, &mm, id).await?;

	Ok(indexes)
}
