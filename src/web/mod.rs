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
#[cfg(test)]
mod tests {
    use super::*;
    use tower_cookies::Cookie;
    use crate::web::SameSite;
    use uuid::Uuid;
    use std::cell::RefCell;

    // Minimal mock struct with interior mutability to track added and removed cookies by name
    struct MockCookies {
        added: RefCell<Vec<String>>,
        removed: RefCell<Vec<String>>,
    }

    impl MockCookies {
        fn new() -> Self {
            Self {
                added: RefCell::new(Vec::new()),
                removed: RefCell::new(Vec::new()),
            }
        }

        fn add_cookie(&self, user: &str, salt: Uuid) -> Result<()> {
            let token = generate_web_token(user, salt)?;
            let cookie = Cookie::new(AUTH_TOKEN, token.to_string());
            self.added.borrow_mut().push(cookie.name().to_string());
            Ok(())
        }

        fn remove_cookie(&self) -> Result<()> {
            let cookie = Cookie::from(AUTH_TOKEN);
            self.removed.borrow_mut().push(cookie.name().to_string());
            Ok(())
        }
    }

    #[test]
    fn test_set_token_cookie() {
        let cookies = MockCookies::new();
        let user = "test_user";
        let salt = Uuid::new_v4();

        // Call the add_cookie method to simulate setting a cookie
        let result = cookies.add_cookie(user, salt);
        assert!(result.is_ok());

        // Verify the added cookie name
        assert_eq!(*cookies.added.borrow(), vec![AUTH_TOKEN]);
    }

    #[test]
    fn test_remove_token_cookie() {
        let cookies = MockCookies::new();

        // Call the remove_cookie method to simulate removing a cookie
        let result = cookies.remove_cookie();
        assert!(result.is_ok());

        // Verify the removed cookie name
        assert_eq!(*cookies.removed.borrow(), vec![AUTH_TOKEN]);
    }
}
