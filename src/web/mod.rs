// region:    --- Modules

mod error;
pub mod mw_auth;
pub mod mw_res_map;
pub mod routes_login;
pub mod routes_rpc;
pub mod routes_static;
use crate::auth::token::generate_web_token;
use tower_cookies::cookie::SameSite;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

pub use self::error::ClientError;
pub use self::error::{Error, Result};

// endregion: --- Modules

pub const AUTH_TOKEN: &str = "auth-token";
pub const PRIVILEGES: &str = "privileges";

fn set_token_cookie(cookies: &Cookies, user: &str, salt: Uuid) -> Result<()> {
	let token = generate_web_token(user, salt)?;

	let mut cookie = Cookie::new(AUTH_TOKEN, token.to_string());
	cookie.set_http_only(true);
	cookie.set_path("/");
	cookie.set_secure(true);
	cookie.set_same_site(SameSite::None);
	//cookie.set_domain("localhost:3400");

	cookies.add(cookie);

	Ok(())
}

fn set_privileges_cookie(cookies: &Cookies, privileges: &Vec<i64>) -> Result<()> {
	let privileges_string = privileges
		.iter()
		.map(|id| id.to_string())
		.collect::<Vec<_>>()
		.join(",");

	let mut cookie = Cookie::new(PRIVILEGES, privileges_string.to_string());
	cookie.set_http_only(true);
	cookie.set_path("/");
	cookie.set_secure(true);
	cookie.set_same_site(SameSite::None);
	//cookie.set_domain("localhost:3400");

	cookies.add(cookie);

	Ok(())
}

fn remove_token_cookie(cookies: &Cookies) -> Result<()> {
	let mut cookie = Cookie::from(AUTH_TOKEN);
	cookie.set_path("/");

	cookies.remove(cookie);

	Ok(())
}
