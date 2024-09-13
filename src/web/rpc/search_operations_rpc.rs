use super::{ParamsForCreate, ParamsForUpdate, ParamsIded};
use crate::ctx::{self, Ctx};
use crate::model::index::{IndexBmc, IndexWithDatatype};
use crate::model::ModelManager;
use crate::web::Result;

pub async fn get_project_fields(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<Vec<IndexWithDatatype>> {
	let ParamsIded { id } = params;

	let indexes = IndexBmc::get_indexes_by_project(&ctx, &mm, id).await?;

	Ok(indexes)
}
