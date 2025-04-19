use actix_web::web;

// Existing module imports
pub mod email;
pub mod health;
// Add new GraphQL routes
pub mod graphql;

/// Central API Route Configuration
///
/// Configures versioned API endpoints under the `/api/v1` namespace with:
/// - REST endpoints for health checks and email validation
/// - GraphQL API endpoints and playground
/// - Unified error handling across all routes
///
/// # API Versioning
/// - Current version: `1.0`
/// - Base path: `/api/v1`
///
/// # Mounted Services
/// - Health Monitoring: [`health::configure_routes`]
/// - Email Validation: [`email::configure_routes`]
/// - GraphQL Interface: [`graphql::configure_routes`]
///
/// # Endpoints Overview
/// ```text
/// GET    /api/v1/health       - Service health status
/// POST   /api/v1/validate-email - Email validation
/// POST   /api/v1/graphql      - GraphQL query endpoint
/// GET    /api/v1/playground   - Interactive GraphQL IDE
/// ```
///
/// # Architecture
/// Routes are organized in scope-based groups to:
/// - Enforce consistent API versioning
/// - Apply middleware at appropriate scopes
/// - Maintain separation of concerns between features
///
/// [`health::configure_routes`]: crate::routes::health::configure_routes
/// [`email::configure_routes`]: crate::routes::email::configure_routes
/// [`graphql::configure_routes`]: crate::routes::graphql::configure_routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(health::configure_routes)
            .configure(email::configure_routes)
            .configure(graphql::configure_routes),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, Error, body::to_bytes, dev::Service, http::StatusCode, test};
    use serde_json::json;

    /// Comprehensive route integration tests
    ///
    /// Validates:
    /// - Correct route mounting under /api/v1 scope
    /// - Endpoint response status codes
    /// - Response body structures
    /// - Error handling for invalid requests
    /// - Scope isolation preventing route leakage
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
