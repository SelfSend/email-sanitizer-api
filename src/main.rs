use actix_web::{App, HttpServer, web::Data};
use email_sanitizer::graphql::schema::create_schema;
use email_sanitizer::openapi::ApiDoc;
use email_sanitizer::routes::email::RedisCache;
use std::env::VarError;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// Email Sanitizer Service Entry Point
///
/// Configures and launches the Actix-web HTTP server with:
/// - GraphQL endpoint powered by Async-GraphQL
/// - REST endpoints for email validation with Redis caching
/// - Swagger UI for API documentation
/// - Environment configuration via `.env` file
/// - Shared application state for schema, Redis cache, and OpenAPI docs
///
/// # Endpoints
/// - GraphQL: `/api/v1/graphql` (configured in routes)
/// - Email validation: `/api/v1/validate-email`
/// - Swagger UI: `/swagger-ui/`
/// - OpenAPI spec: `/api-docs/openapi.json`
///
/// # Configuration
/// - Server binds to `127.0.0.1:8080` by default. Port can be specified as an env variable named "PORT".
/// - Environment variables loaded from `.env` file (if present)
/// - Redis URL from REDIS_URL environment variable (defaults to localhost:6379)
/// - Redis cache TTL from REDIS_CACHE_TTL environment variable (defaults to 86400 seconds/24 hours)
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    // Initialize Redis cache
    let redis_url =
        std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    let redis_ttl = std::env::var("REDIS_CACHE_TTL")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(86400); // Default 24 hours TTL

    let redis_cache =
        RedisCache::new(&redis_url, redis_ttl).expect("Failed to initialize Redis connection");

    // Create GraphQL schema
    let schema = create_schema();

    let port: Result<String, VarError> = std::env::var("PORT");
    let port = match port {
        Ok(v) => v,
        Err(e) => {
            eprintln!(
                "Error reading PORT environment variable: {}, binding to 8080",
                e
            );
            "8080".to_string()
        }
    };

    HttpServer::new(move || {
        let openapi = ApiDoc::openapi();

        App::new()
            .app_data(Data::new(openapi.clone()))
            .app_data(Data::new(schema.clone()))
            .app_data(Data::new(redis_cache.clone()))
            .configure(email_sanitizer::routes::configure)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi))
    })
    .bind((
        "0.0.0.0",  // Changed from 127.0.0.1 to allow external connections
        port.parse::<u16>().expect("Failed to parse port"),
    ))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{App, test, web::Data};
    use email_sanitizer::graphql::schema::create_schema;
    use email_sanitizer::openapi::ApiDoc;
    use email_sanitizer::routes::email::RedisCache;
    use utoipa::OpenApi;
    use utoipa_swagger_ui::SwaggerUi;

    // Helper function to create test Redis cache
    fn create_test_redis_cache() -> RedisCache {
        let redis_url = std::env::var("TEST_REDIS_URL")
            .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());

        RedisCache::new(&redis_url, 3600) // 1 hour TTL for tests
            .expect("Failed to initialize test Redis connection")
    }

    #[actix_web::test]
    async fn test_server_configuration() {
        // Create the app with the same configuration as in main()
        let schema = create_schema();
        let openapi = ApiDoc::openapi();
        let redis_cache = create_test_redis_cache();

        let _app = test::init_service(
            App::new()
                .app_data(Data::new(openapi.clone()))
                .app_data(Data::new(schema.clone()))
                .app_data(Data::new(redis_cache.clone()))
                .configure(email_sanitizer::routes::configure)
                .service(
                    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi),
                ),
        )
        .await;

        // The app initializes successfully if we get here
    }

    #[actix_web::test]
    async fn test_openapi_docs_endpoint() {
        // Set up test app with the same configuration as in main()
        let schema = create_schema();
        let openapi = ApiDoc::openapi();
        let redis_cache = create_test_redis_cache();

        let app = test::init_service(
            App::new()
                .app_data(Data::new(openapi.clone()))
                .app_data(Data::new(schema.clone()))
                .app_data(Data::new(redis_cache.clone()))
                .configure(email_sanitizer::routes::configure)
                .service(
                    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi),
                ),
        )
        .await;

        // Create a test request to the OpenAPI JSON endpoint
        let req = test::TestRequest::get()
            .uri("/api-docs/openapi.json")
            .to_request();

        // Execute request
        let resp = test::call_service(&app, req).await;

        // Assert response is successful
        assert!(
            resp.status().is_success(),
            "OpenAPI docs endpoint should return a successful status code"
        );

        // Get the response body and verify it's valid JSON
        let body = test::read_body(resp).await;
        let json_result = serde_json::from_slice::<serde_json::Value>(&body);
        assert!(
            json_result.is_ok(),
            "OpenAPI docs endpoint should return valid JSON"
        );
    }

    #[actix_web::test]
    async fn test_swagger_ui_endpoint() {
        // Set up test app with the same configuration as in main()
        let schema = create_schema();
        let openapi = ApiDoc::openapi();
        let redis_cache = create_test_redis_cache();

        let app = test::init_service(
            App::new()
                .app_data(Data::new(openapi.clone()))
                .app_data(Data::new(schema.clone()))
                .app_data(Data::new(redis_cache.clone()))
                .configure(email_sanitizer::routes::configure)
                .service(
                    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi),
                ),
        )
        .await;

        // Create a test request to the Swagger UI endpoint
        let req = test::TestRequest::get().uri("/swagger-ui/").to_request();

        // Execute request
        let resp = test::call_service(&app, req).await;

        // Assert response is successful
        assert!(
            resp.status().is_success(),
            "Swagger UI endpoint should return a successful status code"
        );

        // Verify response is HTML
        let content_type = resp
            .headers()
            .get("content-type")
            .and_then(|value| value.to_str().ok())
            .unwrap_or("");

        assert!(
            content_type.contains("text/html"),
            "Swagger UI should return HTML content, got: {}",
            content_type
        );
    }

    #[actix_web::test]
    async fn test_graphql_endpoint_exists() {
        // Set up test app with the same configuration as in main()
        let schema = create_schema();
        let openapi = ApiDoc::openapi();
        let redis_cache = create_test_redis_cache();

        let app = test::init_service(
            App::new()
                .app_data(Data::new(openapi.clone()))
                .app_data(Data::new(schema.clone()))
                .app_data(Data::new(redis_cache.clone()))
                .configure(email_sanitizer::routes::configure)
                .service(
                    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi),
                ),
        )
        .await;

        // Create a POST request to the GraphQL endpoint
        let req = test::TestRequest::post()
            .uri("/api/v1/graphql")
            .insert_header(("content-type", "application/json"))
            .set_payload(r#"{"query":"{ health { status } }"}"#)
            .to_request();

        // Execute request
        let resp = test::call_service(&app, req).await;

        // Assert response is either successful or a 400 (if query is invalid)
        // We're just testing the endpoint exists, not full GraphQL functionality
        assert!(
            resp.status().is_success() || resp.status().as_u16() == 400,
            "GraphQL endpoint should exist and return either 200 or 400, got: {}",
            resp.status()
        );
    }

    #[actix_web::test]
    async fn test_graphql_playground_endpoint() {
        // Set up test app with the same configuration as in main()
        let schema = create_schema();
        let openapi = ApiDoc::openapi();
        let redis_cache = create_test_redis_cache();

        let app = test::init_service(
            App::new()
                .app_data(Data::new(openapi.clone()))
                .app_data(Data::new(schema.clone()))
                .app_data(Data::new(redis_cache.clone()))
                .configure(email_sanitizer::routes::configure)
                .service(
                    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi),
                ),
        )
        .await;

        // Create a test request to the GraphQL playground endpoint
        let req = test::TestRequest::get()
            .uri("/api/v1/playground")
            .to_request();

        // Execute request
        let resp = test::call_service(&app, req).await;

        // Assert response is successful
        assert!(
            resp.status().is_success(),
            "GraphQL playground endpoint should return a successful status code, got: {}",
            resp.status()
        );

        // Verify response is HTML
        let content_type = resp
            .headers()
            .get("content-type")
            .and_then(|value| value.to_str().ok())
            .unwrap_or("");

        assert!(
            content_type.contains("text/html"),
            "GraphQL playground should return HTML content, got: {}",
            content_type
        );
    }

    #[actix_web::test]
    async fn test_health_endpoint() {
        // Set up test app with the same configuration as in main()
        let schema = create_schema();
        let openapi = ApiDoc::openapi();
        let redis_cache = create_test_redis_cache();

        let app = test::init_service(
            App::new()
                .app_data(Data::new(openapi.clone()))
                .app_data(Data::new(schema.clone()))
                .app_data(Data::new(redis_cache.clone()))
                .configure(email_sanitizer::routes::configure)
                .service(
                    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi),
                ),
        )
        .await;

        // Create a test request to the health endpoint
        let req = test::TestRequest::get().uri("/api/v1/health").to_request();

        // Execute request
        let resp = test::call_service(&app, req).await;

        // Assert response is successful
        assert!(
            resp.status().is_success(),
            "Health endpoint should return a successful status code"
        );

        // Get the response body and verify it's valid JSON
        let body = test::read_body(resp).await;
        let json_result = serde_json::from_slice::<serde_json::Value>(&body);
        assert!(
            json_result.is_ok(),
            "Health endpoint should return valid JSON"
        );

        // Check the specific health response fields
        let json = json_result.unwrap();
        assert_eq!(json["status"], "UP", "Health status should be 'UP'");
        assert!(
            json["timestamp"].is_string(),
            "Health response should include a timestamp"
        );
    }

    /* Dummy RedisCache for fallback in tests
    fn test_dummy() -> RedisCache {
        // You may want to implement a mock or in-memory version for real tests
        RedisCache::new("redis://127.0.0.1:6379", 1).expect("Failed to create dummy RedisCache")
    }
    // #[actix_web::test]
    async fn test_email_validation_endpoint() {
        // Set up test app with the same configuration as in main()
        let schema = create_schema();
        let openapi = ApiDoc::openapi();

        // Use test_dummy() directly to avoid Redis connection issues
        let redis_cache = test_dummy();

        let app = test::init_service(
            App::new()
                .app_data(Data::new(openapi.clone()))
                .app_data(Data::new(schema.clone()))
                .app_data(Data::new(redis_cache.clone()))
                .configure(email_sanitizer::routes::configure)
                .service(
                    SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi),
                ),
        )
        .await;

        // Test cases
        let test_cases = vec![
            // Valid email - should return 200 or 400 depending on DNS checks
            (
                serde_json::json!({"email": "test@example.com"}),
                vec![200, 400],
                "Valid email format",
            ),
            // Invalid email syntax - should return 400
            (
                serde_json::json!({"email": "invalid-email"}),
                vec![400],
                "Invalid email syntax",
            ),
            // Empty email - should return 400
            (serde_json::json!({"email": ""}), vec![400], "Empty email"),
            // Missing email field - should return 400
            (serde_json::json!({}), vec![400], "Missing email field"),
            // Invalid JSON - should return 400
            (
                serde_json::json!({"wrong_field": "test@example.com"}),
                vec![400],
                "Wrong JSON field",
            ),
        ];

        for (payload, expected_status_codes, test_name) in test_cases {
            let req = test::TestRequest::post()
                .uri("/api/v1/validate-email")
                .insert_header(("content-type", "application/json"))
                .set_json(payload)
                .to_request();

            let resp = test::call_service(&app, req).await;
            let status = resp.status().as_u16();

            if status == 500 {
                let body = test::read_body(resp).await;
                let body_str = std::str::from_utf8(&body).unwrap_or("Invalid UTF-8");
                panic!(
                    "{}: Expected status codes {:?}, got 500. Response body: {}",
                    test_name, expected_status_codes, body_str
                );
            }

            assert!(
                expected_status_codes.contains(&status),
                "{}: Expected status codes {:?}, got {}",
                test_name,
                expected_status_codes,
                status
            );

            // Additional validation for successful responses
            if status == 200 {
                let body = test::read_body(resp).await;
                let json: serde_json::Value =
                    serde_json::from_slice(&body).expect("Response should be valid JSON");

                assert!(
                    json.get("message").is_some(),
                    "Success response should contain 'message' field"
                );
            }
        }
    } */
}
