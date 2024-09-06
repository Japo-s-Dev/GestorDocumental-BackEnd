use super::{ParamsForCreate, ParamsForUpdate, ParamsIded};
use crate::ctx::{self, Ctx};
use crate::model::document::{
	Document, DocumentBmc, DocumentForCreate, DocumentForUpdate,
};
use crate::model::ModelManager;
use crate::web::Result;
pub async fn create_document(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<DocumentForCreate>,
) -> Result<Document> {
	let ParamsForCreate { data } = params;

	let id = DocumentBmc::create(&ctx, &mm, data).await?;
	let document = DocumentBmc::get(&ctx, &mm, id).await?;

	Ok(document)
}

pub async fn list_documents(ctx: Ctx, mm: ModelManager) -> Result<Vec<Document>> {
	let documents = DocumentBmc::list(&ctx, &mm).await?;

	Ok(documents)
}

pub async fn get_document(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<Document> {
	let ParamsIded { id } = params;

	let document = DocumentBmc::get(&ctx, &mm, id).await?;

	Ok(document)
}

pub async fn update_document(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForUpdate<DocumentForUpdate>,
) -> Result<Document> {
	let ParamsForUpdate { id, data } = params;

	DocumentBmc::update(&ctx, &mm, id, data).await?;

	let document = DocumentBmc::get(&ctx, &mm, id).await?;

	Ok(document)
}

pub async fn delete_document(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<Document> {
	let ParamsIded { id } = params;

	let document = DocumentBmc::get(&ctx, &mm, id).await?;
	DocumentBmc::delete(&ctx, &mm, id).await?;

	Ok(document)
}
