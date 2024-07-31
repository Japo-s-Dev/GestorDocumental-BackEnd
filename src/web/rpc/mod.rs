use crate::web::{Error, Result};
use serde::Deserialize;
use serde_json::{from_value, json, to_value, Value};

#[derive(Deserialize)]
struct RpcRequest {
	id: Option<Value>,
	method: String,
	params: Option<Value>,
}
