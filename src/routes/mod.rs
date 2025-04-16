use actix_web::web;

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
pub mod health;

/// # Email Validation Endpoint
///
/// Validates an email address by checking three aspects:
/// 1. RFC-compliant syntax validation
/// 2. Domain DNS/MX record verification
/// 3. Disposable email domain check
///
/// ## Request
/// - Method: POST
/// - Body: JSON object with `email` field
///
/// ## Responses
/// - **200 OK**: Email is valid
/// - **400 Bad Request**:
///   - Invalid email syntax
///   - Domain has no valid MX/A/AAAA records
///   - Disposable email detected
/// - **500 Internal Server Error**: Database connection failed
///
/// ## Example Request
/// ```json
/// { "email": "user@example.com" }
/// ```
pub mod email;

/// # API Route Configuration
///
/// Sets up versioned API endpoints under the `/api/v1` base path.
///
/// ## API Version
/// - Version: 1.0
/// - Base Path: `/api/v1`
///
/// ## Mounted Services
/// - Health check endpoints (see [`health::configure_routes`] for details)
/// - Email validation endpoints (see [`email::configure_routes`] for details)
///
/// ## Example Endpoints
///
/// ```text
/// GET /api/v1/health - Service health status
/// POST /api/v1/validate-email - Email validation endpoint
/// ```
///
/// [`health::configure_routes`]: crate::routes::health::configure_routes
/// [`email::configure_routes`]: crate::routes::email::configure_routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(health::configure_routes)
            .configure(email::configure_routes),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, Error, body::to_bytes, dev::Service, http::StatusCode, test, web};
    use serde_json::json;

    #[actix_web::test]
    async fn test_api_v1_scope() -> Result<(), Error> {
        let app = test::init_service(App::new().configure(configure)).await;

        // Test health endpoint
        let req = test::TestRequest::get().uri("/api/v1/health").to_request();
        let resp = app.call(req).await?;
        assert_eq!(resp.status(), StatusCode::OK);

        // Verify health response structure
        let body = to_bytes(resp.into_body()).await?;
        let health_response: serde_json::Value = serde_json::from_slice(&body)?;
        assert_eq!(health_response["status"], "UP");
        assert!(health_response["timestamp"].as_str().is_some());

        // Test valid email validation request
        let valid_req = test::TestRequest::post()
            .uri("/api/v1/validate-email")
            .set_json(json!({ "email": "test@example.com" }))
            .to_request();
        let valid_resp = app.call(valid_req).await?;
        assert!(valid_resp.status().is_success());

        // Test invalid email syntax
        let invalid_syntax_req = test::TestRequest::post()
            .uri("/api/v1/validate-email")
            .set_json(json!({ "email": "invalid-email" }))
            .to_request();
        let invalid_syntax_resp = app.call(invalid_syntax_req).await?;
        assert_eq!(invalid_syntax_resp.status(), StatusCode::BAD_REQUEST);

        // Test missing request body
        let empty_body_req = test::TestRequest::post()
            .uri("/api/v1/validate-email")
            .to_request();
        let empty_body_resp = app.call(empty_body_req).await?;
        assert_eq!(empty_body_resp.status(), StatusCode::BAD_REQUEST);

        // Test non-existent endpoint within scope
        let req = test::TestRequest::get()
            .uri("/api/v1/nonexistent")
            .to_request();
        let resp = app.call(req).await?;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        // Verify scope isolation - health endpoint shouldn't exist outside /api/v1
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = app.call(req).await?;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        Ok(())
    }
}
