mod archive_rpc;
mod datatype_rpc;
mod document_rpc;
mod index_rpc;
mod project_rpc;
mod role_rpc;
mod search_operations_rpc;
mod separator_rpc;
mod user_rpc;
mod value_rpc;

use std::{collections::HashSet, fmt::format};

use crate::{
	config,
	ctx::{self, Ctx},
	model::{
		document::{DocumentForCreate, DocumentForUpdate},
		ModelManager,
	},
	web::{Error, Result},
};
use archive_rpc::*;
use aws_sdk_s3::{primitives::ByteStream, Client};
use axum::{
	body::Bytes,
	extract::{Multipart, State},
	response::{IntoResponse, Response},
	routing::post,
	Json, Router,
};
use datatype_rpc::*;
use document_rpc::*;
use index_rpc::*;
use project_rpc::*;
use role_rpc::*;
use search_operations_rpc::*;
use separator_rpc::*;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, json, to_value, Value};
use tracing::debug;
use user_rpc::*;
use value_rpc::*;

#[derive(Deserialize)]
struct RpcRequest {
	id: Option<Value>,
	method: String,
	params: Option<Value>,
}

#[derive(Deserialize)]
pub struct ParamsForCreate<D> {
	data: D,
}

#[derive(Deserialize)]
pub struct ParamsForUpdate<D> {
	id: i64,
	data: D,
}

#[derive(Deserialize)]
pub struct ParamsIded {
	id: i64,
}

#[derive(Serialize)]
struct File {
	key: String,
	successful: bool,
	url: String,
	file_name: String,
	content_type: String,
	#[serde(skip_serializing)]
	bytes: Bytes,
}

fn get_file_insertion_methods() -> HashSet<&'static str> {
	let mut methods = HashSet::new();
	methods.insert("create_document");
	methods.insert("update_document");
	methods
}

async fn upload_to_s3(s3_client: &Client, file: &mut File) -> Result<()> {
	let res = s3_client
		.put_object()
		.bucket(&config().AWS_BUCKET_NAME)
		.content_type(file.content_type.clone())
		.content_length(file.bytes.len() as i64)
		.key(file.key.clone())
		.body(ByteStream::from(file.bytes.to_vec()))
		.send()
		.await;

	file.successful = res.is_ok();

	Ok(())
}

pub fn routes(mm: ModelManager) -> Router {
	Router::new()
		.route("/rpc", post(rpc_handler))
		.with_state(mm)
}

