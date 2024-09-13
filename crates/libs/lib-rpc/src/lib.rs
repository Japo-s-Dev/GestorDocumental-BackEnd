mod archive_rpc;
mod config;
mod datatype_rpc;
mod document_rpc;
mod error;
mod index_rpc;
mod params;
mod project_rpc;
mod role_rpc;
mod search_operations_rpc;
mod separator_rpc;
mod user_rpc;
mod value_rpc;

pub use self::error::{Error, Result};
use self::{
	archive_rpc::*, datatype_rpc::*, document_rpc::*, index_rpc::*, project_rpc::*,
	role_rpc::*, search_operations_rpc::*, separator_rpc::*, user_rpc::*,
	value_rpc::*,
};
use axum::body::Bytes;
use lib_core::{ctx::Ctx, model::ModelManager};
use params::*;
use serde::{Deserialize, Serialize};
use serde_json::{from_value, to_value, Value};

#[derive(Deserialize)]
pub struct RpcRequest {
	pub id: Option<Value>,
	pub method: String,
	pub params: Option<Value>,
}

#[derive(Serialize)]
pub struct File {
	pub key: String,
	pub successful: bool,
	pub url: String,
	pub file_name: String,
	pub content_type: String,
	#[serde(skip_serializing)]
	pub bytes: Bytes,
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

pub async fn exec_rpc(
	ctx: Ctx,
	mm: ModelManager,
	rpc_req: RpcRequest,
	_file: Option<File>,
) -> Result<Value> {
	let rpc_method = rpc_req.method;
	let rpc_params = rpc_req.params;

	let result_json: Value = match rpc_method.as_str() {
		// User CRUD
		"create_user" => exec_rpc_fn!(create_user, ctx, mm, rpc_params),
		"list_users" => exec_rpc_fn!(list_users, ctx, mm, rpc_params),
		"get_user" => exec_rpc_fn!(get_user, ctx, mm, rpc_params),
		"update_user" => exec_rpc_fn!(update_user, ctx, mm, rpc_params),
		"delete_user" => exec_rpc_fn!(delete_user, ctx, mm, rpc_params),

		// Role CRUD
		"create_role" => exec_rpc_fn!(create_role, ctx, mm, rpc_params),
		"list_roles" => exec_rpc_fn!(list_roles, ctx, mm, rpc_params),
		"get_role" => exec_rpc_fn!(get_role, ctx, mm, rpc_params),
		"update_role" => exec_rpc_fn!(update_role, ctx, mm, rpc_params),
		"delete_role" => exec_rpc_fn!(delete_role, ctx, mm, rpc_params),

		// Project CRUD
		"create_project" => exec_rpc_fn!(create_project, ctx, mm, rpc_params),
		"list_projects" => exec_rpc_fn!(list_projects, ctx, mm, rpc_params),
		"get_project" => exec_rpc_fn!(get_project, ctx, mm, rpc_params),
		"update_project" => exec_rpc_fn!(update_project, ctx, mm, rpc_params),
		"delete_project" => exec_rpc_fn!(delete_project, ctx, mm, rpc_params),

		// Datatype CRUD
		"create_datatype" => exec_rpc_fn!(create_datatype, ctx, mm, rpc_params),
		"list_datatypes" => exec_rpc_fn!(list_datatypes, ctx, mm, rpc_params),
		"get_datatype" => exec_rpc_fn!(get_datatype, ctx, mm, rpc_params),
		"update_datatype" => exec_rpc_fn!(update_datatype, ctx, mm, rpc_params),
		"delete_datatype" => exec_rpc_fn!(delete_datatype, ctx, mm, rpc_params),

		// Index CRUD
		"create_index" => exec_rpc_fn!(create_index, ctx, mm, rpc_params),
		"list_indexes" => exec_rpc_fn!(list_indexes, ctx, mm, rpc_params),
		"get_index" => exec_rpc_fn!(get_index, ctx, mm, rpc_params),
		"update_index" => exec_rpc_fn!(update_index, ctx, mm, rpc_params),
		"delete_index" => exec_rpc_fn!(delete_index, ctx, mm, rpc_params),

		// Archive CRUD
		"create_archive" => exec_rpc_fn!(create_archive, ctx, mm, rpc_params),
		"list_archives" => exec_rpc_fn!(list_archives, ctx, mm, rpc_params),
		"get_archive" => exec_rpc_fn!(get_archive, ctx, mm, rpc_params),
		"update_archive" => exec_rpc_fn!(update_archive, ctx, mm, rpc_params),
		"delete_archive" => exec_rpc_fn!(delete_archive, ctx, mm, rpc_params),

		// Value CRUD
		"create_value" => exec_rpc_fn!(create_value, ctx, mm, rpc_params),
		"list_values" => exec_rpc_fn!(list_values, ctx, mm, rpc_params),
		"get_value" => exec_rpc_fn!(get_value, ctx, mm, rpc_params),
		"update_value" => exec_rpc_fn!(update_value, ctx, mm, rpc_params),
		"delete_value" => exec_rpc_fn!(delete_value, ctx, mm, rpc_params),

		// Separator CRUD
		"create_separator" => exec_rpc_fn!(create_separator, ctx, mm, rpc_params),
		"list_separators" => exec_rpc_fn!(list_separators, ctx, mm, rpc_params),
		"get_separator" => exec_rpc_fn!(get_separator, ctx, mm, rpc_params),
		"update_separator" => exec_rpc_fn!(update_separator, ctx, mm, rpc_params),
		"delete_separator" => exec_rpc_fn!(delete_separator, ctx, mm, rpc_params),

		// Document CRUD
		"create_document" => exec_rpc_fn!(create_document, ctx, mm, rpc_params),
		"list_documents" => exec_rpc_fn!(list_documents, ctx, mm, rpc_params),
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

	Ok(result_json)
}

/*
*

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

* */
