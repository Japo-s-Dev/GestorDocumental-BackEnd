mod error;

use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;
use tokio::sync::OnceCell;
use tracing::debug;

pub use self::error::{Error, Result};

static S3_CLIENT: OnceCell<Client> = OnceCell::const_new();

pub type Bucket = Client;

async fn new_s3_client() -> Result<Client> {
	let config = aws_config::load_defaults(BehaviorVersion::v2024_03_28()).await;

	let client = Client::new(&config);

	debug!("S3 client created successfully");

	Ok(client)
}

pub async fn get_s3_client() -> Result<Bucket> {
	S3_CLIENT
		.get_or_try_init(new_s3_client)
		.await
		.map(|client| client.clone())
		.map_err(|_| {
			Error::FailedToCreateClient("Failed to retrieve S3 client".to_string())
		})
}
#[cfg(test)]
mod tests {
    use super::*;
    use aws_sdk_s3::Client;
    use tokio::sync::OnceCell;
    use crate::core::model::bucket::{new_s3_client, get_s3_client, Error}; // Asegúrate de que la ruta a los módulos sea correcta

    #[tokio::test]
    async fn test_new_s3_client_creation() {
        let client_result = new_s3_client().await;

        // Verifica que el cliente se cree correctamente
        assert!(client_result.is_ok(), "El cliente S3 debería crearse correctamente");
    }


    #[tokio::test]
    async fn test_s3_client_error() {
        // Simular un error al crear el cliente S3
        // En esta prueba deberíamos provocar intencionadamente que new_s3_client falle
        let result = get_s3_client().await;

        // Verifica si el error que obtenemos es el que esperamos
        if let Err(Error::FailedToCreateClient(msg)) = result {
            assert_eq!(msg, "Failed to retrieve S3 client".to_string(), "El error debería corresponder con el mensaje esperado");
        } else {
            panic!("Debería haber fallado en la creación del cliente S3");
        }
    }
}
