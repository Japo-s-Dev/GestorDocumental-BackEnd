use crate::auth::pwd::{self, ContentToHash, SchemeStatus};
use crate::core::ctx::Ctx;
use crate::core::model::associated_privilege::AssociatedPrivilegeBmc;
use crate::core::model::user::{UserBmc, UserForLogin};
use crate::core::model::ModelManager;
use crate::web::{remove_token_cookie, set_token_cookie, Error, Result};
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::Cookies;
use tracing::debug;

pub fn routes(mm: ModelManager) -> Router {
	Router::new()
		.route("/api/login", post(api_login_handler))
		.route("/api/logoff", post(api_logoff_handler))
		.with_state(mm)
}

async fn api_login_handler(
	State(mm): State<ModelManager>,
	cookies: Cookies,
	Json(payload): Json<LoginPayload>,
) -> Result<Json<Value>> {
	debug!(" {:<12} - api_login_handler", "HANDLER");

	let LoginPayload {
		username,
		pwd: pwd_clear,
	} = payload;
	let root_ctx = Ctx::root_ctx();

	let user: UserForLogin = UserBmc::first_by_username(&root_ctx, &mm, &username)
		.await?
		.ok_or(Error::LoginFailUsernameNotFound)?;

	let user_id = user.id;

	let Some(pwd): Option<String> = user.pwd else {
		return Err(Error::LoginFailUserHasNoPwd { user_id });
	};

	let scheme_status = pwd::validate_pwd(
		ContentToHash {
			salt: user.pwd_salt,
			content: pwd_clear.clone(),
		},
		pwd,
	)
	.await
	.map_err(|_| Error::LoginFailPwdNotMatching { user_id })?;

	if let SchemeStatus::Outdated = scheme_status {
		debug!("pwd encrypt scheme outdated, upgrading");
		UserBmc::update_pwd(&root_ctx, &mm, user.id, &pwd_clear).await?;
	}

	// Set web token
	set_token_cookie(&cookies, &user.username, user.token_salt)?;

	// Create the success body.
	let body = Json(json!({
		"result": {
			"success": true,
			"username": user.username,
			"role": user.assigned_role
		}
	}));

	Ok(body)
}

async fn api_logoff_handler(
	cookies: Cookies,
	Json(payload): Json<LogoffPayload>,
) -> Result<Json<Value>> {
	debug!("{:<12} - api_logoff_handler", "HANDLER");
	let should_logoff = payload.logoff;

	if should_logoff {
		remove_token_cookie(&cookies)?;
	}

	// Create the success body.
	let body = Json(json!({
		"result": {
			"logged_off": should_logoff
		}
	}));

	Ok(body)
}

#[derive(Debug, Deserialize)]
struct LogoffPayload {
	logoff: bool,
}

#[derive(Debug, Deserialize)]
struct LoginPayload {
	username: String,
	pwd: String,
}
