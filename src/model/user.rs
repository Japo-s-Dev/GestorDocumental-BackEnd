use crate::crypt::{pwd, EncryptContent};
use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use serde_json::{from_value, json};
use sqlb::{Fields, HasFields};
use sqlx::postgres::PgRow;
use sqlx::FromRow;
use uuid::Uuid;

// region:    --- User Types
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct User {
	pub id: i64,
	pub email: String,
	pub username: String,
}

#[derive(Deserialize, Fields)]
pub struct UserForCreate {
	pub username: String,
	pub pwd_clear: String,
	pub email: String,
}

#[derive(Fields)]
pub struct UserForInsert {
	pub username: String,
	pub email: String,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForLogin {
	pub id: i64,
	pub username: String,
	pub email: String,

	// -- pwd and token info
	pub pwd: Option<String>, // encrypted, #_scheme_id_#....
	pub pwd_salt: Uuid,
	pub token_salt: Uuid,
}

#[derive(Clone, FromRow, Fields, Debug)]
pub struct UserForAuth {
	pub id: i64,
	pub username: String,

	// -- token info
	pub token_salt: Uuid,
}

#[derive(Deserialize, Clone, FromRow, Fields, Debug)]
pub struct UserForUpdate {
	pub username: String,
	pub email: String,
}

/// Marker trait
pub trait UserBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl UserBy for User {}
impl UserBy for UserForLogin {}
impl UserBy for UserForAuth {}
impl UserBy for UserForUpdate {}
// endregion: --- User Types

pub struct UserBmc;

impl DbBmc for UserBmc {
	const TABLE: &'static str = "user";
}

impl UserBmc {
	pub async fn get<E>(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<E>
	where
		E: UserBy,
	{
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn first_by_username<E>(
		_ctx: &Ctx,
		mm: &ModelManager,
		username: &str,
	) -> Result<Option<E>>
	where
		E: UserBy,
	{
		let db = mm.db();

		let user = sqlb::select()
			.table(Self::TABLE)
			.and_where("username", "=", username)
			.fetch_optional::<_, E>(db)
			.await?;

		Ok(user)
	}

	pub async fn update_pwd(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		pwd_clear: &str,
	) -> Result<()> {
		let db = mm.db();

		let user: UserForLogin = Self::get(ctx, mm, id).await?;
		let pwd = pwd::encrypt_pwd(&EncryptContent {
			content: pwd_clear.to_string(),
			salt: user.pwd_salt.to_string(),
		})?;

		sqlb::update()
			.table(Self::TABLE)
			.and_where("id", "=", id)
			.data(vec![("pwd", pwd.to_string()).into()])
			.exec(db)
			.await?;

		Ok(())
	}

	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		user_c: UserForCreate,
	) -> Result<i64> {
		let data: UserForInsert = UserForInsert {
			username: user_c.username,
			email: user_c.email,
		};

		let user_id = base::create::<Self, _>(ctx, mm, data).await?;

		Self::update_pwd(ctx, mm, user_id, &user_c.pwd_clear).await?;

		Ok(user_id)
	}

	pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<User>> {
		base::list::<Self, _>(ctx, mm).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		user_u: UserForUpdate,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, user_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}

// region:    --- Tests
// #[cfg(test)]
// mod tests {
// 	use super::*;
// 	use crate::_dev_utils;
// 	use anyhow::{Context, Result};
// 	use serial_test::serial;
//
// 	#[serial]
// 	#[tokio::test]
// 	async fn test_first_ok_demo1() -> Result<()> {
// 		// -- Setup & Fixtures
// 		let mm = _dev_utils::init_test().await;
// 		let ctx = Ctx::root_ctx();
// 		let fx_username = "demo1";
//
// 		// -- Exec
// 		let user: User = UserBmc::first_by_username(&ctx, &mm, fx_username)
// 			.await?
// 			.context("Should have user 'demo1'")?;
//
// 		// -- Check
// 		assert_eq!(user.username, fx_username);
//
// 		Ok(())
// 	}
// }
// endregion: --- Tests
