use super::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
use crate::config::rpc_config;
use crate::File;
use crate::Result;
use aws_sdk_s3::{primitives::ByteStream, Client};
use lib_core::ctx::Ctx;
use lib_core::model::document::{
	Document, DocumentBmc, DocumentFilter, DocumentForCreate, DocumentForUpdate,
};
use lib_core::model::ModelManager;

pub async fn create_document(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<DocumentForCreate>,
) -> Result<Document> {
	let ParamsForCreate { data } = params;

	let document_id = DocumentBmc::create(&ctx, &mm, data).await?;
	let document = DocumentBmc::get(&ctx, &mm, document_id).await?;

	Ok(document)
}

pub async fn list_documents(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsList<DocumentFilter>,
) -> Result<Vec<Document>> {
	let documents =
		DocumentBmc::list(&ctx, &mm, params.filters, params.list_options).await?;

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

async fn upload_to_s3(s3_client: &Client, file: &mut File) -> Result<()> {
	let res = s3_client
		.put_object()
		.bucket(&rpc_config().AWS_BUCKET_NAME)
		.content_type(file.content_type.clone())
		.content_length(file.bytes.len() as i64)
		.key(file.key.clone())
		.body(ByteStream::from(file.bytes.to_vec()))
		.send()
		.await;

	file.successful = res.is_ok();

	Ok(())
}
