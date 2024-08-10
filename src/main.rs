#![allow(unused)] // For early development.

// region:    --- Modules

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
use tracing_subscriber::fmt::layer; // use crate::config

use crate::model::ModelManager;
use crate::web::mw_auth::{mw_ctx_require, mw_ctx_resolve};
use crate::web::mw_res_map::mw_reponse_map;
use crate::web::{routes_login, routes_static};
use axum::http::header::{ACCESS_CONTROL_ALLOW_ORIGIN, AUTHORIZATION, CONTENT_TYPE};
use axum::http::Method;
use axum::response::Html;
use axum::routing::get;
use axum::{middleware, Router};
use dotenvy::dotenv;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;
use tracing_subscriber::EnvFilter;

// endregion: --- Modules

#[tokio::main]
async fn main() -> Result<()> {
	dotenv().ok();

	tracing_subscriber::fmt()
		.with_target(false)
		.with_env_filter(EnvFilter::from_default_env())
		.init();

	// -- FOR DEV ONLY
	// _dev_utils::init_dev().await;

	// Set up cors
	let cors = CorsLayer::new()
		.allow_methods([Method::GET, Method::POST, Method::PUT])
		.allow_origin(Any)
		.allow_headers([CONTENT_TYPE, AUTHORIZATION, ACCESS_CONTROL_ALLOW_ORIGIN])
		.allow_credentials(true);
	// Initialize ModelManager.
	let mm = ModelManager::new().await?;

	// -- Define Routes
	// let routes_rpc = rpc::routes(mm.clone())
	//   .route_layer(middleware::from_fn(mw_ctx_require));

	let routes_hello = Router::new()
		.route("/hello", get(|| async { Html("Hello World") }))
		.route_layer(middleware::from_fn(mw_ctx_require))
		.layer(cors);

	let routes_all = Router::new()
		.merge(routes_login::routes(mm.clone()))
		.merge(routes_hello)
		// .nest("/api", routes_rpc)
		.layer(middleware::map_response(mw_reponse_map))
		.layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
		.layer(CookieManagerLayer::new())
		.fallback_service(routes_static::serve_dir());

	// region:    --- Start Server
	let addr: SocketAddr = SocketAddr::from(([0, 0, 0, 0], 8000));
	info!("{:<12} - {addr}\n", "LISTENING");
	axum::Server::bind(&addr)
		.serve(routes_all.into_make_service())
		.await
		.unwrap();
	// endregion: --- Start Server

	Ok(())
}
