mod error;

use std::time::Duration;

pub use self::error::{Error, Result};

use crate::core::config::core_config;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
	let max_connections = if cfg!(test) { 1 } else { 5 };

	let db_url = core_config().DB_URL.clone();

	// let ssl_db_url = format!("{}?sslmode=require", db_url);

	PgPoolOptions::new()
		.max_connections(max_connections)
		.acquire_timeout(Duration::from_millis(10000))
		.connect(&db_url)
		.await
		.map_err(|ex| Error::FailedToCreatePool(ex.to_string()))
}
#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::{Pool, Postgres};
    use crate::core::config::core_config; // Asegúrate de que esta ruta a core_config sea correcta.
    use sqlx::postgres::PgPoolOptions;
    use std::time::Duration;
    use crate::core::model::store::Error; // Asegúrate de que la ruta a los errores sea correcta.

    // Esta función devuelve una URL de base de datos de prueba
    fn get_test_db_url() -> String {
        "postgres://user:password@localhost/test_db".to_string() // Cambia esto según sea necesario
    }

    #[tokio::test]
    async fn test_new_db_pool_creation() {
        let db_url = get_test_db_url(); // Utilizamos la URL de prueba

        let pool_result = PgPoolOptions::new()
            .max_connections(1)
            .acquire_timeout(Duration::from_millis(10000))
            .connect(&db_url)
            .await;

        // Verificamos que el pool de conexiones se haya creado correctamente
        assert!(pool_result.is_ok(), "El pool de conexiones de la base de datos debería crearse correctamente");
    }

   
}
