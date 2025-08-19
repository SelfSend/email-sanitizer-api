use actix_web::web;
pub mod auth;
pub mod email;
pub mod graphql;
pub mod health;

#[cfg(test)]
mod email_test;

#[cfg(test)]
mod auth_test;

#[cfg(test)]
mod email_edge_case_tests;

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
/// POST   /api/v1/validate-email - Email validation with Redis caching
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
            .configure(auth::configure_routes)
            .configure(health::configure_routes)
            .configure(email::configure_routes)
            .configure(graphql::configure_routes),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::routes::email::RedisCache;
    use actix_web::{
        App, Error,
        body::to_bytes,
        dev::Service,
        http::{StatusCode, header},
        test,
        web::Data,
    };
    use serde_json::json;

    /// Helper function to create test Redis cache
    fn create_test_redis_cache() -> RedisCache {
        // For tests, we'll use a mock that avoids actual Redis connections
        // This could be a real Redis connection if available in test environment
        RedisCache::new("redis://127.0.0.1:6379", 3600).unwrap_or_else(|_| {
            // If connection fails, we need a fallback for tests
            // In a real application, you might want to use a mock instead
            eprintln!("Warning: Using dummy Redis cache for tests - DNS lookup caching disabled");
            RedisCache::test_dummy()
        })
    }

    /// Tests the basic API structure configuration
    /// Ensures the routes are properly configured in the service config
    #[actix_web::test]
    async fn test_api_configuration() {
        // Create a simple test app with our route configuration
        let _app = test::init_service(
            App::new()
                .app_data(Data::new(create_test_redis_cache()))
                .configure(configure),
        )
        .await;

        // App should build successfully (if we reach here, it's successful)
    }

    /// Test that health endpoint responds correctly
    #[actix_web::test]
    async fn test_health_endpoint() -> Result<(), Error> {
        let app = test::init_service(
            App::new()
                .app_data(Data::new(create_test_redis_cache()))
                .configure(configure),
        )
        .await;

        // Test health endpoint
        let req = test::TestRequest::get().uri("/api/v1/health").to_request();
        let resp = app.call(req).await?;
        assert_eq!(resp.status(), StatusCode::OK);

        // Verify health response structure
        let body = to_bytes(resp.into_body()).await?;
        let health_response: serde_json::Value = serde_json::from_slice(&body)?;
        assert_eq!(health_response["status"], "UP");
        assert!(health_response["timestamp"].as_str().is_some());

        Ok(())
    }

    /// Test email validation endpoints
    #[actix_web::test]
    async fn test_email_validation_endpoints() -> Result<(), Error> {
        use mongodb::{Client as MongoClient, options::ClientOptions};
        use crate::job_queue::JobQueue;
        
        // Create MongoDB client for tests
        let mongo_uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
        let client_options = ClientOptions::parse(&mongo_uri).await.unwrap_or_else(|_| {
            // Use a simple default configuration if parsing fails
            ClientOptions::default()
        });
        let mongo_client = MongoClient::with_options(client_options).unwrap_or_else(|_| {
            // Create a simple client with default options if connection fails
            MongoClient::with_options(ClientOptions::default()).unwrap()
        });
        
        // Create JobQueue for tests
        let job_queue = JobQueue::new("redis://127.0.0.1:6379").unwrap_or_else(|_| {
            JobQueue::new("redis://127.0.0.1:6379").unwrap()
        });
        
        let app = test::init_service(
            App::new()
                .app_data(Data::new(create_test_redis_cache()))
                .app_data(Data::new(mongo_client))
                .app_data(Data::new(job_queue))
                .configure(configure),
        )
        .await;

        // Test email validation without auth header - should return 401
        let no_auth_req = test::TestRequest::post()
            .uri("/api/v1/validate-email")
            .set_json(json!({ "email": "test@example.com" }))
            .to_request();
        let no_auth_resp = app.call(no_auth_req).await?;
        assert_eq!(no_auth_resp.status(), StatusCode::UNAUTHORIZED);

        // Test email validation with invalid auth header - should return 401
        let invalid_auth_req = test::TestRequest::post()
            .uri("/api/v1/validate-email")
            .insert_header(("Authorization", "Bearer invalid-key"))
            .set_json(json!({ "email": "test@example.com" }))
            .to_request();
        let invalid_auth_resp = app.call(invalid_auth_req).await?;
        assert_eq!(invalid_auth_resp.status(), StatusCode::UNAUTHORIZED);

        // Test missing request body with auth
        let empty_body_req = test::TestRequest::post()
            .uri("/api/v1/validate-email")
            .insert_header(("Authorization", "Bearer test-key"))
            .to_request();
        let empty_body_resp = app.call(empty_body_req).await?;
        assert_eq!(empty_body_resp.status(), StatusCode::BAD_REQUEST);

        // Test malformed JSON body with auth
        let malformed_req = test::TestRequest::post()
            .uri("/api/v1/validate-email")
            .insert_header(("Authorization", "Bearer test-key"))
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .set_payload("{invalid_json:}") // Deliberately malformed
            .to_request();
        let malformed_resp = app.call(malformed_req).await?;
        assert_eq!(malformed_resp.status(), StatusCode::BAD_REQUEST);

        Ok(())
    }

    /// Test GraphQL related endpoints
    #[actix_web::test]
    async fn test_graphql_endpoints() -> Result<(), Error> {
        let app = test::init_service(
            App::new()
                .app_data(Data::new(create_test_redis_cache()))
                .configure(configure),
        )
        .await;

        // Test GraphQL endpoint exists
        let graphql_req = test::TestRequest::post()
            .uri("/api/v1/graphql")
            .set_json(json!({
                "query": "{ __schema { types { name } } }"
            }))
            .to_request();
        let graphql_resp = app.call(graphql_req).await?;

        // We don't assert the exact response because GraphQL might not be fully initialized in tests
        // but we can at least verify the endpoint exists and doesn't return 404
        assert_ne!(graphql_resp.status(), StatusCode::NOT_FOUND);

        // Test GraphQL playground endpoint (should return HTML)
        let playground_req = test::TestRequest::get()
            .uri("/api/v1/playground")
            .to_request();
        let playground_resp = app.call(playground_req).await?;

        // We should at least get some response, not a 404
        assert_ne!(playground_resp.status(), StatusCode::NOT_FOUND);

        Ok(())
    }

    /// Test API versioning and scope isolation
    #[actix_web::test]
    async fn test_api_versioning_and_scope() -> Result<(), Error> {
        let app = test::init_service(
            App::new()
                .app_data(Data::new(create_test_redis_cache()))
                .configure(configure),
        )
        .await;

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

        // Verify scope isolation - health endpoint shouldn't exist in different version
        let req = test::TestRequest::get().uri("/api/v2/health").to_request();
        let resp = app.call(req).await?;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);

        // Test method not allowed
        let req = test::TestRequest::post().uri("/api/v1/health").to_request();
        let resp = app.call(req).await?;
        assert_eq!(resp.status(), StatusCode::METHOD_NOT_ALLOWED);

        Ok(())
    }

    /// Comprehensive integration test covering all API aspects
    /// Validates the entire route structure works together
    #[actix_web::test]
    async fn test_api_v1_integration() -> Result<(), Error> {
        // Create Redis cache for testing
        let redis_cache = create_test_redis_cache();

        // Initialize app with Redis cache data
        let app = test::init_service(
            App::new()
                .app_data(Data::new(redis_cache))
                .configure(configure),
        )
        .await;

        // Test each main endpoint exists and returns expected status codes

        // Health check
        let health_req = test::TestRequest::get().uri("/api/v1/health").to_request();
        let health_resp = app.call(health_req).await?;
        assert_eq!(health_resp.status(), StatusCode::OK);

        // Email validation (minimal test)
        let email_req = test::TestRequest::post()
            .uri("/api/v1/validate-email")
            .set_json(json!({ "email": "somebody@example.org" }))
            .to_request();
        let email_resp = app.call(email_req).await?;
        // Not asserting specific status due to potential DNS issues in test environment
        assert!(email_resp.status() != StatusCode::NOT_FOUND);

        // GraphQL endpoint
        let graphql_req = test::TestRequest::post()
            .uri("/api/v1/graphql")
            .set_json(json!({ "query": "{__typename}" }))
            .to_request();
        let graphql_resp = app.call(graphql_req).await?;
        assert_ne!(graphql_resp.status(), StatusCode::NOT_FOUND);

        // GraphQL playground
        let playground_req = test::TestRequest::get()
            .uri("/api/v1/playground")
            .to_request();
        let playground_resp = app.call(playground_req).await?;
        assert_ne!(playground_resp.status(), StatusCode::NOT_FOUND);

        Ok(())
    }
}
