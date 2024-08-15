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
use axum::http::request::Parts;
pub use config::config;
use tower::{Layer, ServiceBuilder};
use tower_http::trace::TraceLayer;
use tracing_subscriber::fmt::layer; // use crate::config

use crate::model::ModelManager;
use crate::web::mw_auth::{mw_ctx_require, mw_ctx_resolve};
use crate::web::mw_res_map::mw_reponse_map;
use crate::web::rpc;
use crate::web::{routes_login, routes_static};
use axum::body::{boxed, Body, BoxBody};
use axum::extract::{ConnectInfo, TypedHeader};
use axum::headers::{HeaderMapExt, Origin};
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderName, HeaderValue, Method, Request, StatusCode};
use axum::middleware::Next;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{any, get};
use axum::{middleware, RequestPartsExt, Router, ServiceExt};
use axum_client_ip::XRealIp;
use dotenvy::dotenv;
use std::net::SocketAddr;
use tower_cookies::CookieManagerLayer;
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
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
	];

	// Set up cors
	let cors = CorsLayer::new()
		.allow_origin(origins)
		//.allow_credentials(true)
		.allow_methods([Method::GET, Method::POST, Method::OPTIONS])
		.allow_headers([CONTENT_TYPE, AUTHORIZATION, ACCEPT]);

	// Initialize ModelManager.
	let mm = ModelManager::new().await?;

	let middleware_stack = ServiceBuilder::new()
		.layer(axum::middleware::from_fn::<_, Body>(ip_extractor::<Body>));

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
		.layer(middleware::from_fn(ip_extractor))
		.layer(middleware::from_fn_with_state(mm.clone(), mw_ctx_resolve))
		.layer(CookieManagerLayer::new())
		.layer(cors)
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

async fn ip_extractor<B>(
	mut req: Request<B>,
	next: Next<B>,
) -> Result<Response<BoxBody>>
where
	B: Send + 'static,
{
	let real_ip = req
		.headers()
		.get("X-Real-Ip")
		.and_then(|hv| hv.to_str().ok())
		.unwrap_or("*")
		.to_string();

	let mut owned: String = "http://".to_owned();
	owned.push_str(&real_ip);

	req.extensions_mut().insert(owned.clone());

	// Directly get the response, no need for map_err here
	let mut res = next.run(req).await;

	res.headers_mut().insert(
		"Access-Control-Allow-Origin",
		HeaderValue::from_str(&owned).unwrap_or(HeaderValue::from_static("*")),
	);

	// Return the boxed response
	Ok(res.map(boxed))
}
