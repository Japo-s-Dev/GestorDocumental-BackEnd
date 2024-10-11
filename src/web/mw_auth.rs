use crate::auth::token::{validate_web_token, Token};
use crate::core::ctx::Ctx;
use crate::core::model::associated_privilege::AssociatedPrivilegeBmc;
use crate::core::model::user::{UserBmc, UserForAuth};
use crate::core::model::ModelManager;
use crate::web::{set_privileges_cookie, set_token_cookie, AUTH_TOKEN, PRIVILEGES};
use crate::web::{Error, Result};
use async_trait::async_trait;
use axum::body::Body;
use axum::extract::{FromRequestParts, State};
use axum::http::request::Parts;
use axum::http::Request;
use axum::middleware::Next;
use axum::response::Response;
use serde::Serialize;
use tower_cookies::{Cookie, Cookies};
use tracing::debug;

pub async fn mw_ctx_require(
	ctx: Result<CtxW>,
	req: Request<Body>,
	next: Next,
) -> Result<Response> {
	debug!(" {:<12} - mw_ctx_require - {ctx:?}", "MIDDLEWARE");

	ctx?;

	Ok(next.run(req).await)
}

pub async fn mw_ctx_resolve(
	mm: State<ModelManager>,
	cookies: Cookies,
	mut req: Request<Body>,
	next: Next,
) -> Result<Response> {
	debug!(" {:<12} - mw_ctx_resolve", "MIDDLEWARE");

	let ctx_ext_result = _ctx_resolve(mm, &cookies).await;

	// Remove the cookie if something went wrong other than NoAuthTokenCookie.
	if ctx_ext_result.is_err()
		&& !matches!(ctx_ext_result, Err(CtxExtError::TokenNotInCookie))
	{
		cookies.remove(Cookie::from(AUTH_TOKEN));
		cookies.remove(Cookie::from(PRIVILEGES));
	}

	// Store the ctx_result in the request extension.
	req.extensions_mut().insert(ctx_ext_result);

	Ok(next.run(req).await)
}

async fn _ctx_resolve(mm: State<ModelManager>, cookies: &Cookies) -> CtxExtResult {
	// -- Get Token String
	let token = cookies
		.get(AUTH_TOKEN)
		.map(|c| c.value().to_string())
		.ok_or(CtxExtError::TokenNotInCookie)?;

	// -- Parse Token
	let token: Token = token.parse().map_err(|_| CtxExtError::TokenWrongFormat)?;

	let tmp_ctx = Ctx::root_ctx();

	// -- Get UserForAuth
	let user: UserForAuth = UserBmc::first_by_username(&tmp_ctx, &mm, &token.ident)
		.await
		.map_err(|ex| CtxExtError::ModelAccessError(ex.to_string()))?
		.ok_or(CtxExtError::UserNotFound)?;

	let privileges = AssociatedPrivilegeBmc::list_by_role_name(
		&tmp_ctx,
		&mm,
		&user.assigned_role,
	)
	.await
	.map_err(|ex| CtxExtError::ModelAccessError(ex.to_string()))?;

	let privilege_ids: Vec<i64> = privileges
		.iter()
		.map(|privilege| privilege.privilege_id)
		.collect();

	// -- Validate Token
	validate_web_token(&token, user.token_salt)
		.map_err(|_| CtxExtError::FailValidate)?;

	// -- Update Token
	set_token_cookie(cookies, &user.username, user.token_salt)
		.map_err(|_| CtxExtError::CannotSetTokenCookie)?;

	set_privileges_cookie(cookies, &privilege_ids)
		.map_err(|_| CtxExtError::CannotSetPrivilegesCookie)?;

	// -- Create CtxExtResult
	Ctx::new(user.id)
		.map(CtxW)
		.map_err(|ex| CtxExtError::CtxCreateFail(ex.to_string()))
}

#[derive(Debug, Clone)]
pub struct CtxW(pub Ctx);

// region:    --- Ctx Extractor
#[async_trait]
impl<S: Send + Sync> FromRequestParts<S> for CtxW {
	type Rejection = Error;

	async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
		debug!(" {:<12} - Ctx", "EXTRACTOR");

		parts
			.extensions
			.get::<CtxExtResult>()
			.ok_or(Error::CtxExt(CtxExtError::CtxNotInRequestExt))?
			.clone()
			.map_err(Error::CtxExt)
	}
}
// endregion: --- Ctx Extractor

// region:    --- Ctx Extractor Result/Error
type CtxExtResult = core::result::Result<CtxW, CtxExtError>;

#[derive(Clone, Serialize, Debug)]
pub enum CtxExtError {
	TokenNotInCookie,

	TokenWrongFormat,

	UserNotFound,
	ModelAccessError(String),
	FailValidate,
	CannotSetTokenCookie,
	CannotSetPrivilegesCookie,

	CtxNotInRequestExt,
	CtxCreateFail(String),
}
// endregion: --- Ctx Extractor Result/Error
