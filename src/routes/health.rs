use crate::models::HealthResponse;
use actix_web::{HttpResponse, Responder, get};

/// # Health Check Endpoint
///
/// Returns the current health status of the service along with a timestamp.
///
/// ## Response
///
/// - **200 OK**: Service is healthy
///   - Body: JSON object with `status` ("UP") and `timestamp` in ISO 8601 format
///
/// ## Example Response
///
/// ```json
/// {
///   "status": "UP",
///   "timestamp": "2023-10-05T12:34:56.789Z"
/// }
/// ```
#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse::up())
}

/// # Route Configuration
///
/// Registers all API endpoints with the Actix-web service configuration.
///
/// ## Currently Configured Routes
///
/// - `GET /health`: Health check endpoint
pub fn configure_routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(health);
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, test};
    use serde_json::from_str;

    /// Health endpoint test suite
    #[actix_web::test]
    async fn test_health_endpoint() {
        // Set up test app
        let app = test::init_service(App::new().configure(configure_routes)).await;

        // Create test request
        let req = test::TestRequest::get().uri("/health").to_request();

        // Execute request
        let resp = test::call_service(&app, req).await;

        // Verify status code
        assert!(resp.status().is_success());

        // Verify response body
        let body = test::read_body(resp).await;
        let body_str = std::str::from_utf8(&body).unwrap();
        let health_response: HealthResponse = from_str(body_str).unwrap();

        assert_eq!(health_response.status, "UP");

        // Verify timestamp is present (more thorough validation in model tests)
        assert!(!health_response.timestamp.is_empty());
    }
}
