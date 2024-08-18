// region:    --- Modules
#![allow(unused)]
mod config;
mod crypt;
mod ctx;
mod error;
mod log;
mod model;
mod utils;
mod web;
// #[cfg(test)]
pub mod _dev_utils;

pub use self::error::{Error, Result};
pub use config::config;
use hyper::header::{COOKIE, SET_COOKIE};
use tower_http::trace::TraceLayer;

use crate::model::ModelManager;
use crate::web::mw_auth::{mw_ctx_require, mw_ctx_resolve};
use crate::web::mw_res_map::mw_reponse_map;
use crate::web::rpc;
use crate::web::{routes_login, routes_static};
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderValue, Method};
use axum::{middleware, Router};
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};
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
	let routes_rpc =
		rpc::routes(mm.clone()).route_layer(middleware::from_fn(mw_ctx_require));

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
		.layer(TraceLayer::new_for_http())
		.fallback_service(routes_static::serve_dir());

	// region:    --- Start Server
	let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 8000));
	info!("{:<12} - {addr}\n", "LISTENING");
	axum::Server::bind(&addr)
		.serve(routes_all.into_make_service_with_connect_info::<SocketAddr>())
		.await
		.unwrap();
	// endregion: --- Start Server

	Ok(())
}