async fn rpc_handler(
	State(mm): State<ModelManager>,
	ctx: Ctx,
	mut multipart: Multipart,
) -> Response {
	// Initialize variables for json and file extraction
	let mut json_data: Option<String> = None;

	let mut file_data: Option<File> = None;

	// Process the multipart fields
	while let Some(field) = multipart.next_field().await.unwrap() {
		let name = field.name().unwrap().to_string();

		if name == "json" {
			json_data = Some(field.text().await.unwrap());
		} else if name == "file" {
			if file_data.is_some() {
				return Error::RpcInvalidMethod {
					rpc_method: "Too many files".to_string(),
					message: "Only one file is allowed.".to_string(),
				}
				.into_response();
			}

			let name = field.file_name().unwrap_or_default().to_owned();
			let content_type = field.content_type().unwrap_or_default().to_owned();
			let key = uuid::Uuid::new_v4().to_string();

			let bytes = field.bytes().await.unwrap();

			let file_name = format!("{}-{}", key, name);

			let url = format!(
				"https://{}.s3.amazonaws.com/{}",
				&config().AWS_BUCKET_NAME,
				key
			);

			file_data = Some(File {
				file_name,
				content_type,
				key,
				url,
				bytes,
				successful: false,
			})
		}
	}

	let rpc_req: RpcRequest = match json_data {
		Some(data) => match serde_json::from_str(&data) {
			Ok(req) => req,
			Err(_) => {
				return Error::RpcFailJsonParams {
					rpc_method: "Invalid JSON".to_string(),
				}
				.into_response();
			}
		},
		None => {
			return Error::RpcMisingParams {
				rpc_method: "Missing Params".to_string(),
			}
			.into_response();
		}
	};

	let allowed_methods = get_file_insertion_methods();

	let s3_client = mm.bucket.clone();

	// Check if file upload is allowed for this method
	if let Some(mut file) = file_data {
		if !allowed_methods.contains(rpc_req.method.as_str()) {
			return Error::RpcInvalidMethod {
				rpc_method: rpc_req.method.clone(),
				message: "This method does not allow file insertion.".to_string(),
			}
			.into_response();
		}

		// Upload the file to S3
		let s3_client = mm.bucket.clone();
		if let Err(e) = upload_to_s3(&s3_client, &mut file).await {
			return Error::RpcInvalidMethod {
				rpc_method: rpc_req.method.clone(),
				message: format!("File upload failed: {}", e),
			}
			.into_response();
		}

		// Update the params after file upload
		if rpc_req.method == "create_document" {
			let mut params: ParamsForCreate<DocumentForCreate> =
				match serde_json::from_value(rpc_req.params.unwrap()) {
					Ok(p) => p,
					Err(_) => {
						return Error::RpcFailJsonParams {
							rpc_method: "Invalid Params for Create".to_string(),
						}
						.into_response();
					}
				};

			// Set the `url`, `doc_type`, and `file_name`
			params.data.url = file.url.clone();
			params.data.doc_type = file.content_type.clone();
			params.data.name = file.file_name.clone();

			// Call the create_document function
			let document = create_document(ctx, mm, params).await.unwrap();
			return Json(document).into_response();
		} else if rpc_req.method == "update_document" {
			let mut params: ParamsForUpdate<DocumentForUpdate> =
				match serde_json::from_value(rpc_req.params.unwrap()) {
					Ok(p) => p,
					Err(_) => {
						return Error::RpcFailJsonParams {
							rpc_method: "Invalid Params for Update".to_string(),
						}
						.into_response();
					}
				};

			// Set the `url`, `doc_type`, and `file_name`
			params.data.url = file.url.clone();
			params.data.doc_type = file.content_type.clone();
			params.data.name = file.file_name.clone();

			// Call the update_document function
			let document = update_document(ctx, mm, params).await.unwrap();
			return Json(document).into_response();
		}
	}

	let rpc_info = RpcInfo {
		id: rpc_req.id.clone(),
		method: rpc_req.method.clone(),
	};

	// Call the original handler
	let mut res = _rpc_handler(ctx, mm, rpc_req).await.into_response();
	res.extensions_mut().insert(rpc_info);
	res
}

#[derive(Debug)]
pub struct RpcInfo {
	pub id: Option<Value>,
	pub method: String,
}

macro_rules! exec_rpc_fn {
	// With Params
	($rpc_fn:expr, $ctx:expr, $mm:expr, $rpc_params: expr) => {{
		let rpc_fn_name = stringify!($rpc_fn);

		let params = $rpc_params.ok_or(Error::RpcMisingParams {
			rpc_method: rpc_fn_name.to_string(),
		})?;

		let params = from_value(params).map_err(|_| Error::RpcFailJsonParams {
			rpc_method: rpc_fn_name.to_string(),
		})?;

		$rpc_fn($ctx, $mm, params).await.map(to_value)??
	}};

	// Without Params
	($rpc_fn:expr, $ctx:expr, $mm:expr) => {
		$rpc_fn($ctx, $mm).await.map(|r| to_value(r))??
	};
}

