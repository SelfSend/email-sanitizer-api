use actix_web::{App, HttpServer, web::Data};
use email_sanitizer::openapi::ApiDoc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    HttpServer::new(|| {
        let openapi = ApiDoc::openapi();

        App::new()
            .app_data(Data::new(openapi.clone()))
            .configure(email_sanitizer::routes::configure)
            .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
