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

#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct User {
	pub id: i64,
	pub email: String,
	pub username: String,
	pub assigned_role: String,
}

#[derive(Deserialize, Fields)]
pub struct UserForCreate {
	pub username: String,
	pub pwd_clear: String,
	pub email: String,
	pub assigned_role: String,
}

#[derive(Fields)]
pub struct UserForInsert {
	pub username: String,
	pub email: String,
	pub assigned_role: String,
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
	pub assigned_role: String,
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
	pub assigned_role: String,
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
			assigned_role: user_c.assigned_role,
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
#[cfg(test)]
mod tests {
	use super::*;
	use crate::ctx::Ctx;
	use crate::model::ModelManager;
	use sqlx::{Executor, PgPool};
	use uuid::Uuid;

	async fn setup_test_db() -> PgPool {
		let pool = PgPool::connect("postgres://user:password@localhost/test_db")
			.await
			.unwrap();

		// Crear las tablas necesarias para la prueba
		pool.execute(
			"CREATE TABLE IF NOT EXISTS \"user\" (
            id BIGSERIAL PRIMARY KEY,
            username VARCHAR NOT NULL,
            email VARCHAR NOT NULL,
            pwd VARCHAR,
            pwd_salt UUID NOT NULL,
            token_salt UUID NOT NULL,
            assigned_role VARCHAR NOT NULL
        )",
		)
		.await
		.unwrap();

		pool
	}

	fn setup_test_ctx() -> Ctx {
		// Aquí asumo que estás configurando un user_id de prueba. Puedes ajustar esto según tus necesidades.
		Ctx::new(1).expect("Failed to create test Ctx")
	}

	#[tokio::test]
	async fn test_create_user() {
		let pool = setup_test_db().await;
		let ctx = setup_test_ctx();
		let mm = ModelManager::new();

		let user_c = UserForCreate {
			username: "testuser".to_string(),
			pwd_clear: "securepassword".to_string(),
			email: "test@example.com".to_string(),
			assigned_role: "user".to_string(),
		};

		let user_id = UserBmc::create(&ctx, &mm, user_c).await.unwrap();

		let user = UserBmc::get::<User>(&ctx, &mm, user_id).await.unwrap();
		assert_eq!(user.username, "testuser");
		assert_eq!(user.email, "test@example.com");

		pool.execute("DELETE FROM \"user\"").await.unwrap();
	}

	#[tokio::test]
	async fn test_get_user_by_id() {
		let pool = setup_test_db().await;
		let ctx = setup_test_ctx();
		let mm = ModelManager::new(pool.clone());

		let user_c = UserForCreate {
			username: "testuser".to_string(),
			pwd_clear: "securepassword".to_string(),
			email: "test@example.com".to_string(),
			assigned_role: "user".to_string(),
		};

		let user_id = UserBmc::create(&ctx, &mm, user_c).await.unwrap();
		let user = UserBmc::get::<User>(&ctx, &mm, user_id).await.unwrap();

		assert_eq!(user.id, user_id);
		assert_eq!(user.username, "testuser");

		pool.execute("DELETE FROM \"user\"").await.unwrap();
	}

	#[tokio::test]
	async fn test_get_user_by_username() {
		let pool = setup_test_db().await;
		let ctx = setup_test_ctx();
		let mm = ModelManager::new(pool.clone());

		let user_c = UserForCreate {
			username: "testuser".to_string(),
			pwd_clear: "securepassword".to_string(),
			email: "test@example.com".to_string(),
			assigned_role: "user".to_string(),
		};

		UserBmc::create(&ctx, &mm, user_c).await.unwrap();
		let user = UserBmc::first_by_username::<User>(&ctx, &mm, "testuser")
			.await
			.unwrap()
			.unwrap();

		assert_eq!(user.username, "testuser");

		pool.execute("DELETE FROM \"user\"").await.unwrap();
	}

