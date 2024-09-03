use crate::ctx::Ctx;
use crate::model::base::{self, DbBmc};
use crate::model::ModelManager;
use crate::model::{Error, Result};
use serde::{Deserialize, Serialize};
use sqlb::{Fields, HasFields};
use sqlx::postgres::PgRow;
use sqlx::FromRow;
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Role {
	pub id: i64,
	pub role_name: String,
	pub description: String,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct RoleForOp {
	pub role_name: String,
	pub description: String,
}

pub trait RoleBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl RoleBy for Role {}
impl RoleBy for RoleForOp {}

pub struct RoleBmc;

impl DbBmc for RoleBmc {
	const TABLE: &'static str = "role";
}

impl RoleBmc {
	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Role> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn first_by_role_name<E>(
		_ctx: &Ctx,
		mm: &ModelManager,
		role_name: &str,
	) -> Result<Option<E>>
	where
		E: RoleBy,
	{
		let db = mm.db();

		let user = sqlb::select()
			.table(Self::TABLE)
			.and_where("role_name", "=", role_name)
			.fetch_optional::<_, E>(db)
			.await?;

		Ok(user)
	}

	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		role_c: RoleForOp,
	) -> Result<i64> {
		let role_id = base::create::<Self, _>(ctx, mm, role_c).await?;

		Ok(role_id)
	}

	pub async fn list(ctx: &Ctx, mm: &ModelManager) -> Result<Vec<Role>> {
		base::list::<Self, _>(ctx, mm).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		role_u: RoleForOp,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, role_u).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{PgPool, Executor};
    use crate::ctx::Ctx;
    use crate::model::ModelManager;

    async fn setup_test_db() -> PgPool {
        let pool = PgPool::connect("postgres://user:password@localhost/test_db").await.unwrap();

        // Crear la tabla `role` para las pruebas
        pool.execute("CREATE TABLE IF NOT EXISTS \"role\" (
            id BIGSERIAL PRIMARY KEY,
            role_name VARCHAR NOT NULL,
            description VARCHAR NOT NULL
        )").await.unwrap();

        pool
    }

    fn setup_test_ctx() -> Ctx {
        Ctx::new(1).expect("Failed to create test Ctx")
    }

    #[tokio::test]
    async fn test_create_role() {
        let pool = setup_test_db().await;
        let ctx = setup_test_ctx();
        let mm = ModelManager::new(pool.clone());

        let role_c = RoleForOp {
            role_name: "admin".to_string(),
            description: "Administrator role".to_string(),
        };

        let role_id = RoleBmc::create(&ctx, &mm, role_c).await.unwrap();

        let role = RoleBmc::get(&ctx, &mm, role_id).await.unwrap();
        assert_eq!(role.role_name, "admin");
        assert_eq!(role.description, "Administrator role");

        pool.execute("DELETE FROM \"role\"").await.unwrap();
    }

    #[tokio::test]
    async fn test_get_role_by_id() {
        let pool = setup_test_db().await;
        let ctx = setup_test_ctx();
        let mm = ModelManager::new(pool.clone());

        let role_c = RoleForOp {
            role_name: "user".to_string(),
            description: "Regular user role".to_string(),
        };

        let role_id = RoleBmc::create(&ctx, &mm, role_c).await.unwrap();
        let role = RoleBmc::get(&ctx, &mm, role_id).await.unwrap();

        assert_eq!(role.id, role_id);
        assert_eq!(role.role_name, "user");

        pool.execute("DELETE FROM \"role\"").await.unwrap();
    }

    #[tokio::test]
    async fn test_get_role_by_role_name() {
        let pool = setup_test_db().await;
        let ctx = setup_test_ctx();
        let mm = ModelManager::new(pool.clone());

        let role_c = RoleForOp {
            role_name: "editor".to_string(),
            description: "Editor role".to_string(),
        };

        RoleBmc::create(&ctx, &mm, role_c).await.unwrap();
        let role = RoleBmc::first_by_role_name::<Role>(&ctx, &mm, "editor").await.unwrap().unwrap();

        assert_eq!(role.role_name, "editor");

        pool.execute("DELETE FROM \"role\"").await.unwrap();
    }

    #[tokio::test]
    async fn test_update_role() {
        let pool = setup_test_db().await;
        let ctx = setup_test_ctx();
        let mm = ModelManager::new(pool.clone());

        let role_c = RoleForOp {
            role_name: "user".to_string(),
            description: "Regular user role".to_string(),
        };

        let role_id = RoleBmc::create(&ctx, &mm, role_c).await.unwrap();

        let role_u = RoleForOp {
            role_name: "updated_user".to_string(),
            description: "Updated user role".to_string(),
        };

        RoleBmc::update(&ctx, &mm, role_id, role_u).await.unwrap();

        let updated_role = RoleBmc::get(&ctx, &mm, role_id).await.unwrap();

        assert_eq!(updated_role.role_name, "updated_user");
        assert_eq!(updated_role.description, "Updated user role");

        pool.execute("DELETE FROM \"role\"").await.unwrap();
    }

    #[tokio::test]
    async fn test_list_roles() {
        let pool = setup_test_db().await;
        let ctx = setup_test_ctx();
        let mm = ModelManager::new(pool.clone());

        let role_c1 = RoleForOp {
            role_name: "role1".to_string(),
            description: "Description for role1".to_string(),
        };

        let role_c2 = RoleForOp {
            role_name: "role2".to_string(),
            description: "Description for role2".to_string(),
        };

        RoleBmc::create(&ctx, &mm, role_c1).await.unwrap();
        RoleBmc::create(&ctx, &mm, role_c2).await.unwrap();

        let roles = RoleBmc::list(&ctx, &mm).await.unwrap();

        assert_eq!(roles.len(), 2);
        assert_eq!(roles[0].role_name, "role1");
        assert_eq!(roles[1].role_name, "role2");

        pool.execute("DELETE FROM \"role\"").await.unwrap();
    }

    #[tokio::test]
    async fn test_delete_role() {
        let pool = setup_test_db().await;
        let ctx = setup_test_ctx();
        let mm = ModelManager::new(pool.clone());

        let role_c = RoleForOp {
            role_name: "role_to_delete".to_string(),
            description: "Role that will be deleted".to_string(),
        };

        let role_id = RoleBmc::create(&ctx, &mm, role_c).await.unwrap();
        RoleBmc::delete(&ctx, &mm, role_id).await.unwrap();

        let role = RoleBmc::get(&ctx, &mm, role_id).await;

        assert!(role.is_err()); // Debe devolver un error porque el rol ya no existe

        pool.execute("DELETE FROM \"role\"").await.unwrap();
    }

    #[tokio::test]
    async fn test_first_by_role_name_not_found() {
        let pool = setup_test_db().await;
        let ctx = setup_test_ctx();
        let mm = ModelManager::new(pool.clone());

        let role = RoleBmc::first_by_role_name::<Role>(&ctx, &mm, "nonexistentrole").await.unwrap();

        assert!(role.is_none());
    }
}

