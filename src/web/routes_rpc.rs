use super::error::{Error, Result};
use super::mw_auth::CtxW;
use crate::core::ctx::Ctx;
use crate::core::model::ModelManager;
use crate::rpc::{exec_rpc, File, RpcRequest};
use axum::extract::{Multipart, State};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, Router};
use serde_json::{json, Value};
use std::sync::Arc;
use tracing::debug;

pub fn routes(mm: ModelManager) -> Router {
	Router::new()
		.route("/rpc", post(rpc_handler))
		.with_state(mm)
}

#[derive(Debug)]
pub struct RpcInfo {
	pub id: Option<Value>,
	pub method: String,
}

async fn rpc_handler(
	State(mm): State<ModelManager>,
	ctx: CtxW,
	mut multipart: Multipart,
) -> Response {
	let ctx = ctx.0;
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
				return Error::FileExtractFailed.into_response();
			}

			let name = field.file_name().unwrap_or_default().to_owned();
			let content_type = field.content_type().unwrap_or_default().to_owned();
			let key = uuid::Uuid::new_v4().to_string();

			let bytes = field.bytes().await.unwrap();

			let file_name = format!("{}-{}", key, name);

			file_data = Some(File {
				file_name,
				content_type,
				key,
				bytes,
				successful: false,
			})
		}
	}

	let rpc_req: RpcRequest = match json_data {
		Some(data) => match serde_json::from_str(&data) {
			Ok(req) => req,
			Err(_) => {
				return Error::InvalidJson.into_response();
			}
		},
		None => {
			return Error::NoJsonInRequest.into_response();
		}
	};

	let rpc_info = RpcInfo {
		id: rpc_req.id.clone(),
		method: rpc_req.method.clone(),
	};

	// Call the original handler
	let mut res = _rpc_handler(ctx, mm, rpc_req, file_data)
		.await
		.into_response();
	res.extensions_mut().insert(Arc::new(rpc_info));
	res
}

async fn _rpc_handler(
	ctx: Ctx,
	mm: ModelManager,
	rpc_req: RpcRequest,
	file: Option<File>,
) -> Result<Json<Value>> {
	let rpc_method = rpc_req.method.clone();
	let rpc_id = rpc_req.id.clone();

	debug!("{:<12} - _rpc_handler - method: {rpc_method}", "HANDLER");

	let result = exec_rpc(ctx, mm, rpc_req, file).await?;

	let body_response = json!({
		"id": rpc_id,
		"result": result
	});

	Ok(Json(body_response))
}
