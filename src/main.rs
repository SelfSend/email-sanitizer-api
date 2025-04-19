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
