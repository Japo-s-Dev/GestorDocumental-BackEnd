use crate::core::ctx::Ctx;
use crate::core::model::base::{self, DbBmc};
use crate::core::model::modql_utils::time_to_sea_value;
use crate::core::model::ModelManager;
use crate::core::model::Result;
use crate::utils::time::Rfc3339;
use modql::field::{Fields, HasFields};
use modql::filter::{
	FilterNodes, ListOptions, OpValsInt64, OpValsString, OpValsValue,
};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use sqlx::postgres::PgRow;
use sqlx::types::time::OffsetDateTime;
use sqlx::FromRow;

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize)]
pub struct Archive {
	pub id: i64,
	pub project_id: i64,
	pub owner: i64,
	pub last_edit_user: i64,
	pub tag: String,
	pub cid: i64,
	#[serde_as(as = "Rfc3339")]
	pub ctime: OffsetDateTime,
	pub mid: i64,
	#[serde_as(as = "Rfc3339")]
	pub mtime: OffsetDateTime,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct ArchiveForCreate {
	pub project_id: i64,
	pub tag: String,
}

#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct ArchiveForUpdate {
	pub tag: String,
}

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct ArchiveForInsertCreate {
	pub project_id: i64,
	pub owner: i64,
	pub last_edit_user: i64,
	pub tag: String,
}

#[serde_as]
#[derive(Clone, Fields, FromRow, Debug, Serialize, Deserialize)]
pub struct ArchiveForInsertUpdate {
	pub last_edit_user: i64,
	pub tag: String,
}

#[derive(FilterNodes, Deserialize, Default, Debug)]
pub struct ArchiveFilter {
	id: Option<OpValsInt64>,

	project_id: Option<OpValsInt64>,
	owner: Option<OpValsInt64>,
	last_edit_user: Option<OpValsInt64>,
	tag: Option<OpValsString>,
	cid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	ctime: Option<OpValsValue>,
	mid: Option<OpValsInt64>,
	#[modql(to_sea_value_fn = "time_to_sea_value")]
	mtime: Option<OpValsValue>,
}

#[allow(dead_code)]
pub trait ArchiveBy: HasFields + for<'r> FromRow<'r, PgRow> + Unpin + Send {}

impl ArchiveBy for Archive {}
impl ArchiveBy for ArchiveForCreate {}
impl ArchiveBy for ArchiveForUpdate {}
impl ArchiveBy for ArchiveForInsertCreate {}
impl ArchiveBy for ArchiveForInsertUpdate {}

pub struct ArchiveBmc;

impl DbBmc for ArchiveBmc {
	const TABLE: &'static str = "archive";
}

impl ArchiveBmc {
	pub async fn get(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<Archive> {
		base::get::<Self, _>(ctx, mm, id).await
	}

	pub async fn create(
		ctx: &Ctx,
		mm: &ModelManager,
		archive_op: ArchiveForCreate,
	) -> Result<i64> {
		let archive_insert = ArchiveForInsertCreate {
			owner: ctx.user_id(),
			project_id: archive_op.project_id,
			last_edit_user: ctx.user_id(),
			tag: archive_op.tag,
		};

		let archive_id = base::create::<Self, _>(ctx, mm, archive_insert).await?;

		Ok(archive_id)
	}

	pub async fn list(
		ctx: &Ctx,
		mm: &ModelManager,
		filters: Option<Vec<ArchiveFilter>>,
		list_options: Option<ListOptions>,
	) -> Result<Vec<Archive>> {
		base::list::<Self, _, _>(ctx, mm, filters, list_options).await
	}

	pub async fn update(
		ctx: &Ctx,
		mm: &ModelManager,
		id: i64,
		archive_op: ArchiveForUpdate,
	) -> Result<()> {
		let archive_insert = ArchiveForInsertUpdate {
			last_edit_user: ctx.user_id(),
			tag: archive_op.tag,
		};

		base::update::<Self, _>(ctx, mm, id, archive_insert).await
	}

	pub async fn delete(ctx: &Ctx, mm: &ModelManager, id: i64) -> Result<()> {
		base::delete::<Self>(ctx, mm, id).await
	}
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ctx::Ctx;
    use crate::core::model::{ModelManager, Result};
    use crate::core::model::store::new_db_pool;
    use sqlx::{Pool, Postgres};

    // Esta función prepara el contexto y el ModelManager para las pruebas
    async fn setup() -> (Ctx, ModelManager) {
        // Creación del pool de la base de datos
        let pool = new_db_pool().await.unwrap(); // Asume que el pool se inicializa correctamente

        // Inicializa el ModelManager con el pool de conexiones
        let mm = ModelManager::new().await.unwrap();

        // Crea un contexto de prueba, por ejemplo, con un user_id ficticio
        let ctx = Ctx::new(1).unwrap(); // Ajustar según la implementación correcta de Ctx

        (ctx, mm)
    }

    #[tokio::test]
    async fn test_create_archive() {
        let (ctx, mm) = setup().await;

        // Datos de prueba para crear un archivo
        let archive_data = ArchiveForCreate {
            project_id: 1, // Cambia según tus datos de prueba
            tag: "Nuevo archivo de prueba".to_string(),
        };

        let result = ArchiveBmc::create(&ctx, &mm, archive_data).await;
        assert!(result.is_ok(), "La creación del archivo debería ser exitosa");
        let archive_id = result.unwrap();

        assert!(archive_id > 0, "El ID del archivo debería ser mayor a 0");
    }

    #[tokio::test]
    async fn test_get_archive() {
        let (ctx, mm) = setup().await;

        // ID del archivo de prueba
        let archive_id = 1; // Cambia esto según los datos existentes en tu base de datos

        let result = ArchiveBmc::get(&ctx, &mm, archive_id).await;
        assert!(result.is_ok(), "Debería obtener el archivo correctamente");

        let archive = result.unwrap();
        assert_eq!(archive.id, archive_id, "El ID del archivo debería coincidir");
    }

    #[tokio::test]
    async fn test_list_archives() {
        let (ctx, mm) = setup().await;

        // Sin filtros para esta prueba
        let filters: Option<Vec<ArchiveFilter>> = None;
        let list_options: Option<ListOptions> = None;

        let result = ArchiveBmc::list(&ctx, &mm, filters, list_options).await;
        assert!(result.is_ok(), "Debería listar los archivos correctamente");

        let archives = result.unwrap();
        assert!(!archives.is_empty(), "La lista de archivos no debería estar vacía");
    }

    #[tokio::test]
    async fn test_update_archive() {
        let (ctx, mm) = setup().await;

        // ID del archivo a actualizar
        let archive_id = 1; // Cambia esto según los datos existentes en tu base de datos

        // Datos de prueba para actualizar
        let update_data = ArchiveForUpdate {
            tag: "Etiqueta actualizada".to_string(),
        };

        let result = ArchiveBmc::update(&ctx, &mm, archive_id, update_data).await;
        assert!(result.is_ok(), "La actualización del archivo debería ser exitosa");
    }

    #[tokio::test]
    async fn test_delete_archive() {
        let (ctx, mm) = setup().await;

        // ID del archivo a eliminar
        let archive_id = 1; // Cambia esto según los datos existentes en tu base de datos

        let result = ArchiveBmc::delete(&ctx, &mm, archive_id).await;
        assert!(result.is_ok(), "La eliminación del archivo debería ser exitosa");
    }
}
