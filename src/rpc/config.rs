use crate::utils::envs::get_env;
use std::sync::OnceLock;

pub fn rpc_config() -> &'static RpcConfig {
	static INSTANCE: OnceLock<RpcConfig> = OnceLock::new();

	INSTANCE.get_or_init(|| {
		RpcConfig::load_from_env().unwrap_or_else(|ex| {
			panic!("FATAL - WHILE LOADING CONF - Cause: {ex:?}")
		})
	})
}

#[allow(non_snake_case)]
pub struct RpcConfig {
	// -- S3
	pub AWS_BUCKET_NAME: String,
}

impl RpcConfig {
	fn load_from_env() -> crate::utils::envs::Result<RpcConfig> {
		Ok(RpcConfig {
			// -- S3
			AWS_BUCKET_NAME: get_env("AWS_BUCKET_NAME")?,
		})
	}
}
