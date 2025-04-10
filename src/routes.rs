use crate::models::HealthResponse;
use actix_web::{HttpResponse, Responder, get};

#[get("/health")]
pub async fn health() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse::up())
}

pub fn configure_routes(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(health);
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, test};
    use serde_json::from_str;

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
