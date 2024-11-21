// region:    --- Modules

mod error;
mod scheme;

pub use self::error::{Error, Result};
pub use self::scheme::SchemeStatus;

use crate::auth::pwd::scheme::{get_scheme, Scheme, DEFAULT_SCHEME};
use lazy_regex::regex_captures;
use std::str::FromStr;
use uuid::Uuid;

// endregion: --- Modules

// region:    --- Types

#[cfg_attr(test, derive(Clone))]
pub struct ContentToHash {
	pub content: String, // Clear content.
	pub salt: Uuid,
}

// endregion: --- Types

// region:    --- Public Functions

/// Hash the password with the default scheme.
pub async fn hash_pwd(to_hash: ContentToHash) -> Result<String> {
	tokio::task::spawn_blocking(move || hash_for_scheme(DEFAULT_SCHEME, to_hash))
		.await
		.map_err(|_| Error::FailSpawnBlockForHash)?
}

/// Validate if an ContentToHash matches.
pub async fn validate_pwd(
	to_hash: ContentToHash,
	pwd_ref: String,
) -> Result<SchemeStatus> {
	let PwdParts {
		scheme_name,
		hashed,
	} = pwd_ref.parse()?;

	let scheme_status = if scheme_name == DEFAULT_SCHEME {
		SchemeStatus::Ok
	} else {
		SchemeStatus::Outdated
	};
	tokio::task::spawn_blocking(move || {
		validate_for_scheme(&scheme_name, to_hash, hashed)
	})
	.await
	.map_err(|_| Error::FailSpawnBlockForValidate)??;

	Ok(scheme_status)
}
// endregion: --- Public Functions

// region:    --- Privates

fn hash_for_scheme(scheme_name: &str, to_hash: ContentToHash) -> Result<String> {
	let pwd_hashed = get_scheme(scheme_name)?.hash(&to_hash)?;

	Ok(format!("#{scheme_name}#{pwd_hashed}"))
}

fn validate_for_scheme(
	scheme_name: &str,
	to_hash: ContentToHash,
	pwd_ref: String,
) -> Result<()> {
	get_scheme(scheme_name)?.validate(&to_hash, &pwd_ref)?;
	Ok(())
}

struct PwdParts {
	/// The scheme only (e.g., "01")
	scheme_name: String,
	/// The hashed password,
	hashed: String,
}

impl FromStr for PwdParts {
	type Err = Error;

	fn from_str(pwd_with_scheme: &str) -> Result<Self> {
		regex_captures!(
			r#"^#(\w+)#(.*)"#, // a literal regex
			pwd_with_scheme
		)
		.map(|(_, scheme, hashed)| Self {
			scheme_name: scheme.to_string(),
			hashed: hashed.to_string(),
		})
		.ok_or(Error::PwdWithSchemeFailedParse)
	}
}

// endregion: --- Privates

// region:    --- Tests
#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use uuid::Uuid;

    #[tokio::test]
    async fn test_multi_scheme_ok() -> Result<()> {
        // -- Setup & Fixtures
        let fx_salt = Uuid::parse_str("fa00ff8e-1757-490e-8856-1c046ef7ae80")?;
        let fx_to_hash = ContentToHash {
            content: "welcome".to_string(),
            salt: fx_salt,
        };

        // -- Exec
        let pwd_hashed = hash_for_scheme("02", fx_to_hash.clone())?;
        let pwd_validate = validate_pwd(fx_to_hash.clone(), pwd_hashed.clone()).await?;

        // -- Check that the status is `SchemeStatus::Outdated` because the scheme is not the default.
        assert!(
            matches!(pwd_validate, SchemeStatus::Outdated),
            "status should be SchemeStatus::Outdated"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_hash_pwd_ok() -> Result<()> {
        // -- Setup & Fixtures
        let fx_salt = Uuid::new_v4();
        let fx_to_hash = ContentToHash {
            content: "password123".to_string(),
            salt: fx_salt,
        };

        // -- Exec
        let hashed_pwd = hash_pwd(fx_to_hash.clone()).await?;

        // -- Check
        assert!(hashed_pwd.starts_with("#"), "The hash should contain the scheme identifier.");
        Ok(())
    }

    #[tokio::test]
    async fn test_validate_pwd_ok() -> Result<()> {
        // -- Setup & Fixtures
        let fx_salt = Uuid::new_v4();
        let fx_to_hash = ContentToHash {
            content: "password123".to_string(),
            salt: fx_salt,
        };

        // -- Hash the password first
        let hashed_pwd = hash_pwd(fx_to_hash.clone()).await?;

        // -- Validate the password
        let validation_result = validate_pwd(fx_to_hash, hashed_pwd.clone()).await?;

        // -- Check
        assert!(
            matches!(validation_result, SchemeStatus::Ok),
            "The status should be SchemeStatus::Ok for the default scheme"
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_validate_pwd_outdated_scheme() -> Result<()> {
        // -- Setup & Fixtures
        let fx_salt = Uuid::new_v4();
        let fx_to_hash = ContentToHash {
            content: "password123".to_string(),
            salt: fx_salt,
        };

        // -- Hash the password with a different scheme
        let hashed_pwd = hash_for_scheme("02", fx_to_hash.clone())?;

        // -- Validate using the default scheme (should return Outdated)
        let validation_result = validate_pwd(fx_to_hash, hashed_pwd.clone()).await?;

        // -- Check
        assert!(
            matches!(validation_result, SchemeStatus::Outdated),
            "The status should be SchemeStatus::Outdated for non-default schemes"
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_invalid_password_validation() -> Result<()> {
        // -- Setup & Fixtures
        let fx_salt = Uuid::new_v4();
        let fx_to_hash = ContentToHash {
            content: "password123".to_string(),
            salt: fx_salt,
        };

        // -- Use an incorrect password hash reference
        let invalid_hashed_pwd = "#01#invalid_hash".to_string();

        // -- Validate with the wrong hash (should fail)
        let validation_result = validate_pwd(fx_to_hash, invalid_hashed_pwd).await;

        // -- Check that validation fails
        assert!(validation_result.is_err(), "Validation should fail for invalid password hash");
        Ok(())
    }
}
