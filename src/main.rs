use actix_web::{App, HttpServer, web::Data};
use email_sanitizer::graphql::schema::create_schema;
use email_sanitizer::openapi::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

/// Email Sanitizer Service Entry Point
///
/// Configures and launches the Actix-web HTTP server with:
/// - GraphQL endpoint powered by Async-GraphQL
/// - Swagger UI for API documentation
/// - Environment configuration via `.env` file
/// - Shared application state for schema and OpenAPI docs
///
/// # Endpoints
/// - GraphQL: `/api/v1/graphql` (configured in routes)
/// - Swagger UI: `/swagger-ui/`
/// - OpenAPI spec: `/api-docs/openapi.json`
///
/// # Configuration
/// - Server binds to `127.0.0.1:8080` by default
/// - Environment variables loaded from `.env` file (if present)
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    // Create GraphQL schema
    let schema = create_schema();

    HttpServer::new(move || {
        let openapi = ApiDoc::openapi();

        App::new()
            .app_data(Data::new(openapi.clone()))
            .app_data(Data::new(schema.clone()))
            .configure(email_sanitizer::routes::configure)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    use actix_web::{App, test, web::Data};
    use email_sanitizer::graphql::schema::create_schema;
    use email_sanitizer::openapi::ApiDoc;
    use utoipa::OpenApi;
    use utoipa_swagger_ui::SwaggerUi;

    #[actix_web::test]
    async fn test_server_configuration() {
        // Create the app with the same configuration as in main()
        let schema = create_schema();
        let openapi = ApiDoc::openapi();

        let _app = test::init_service(
            App::new()
                .app_data(Data::new(openapi.clone()))
                .app_data(Data::new(schema.clone()))
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

        let app = test::init_service(
            App::new()
                .app_data(Data::new(openapi.clone()))
                .app_data(Data::new(schema.clone()))
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

        let app = test::init_service(
            App::new()
                .app_data(Data::new(openapi.clone()))
                .app_data(Data::new(schema.clone()))
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

        let app = test::init_service(
            App::new()
                .app_data(Data::new(openapi.clone()))
                .app_data(Data::new(schema.clone()))
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

        let app = test::init_service(
            App::new()
                .app_data(Data::new(openapi.clone()))
                .app_data(Data::new(schema.clone()))
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

        let app = test::init_service(
            App::new()
                .app_data(Data::new(openapi.clone()))
                .app_data(Data::new(schema.clone()))
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
}
