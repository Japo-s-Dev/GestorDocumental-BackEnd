// region:    --- Modules
//#![allow(unused)]
mod config;
mod error;
mod log;
mod web;
// #[cfg(test)]

pub use crate::error::{Error, Result};
use crate::web::mw_auth::{mw_ctx_require, mw_ctx_resolve};
use crate::web::mw_res_map::mw_reponse_map;
use crate::web::{routes_login, routes_rpc, routes_static};
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE, COOKIE, SET_COOKIE};
use axum::http::{HeaderValue, Method};
use axum::{middleware, Router};
pub use config::web_config;
use lib_core::model::ModelManager;
use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
	tracing_subscriber::fmt()
		.with_target(false)
		.with_env_filter(EnvFilter::from_default_env())
		.init();

	// -- FOR DEV ONLY
	// _dev_utils::init_dev().await;

	let origins = [
		"http://52.204.86.10".parse::<HeaderValue>().unwrap(),
		"http://190.56.194.12:3400".parse::<HeaderValue>().unwrap(),
		"http://localhost:3400".parse::<HeaderValue>().unwrap(),
		"http://localhost:4200".parse::<HeaderValue>().unwrap(),
		"https://52.204.86.10".parse::<HeaderValue>().unwrap(),
		"https://190.56.194.12:3400".parse::<HeaderValue>().unwrap(),
		"https://localhost:3400".parse::<HeaderValue>().unwrap(),
		"https://localhost:4200".parse::<HeaderValue>().unwrap(),
	];

	// Set up cors
	let cors = CorsLayer::new()
		.allow_origin(origins)
		.allow_credentials(true)
		.allow_methods([Method::GET, Method::POST, Method::OPTIONS])
		.allow_headers([CONTENT_TYPE, AUTHORIZATION, ACCEPT, COOKIE, SET_COOKIE]);
	// Initialize ModelManager.
	let mm = ModelManager::new().await?;

	// -- Define Routes
	let routes_rpc = routes_rpc::routes(mm.clone())
		.route_layer(middleware::from_fn(mw_ctx_require));

	//	let routes_hello = Router::new()
	//		.route("/hello", get(|| async { Html("Hello World") }))
	//		.route_layer(middleware::from_fn(mw_ctx_require));

	let routes_all = Router::new()
		.merge(routes_login::routes(mm.clone()))
		//		.merge(routes_hello)
		.nest("/api", routes_rpc)
		.layer(middleware::map_response(mw_reponse_map))
		.layer(cors.clone())
		.layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
		.layer(CookieManagerLayer::new())
		.fallback_service(routes_static::serve_dir());

	// region:    --- Start Server
	let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
	info!("{:<12} - {:?}\n", "LISTENING", listener.local_addr());
	axum::serve(listener, routes_all.into_make_service())
		.await
		.unwrap();
	// endregion: --- Start Server

	Ok(())
}
