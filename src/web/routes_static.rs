use axum::handler::HandlerWithoutStateExt;
use axum::http::StatusCode;
use axum::routing::{any_service, MethodRouter};
use tower_http::services::ServeDir;

use crate::web_config;

// Note: Here we can just return a MethodRouter rather than a full Router
//       since ServeDir is a service.
pub fn serve_dir() -> MethodRouter {
	async fn handle_404() -> (StatusCode, &'static str) {
		(StatusCode::NOT_FOUND, "Resource not found")
	}

	any_service(
		ServeDir::new(&web_config().WEB_FOLDER)
			.not_found_service(handle_404.into_service()),
	)
}
#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt; // for `oneshot`

    #[tokio::test]
    async fn test_serve_existing_file() {
        // Set up the service
        let service = serve_dir();

        // Replace "test_file.html" with an actual file path in your WEB_FOLDER for the test
        let request = Request::builder()
            .uri("/test_file.html")  // Adjust to match an actual file path in the WEB_FOLDER
            .body(Body::empty())
            .unwrap();

        let response = service.oneshot(request).await.unwrap();

        // Check if the response status is 200 OK for an existing file
        assert_eq!(response.status(), StatusCode::OK);
    }
	

    #[tokio::test]
    async fn test_serve_non_existing_file() {
        // Set up the service
        let service = serve_dir();

        // Request a non-existing file
        let request = Request::builder()
            .uri("/non_existing_file.html")
            .body(Body::empty())
            .unwrap();

        let response = service.oneshot(request).await.unwrap();

        // Check if the response status is 404 NOT FOUND
        assert_eq!(response.status(), StatusCode::NOT_FOUND);

        // Optionally, verify the response body for the custom 404 message
   
  
    }
}
