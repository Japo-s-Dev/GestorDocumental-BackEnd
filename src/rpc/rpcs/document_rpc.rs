use std::time::Duration;

use crate::core::ctx::Ctx;
use crate::core::model::base::ListResult;
use crate::core::model::document::{
	Document, DocumentBmc, DocumentFilter, DocumentForCreate, DocumentForRequest,
	DocumentForUpdate,
};
use crate::core::model::separator::SeparatorBmc;
use crate::core::model::ModelManager;
use crate::rpc::config::rpc_config;
use crate::rpc::params::{ParamsForCreate, ParamsForUpdate, ParamsIded, ParamsList};
use crate::rpc::File;
use crate::rpc::Result;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::{primitives::ByteStream, Client};

pub async fn create_document(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsForCreate<DocumentForRequest>,
	file: File,
) -> Result<Document> {
	let s3_client = mm.bucket.clone();
	upload_to_s3(&s3_client, &file).await?;

	let ParamsForCreate { data } = params;

	let separator = SeparatorBmc::get(&ctx, &mm, data.separator_id).await?;

	let final_data = DocumentForCreate {
		archive_id: separator.archive_id,
		separator_id: data.separator_id,
		name: if let Some(file_name) = data.name {
			file_name
		} else {
			file.file_name.clone()
		},
		doc_type: file.content_type.clone(),
		key: file.key.clone(),
	};

	let document_id = DocumentBmc::create(&ctx, &mm, final_data).await?;
	let document = DocumentBmc::get(&ctx, &mm, document_id).await?;

	Ok(document)
}

pub async fn list_documents(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsList<DocumentFilter>,
) -> Result<ListResult<Document>> {
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
	params: ParamsForUpdate<DocumentForRequest>,
	file: Option<File>,
) -> Result<Document> {
	let s3_client = mm.bucket.clone();
	let ParamsForUpdate { id, data } = params;

	let document = DocumentBmc::get(&ctx, &mm, id).await?;

	let separator = SeparatorBmc::get(&ctx, &mm, data.separator_id).await?;

	let mut new_data = DocumentForUpdate {
		archive_id: separator.archive_id,
		separator_id: data.separator_id,
		name: document.name,
		doc_type: document.doc_type,
		key: document.key,
	};

	if let Some(mut file) = file {
		upload_to_s3(&s3_client, &mut file).await?;

		new_data.name = file.file_name.clone();
		new_data.doc_type = file.content_type.clone();
		new_data.key = file.key.clone();
	}

	DocumentBmc::update(&ctx, &mm, id, new_data).await?;

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

pub async fn get_doc_url(
	ctx: Ctx,
	mm: ModelManager,
	params: ParamsIded,
) -> Result<String> {
	let ParamsIded { id } = params;

	let document = DocumentBmc::get(&ctx, &mm, id).await?;

	let presigned_url = generate_presigned_url(&mm.bucket, &document.key).await?;

	Ok(presigned_url)
}

async fn generate_presigned_url(s3_client: &Client, key: &str) -> Result<String> {
	let expiration = Duration::from_secs(900)
		.try_into()
		.unwrap_or(Duration::from_secs(900));

	let presign_config = PresigningConfig::builder()
		.expires_in(expiration)
		.build()
		.unwrap();

	// Build the GetObject request
	let get_object_req = s3_client
		.get_object()
		.bucket(&rpc_config().AWS_BUCKET_NAME)
		.key(key)
		.presigned(presign_config)
		.await?;

	let presigned_url = get_object_req.uri().to_string();

	Ok(presigned_url)
}

async fn upload_to_s3(s3_client: &Client, file: &File) -> Result<()> {
	let res = s3_client
		.put_object()
		.bucket(&rpc_config().AWS_BUCKET_NAME)
		.content_type(file.content_type.clone())
		.content_length(file.bytes.len() as i64)
		.key(file.key.clone())
		.body(ByteStream::from(file.bytes.to_vec()))
		.send()
		.await;

	let _ = res.is_ok();

	Ok(())
}