async fn _rpc_handler(
	ctx: Ctx,
	mm: ModelManager,
	rpc_req: RpcRequest,
) -> Result<Json<Value>> {
	let RpcRequest {
		id: rpc_id,
		method: rpc_method,
		params: rpc_params,
	} = rpc_req;

	debug!("{:<12} - _rpc_handler - method: {rpc_method}", "HANDLER");

	let result_json: Value = match rpc_method.as_str() {
		// User CRUD
		"create_user" => exec_rpc_fn!(create_user, ctx, mm, rpc_params),
		"list_users" => exec_rpc_fn!(list_users, ctx, mm),
		"get_user" => exec_rpc_fn!(get_user, ctx, mm, rpc_params),
		"update_user" => exec_rpc_fn!(update_user, ctx, mm, rpc_params),
		"delete_user" => exec_rpc_fn!(delete_user, ctx, mm, rpc_params),

		// Role CRUD
		"create_role" => exec_rpc_fn!(create_role, ctx, mm, rpc_params),
		"list_roles" => exec_rpc_fn!(list_roles, ctx, mm),
		"get_role" => exec_rpc_fn!(get_role, ctx, mm, rpc_params),
		"update_role" => exec_rpc_fn!(update_role, ctx, mm, rpc_params),
		"delete_role" => exec_rpc_fn!(delete_role, ctx, mm, rpc_params),

		// Project CRUD
		"create_project" => exec_rpc_fn!(create_project, ctx, mm, rpc_params),
		"list_projects" => exec_rpc_fn!(list_projects, ctx, mm),
		"get_project" => exec_rpc_fn!(get_project, ctx, mm, rpc_params),
		"update_project" => exec_rpc_fn!(update_project, ctx, mm, rpc_params),
		"delete_project" => exec_rpc_fn!(delete_project, ctx, mm, rpc_params),

		// Datatype CRUD
		"create_datatype" => exec_rpc_fn!(create_datatype, ctx, mm, rpc_params),
		"list_datatypes" => exec_rpc_fn!(list_datatypes, ctx, mm),
		"get_datatype" => exec_rpc_fn!(get_datatype, ctx, mm, rpc_params),
		"update_datatype" => exec_rpc_fn!(update_datatype, ctx, mm, rpc_params),
		"delete_datatype" => exec_rpc_fn!(delete_datatype, ctx, mm, rpc_params),

		// Index CRUD
		"create_index" => exec_rpc_fn!(create_index, ctx, mm, rpc_params),
		"list_indexes" => exec_rpc_fn!(list_indexes, ctx, mm),
		"get_index" => exec_rpc_fn!(get_index, ctx, mm, rpc_params),
		"update_index" => exec_rpc_fn!(update_index, ctx, mm, rpc_params),
		"delete_index" => exec_rpc_fn!(delete_index, ctx, mm, rpc_params),

		// Archive CRUD
		"create_archive" => exec_rpc_fn!(create_archive, ctx, mm, rpc_params),
		"list_archives" => exec_rpc_fn!(list_archives, ctx, mm),
		"get_archive" => exec_rpc_fn!(get_archive, ctx, mm, rpc_params),
		"update_archive" => exec_rpc_fn!(update_archive, ctx, mm, rpc_params),
		"delete_archive" => exec_rpc_fn!(delete_archive, ctx, mm, rpc_params),

		// Value CRUD
		"create_value" => exec_rpc_fn!(create_value, ctx, mm, rpc_params),
		"list_values" => exec_rpc_fn!(list_values, ctx, mm),
		"get_value" => exec_rpc_fn!(get_value, ctx, mm, rpc_params),
		"update_value" => exec_rpc_fn!(update_value, ctx, mm, rpc_params),
		"delete_value" => exec_rpc_fn!(delete_value, ctx, mm, rpc_params),

		// Separator CRUD
		"create_separator" => exec_rpc_fn!(create_separator, ctx, mm, rpc_params),
		"list_separators" => exec_rpc_fn!(list_separators, ctx, mm),
		"get_separator" => exec_rpc_fn!(get_separator, ctx, mm, rpc_params),
		"update_separator" => exec_rpc_fn!(update_separator, ctx, mm, rpc_params),
		"delete_separator" => exec_rpc_fn!(delete_separator, ctx, mm, rpc_params),

		// Document CRUD
		"create_document" => exec_rpc_fn!(create_document, ctx, mm, rpc_params),
		"list_documents" => exec_rpc_fn!(list_documents, ctx, mm),
		"get_document" => exec_rpc_fn!(get_document, ctx, mm, rpc_params),
		"update_document" => exec_rpc_fn!(update_document, ctx, mm, rpc_params),
		"delete_document" => exec_rpc_fn!(delete_document, ctx, mm, rpc_params),

		// Search Operations
		"get_project_fields" => {
			exec_rpc_fn!(get_project_fields, ctx, mm, rpc_params)
		}

		// -- Fallback error
		_ => return Err(Error::RpcMethodUnknown(rpc_method)),
	};

	let body_response = json!({
	  "id": rpc_id,
	  "result": result_json
	});

	Ok(Json(body_response))
}
