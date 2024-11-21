use crate::core::ctx::Ctx;
use crate::core::model::base::{self, DbBmc};
use crate::core::model::modql_utils::time_to_sea_value;
use crate::core::model::ModelManager;
use crate::core::model::Result;
use crate::utils::time::Rfc3339;
use modql::field::{Fields, HasFields};
use modql::filter::{FilterNodes, ListOptions, OpValsInt64, OpValsValue};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::postgres::PgRow;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

use super::base::ListResult;

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct ArchiveComment {
	pub id: i64,
	pub archive_id: i64,
	pub text: String,
	pub user_id: i64,
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct ArchiveCommentForOp {
	pub text: String,
	pub archive_id: i64,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct ArchiveCommentForOpInsert {
	pub text: String,
	pub archive_id: i64,
	pub user_id: i64,
}

#[allow(dead_code)]
pub trait ArchiveCommentBy:
	HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send
{
}

impl ArchiveCommentBy for ArchiveComment {}
impl ArchiveCommentBy for ArchiveCommentForOp {}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct ArchiveCommentFilter {
	id: Option<OpValsInt64>,

	archive_id: Option<OpValsInt64>,
	user_id: Option<OpValsInt64>,
	cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	ctime: Option<OpValsValue>,
	mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	mtime: Option<OpValsValue>,
}

pub struct ArchiveCommentBmc;

impl DbBmc for ArchiveCommentBmc {
	const TABLE: &'static str = "archive_comment";
	const TIMESTAMPED: bool = true;
	const SOFTDELETED: bool = true;
}

impl ArchiveCommentBmc {
	pub async fn get(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
	) -> Result<ArchiveComment> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		comment_c: ArchiveCommentForOp,
	) -> Result<i64> {
		let comment_data = ArchiveCommentForOpInsert {
			archive_id: comment_c.archive_id,
			text: comment_c.text,
			user_id: ctx.user_id(),
		};

		let comment_id = base::create::<Self, _>(ctx, mm, comment_data).await?;

		Ok(comment_id)
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filters: Option<Vec<ArchiveCommentFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<ListResult<ArchiveComment>> {
		base::list::<Self, _, _>(ctx, mm, filters, list_options).await
	}

	#[allow(unused)]
	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		comment_u: ArchiveCommentForOp,
	) -> Result<()> {
		base::update::<Self, _>(ctx, mm, id, comment_u).await
	}
	#[allow(unused)]
	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ctx::Ctx;
    use crate::core::model::{ModelManager, Result};
    use crate::core::model::base::DbBmc;
    use crate::core::model::store::new_db_pool;
    use sqlx::{Pool, Postgres};

	async fn setup() -> (Ctx, ModelManager) {
		// Creación del pool de la base de datos
		let pool: Pool<Postgres> = new_db_pool().await.unwrap(); // Usamos unwrap para simplificar en las pruebas
	
		// Inicializamos `ModelManager` con el pool de la base de datos
		let mm = ModelManager::new().await.unwrap(); // Ajustar según la implementación correcta de `ModelManager::new`
	
		// Inicialización de `Ctx` con el argumento necesario. Ajusta el valor de `user_id` según tu implementación.
		let ctx = Ctx::new(1).unwrap(); // Se pasa `1` como ID de usuario o lo que sea necesario.
	
		(ctx, mm)
	}

    #[tokio::test]
    async fn test_create_archive_comment() {
        let (ctx, mm) = setup().await;

        // Datos de prueba para el comentario
        let comment = ArchiveCommentForOp {
            text: "Este es un comentario de prueba".to_string(),
            archive_id: 1, // Cambia este ID según tus datos de prueba
        };

        let result = ArchiveCommentBmc::create(&ctx, &mm, comment).await;
        assert!(result.is_ok(), "La creación del comentario debería ser exitosa");
    }

    #[tokio::test]
    async fn test_get_archive_comment() {
        let (ctx, mm) = setup().await;

        // ID del comentario de prueba
        let comment_id = 1; // Cambia esto según tus datos

        let result = ArchiveCommentBmc::get(&ctx, &mm, comment_id).await;
        assert!(result.is_ok(), "Debería obtener el comentario correctamente");

        let comment = result.unwrap();
        assert_eq!(comment.id, comment_id, "El ID del comentario debería coincidir");
    }

    #[tokio::test]
    async fn test_update_archive_comment() {
        let (ctx, mm) = setup().await;

        // ID del comentario a actualizar
        let comment_id = 1; // Cambia esto según tus datos

        // Datos actualizados para el comentario
        let updated_comment = ArchiveCommentForOp {
            text: "Este es un comentario actualizado".to_string(),
            archive_id: 1, // Cambia esto según tus datos
        };

        let result = ArchiveCommentBmc::update(&ctx, &mm, comment_id, updated_comment).await;
        assert!(result.is_ok(), "La actualización del comentario debería ser exitosa");
    }

    #[tokio::test]
    async fn test_delete_archive_comment() {
        let (ctx, mm) = setup().await;

        // ID del comentario a eliminar
        let comment_id = 1; // Cambia esto según tus datos

        let result = ArchiveCommentBmc::delete(&ctx, &mm, comment_id).await;
        assert!(result.is_ok(), "La eliminación del comentario debería ser exitosa");
    }

    #[tokio::test]
    async fn test_list_archive_comments() {
        let (ctx, mm) = setup().await;

        let filters: Option<Vec<ArchiveCommentFilter>> = None; // Sin filtros en esta prueba
        let list_options: Option<ListOptions> = None; // Opciones por defecto

        let result = ArchiveCommentBmc::list(&ctx, &mm, filters, list_options).await;
        assert!(result.is_ok(), "Debería listar los comentarios correctamente");

        let comments = result.unwrap();
        assert!(!comments.is_empty(), "Debería haber al menos un comentario en la lista");
    }
}