	#[tokio::test]
	async fn test_update_password() {
		let pool = setup_test_db().await;
		let ctx = setup_test_ctx();
		let mm = ModelManager::new(pool.clone());

		let user_c = UserForCreate {
			username: "testuser".to_string(),
			pwd_clear: "securepassword".to_string(),
			email: "test@example.com".to_string(),
			assigned_role: "user".to_string(),
		};

		let user_id = UserBmc::create(&ctx, &mm, user_c).await.unwrap();
		UserBmc::update_pwd(&ctx, &mm, user_id, "newpassword")
			.await
			.unwrap();

		let user = UserBmc::get::<UserForLogin>(&ctx, &mm, user_id)
			.await
			.unwrap();

		assert!(user.pwd.is_some());
		assert_ne!(user.pwd.unwrap(), "securepassword");

		pool.execute("DELETE FROM \"user\"").await.unwrap();
	}

	#[tokio::test]
	async fn test_update_user_details() {
		let pool = setup_test_db().await;
		let ctx = setup_test_ctx();
		let mm = ModelManager::new(pool.clone());

		let user_c = UserForCreate {
			username: "testuser".to_string(),
			pwd_clear: "securepassword".to_string(),
			email: "test@example.com".to_string(),
			assigned_role: "user".to_string(),
		};

		let user_id = UserBmc::create(&ctx, &mm, user_c).await.unwrap();

		let user_u = UserForUpdate {
			username: "updateduser".to_string(),
			email: "updated@example.com".to_string(),
			assigned_role: "admin".to_string(),
		};

		UserBmc::update(&ctx, &mm, user_id, user_u).await.unwrap();

		let updated_user = UserBmc::get::<User>(&ctx, &mm, user_id).await.unwrap();

		assert_eq!(updated_user.username, "updateduser");
		assert_eq!(updated_user.email, "updated@example.com");
		assert_eq!(updated_user.assigned_role, "admin");

		pool.execute("DELETE FROM \"user\"").await.unwrap();
	}

	#[tokio::test]
	async fn test_list_users() {
		let pool = setup_test_db().await;
		let ctx = setup_test_ctx();
		let mm = ModelManager::new(pool.clone());

		let user_c1 = UserForCreate {
			username: "user1".to_string(),
			pwd_clear: "password1".to_string(),
			email: "user1@example.com".to_string(),
			assigned_role: "user".to_string(),
		};

		let user_c2 = UserForCreate {
			username: "user2".to_string(),
			pwd_clear: "password2".to_string(),
			email: "user2@example.com".to_string(),
			assigned_role: "user".to_string(),
		};

		UserBmc::create(&ctx, &mm, user_c1).await.unwrap();
		UserBmc::create(&ctx, &mm, user_c2).await.unwrap();

		let users = UserBmc::list(&ctx, &mm).await.unwrap();

		assert_eq!(users.len(), 2);
		assert_eq!(users[0].username, "user1");
		assert_eq!(users[1].username, "user2");

		pool.execute("DELETE FROM \"user\"").await.unwrap();
	}

	#[tokio::test]
	async fn test_delete_user() {
		let pool = setup_test_db().await;
		let ctx = setup_test_ctx();
		let mm = ModelManager::new(pool.clone());

		let user_c = UserForCreate {
			username: "testuser".to_string(),
			pwd_clear: "securepassword".to_string(),
			email: "test@example.com".to_string(),
			assigned_role: "user".to_string(),
		};

		let user_id = UserBmc::create(&ctx, &mm, user_c).await.unwrap();
		UserBmc::delete(&ctx, &mm, user_id).await.unwrap();

		let user = UserBmc::get::<User>(&ctx, &mm, user_id).await;

		assert!(user.is_err()); // Debe devolver un error porque el usuario ya no existe

		pool.execute("DELETE FROM \"user\"").await.unwrap();
	}

	#[tokio::test]
	async fn test_first_by_username_not_found() {
		let pool = setup_test_db().await;
		let ctx = setup_test_ctx();
		let mm = ModelManager::new(pool.clone());

		let user = UserBmc::first_by_username::<User>(&ctx, &mm, "nonexistentuser")
			.await
			.unwrap();

		assert!(user.is_none());
	}
}
