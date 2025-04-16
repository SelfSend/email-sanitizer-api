use actix_web::{App, HttpServer};
use email_sanitizer::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(routes::configure))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
