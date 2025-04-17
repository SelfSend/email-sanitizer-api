use crate::models::health::HealthResponse;
use actix_web::{HttpResponse, Responder};

/// # Service Health Check Endpoint
///
/// Provides a liveness probe for the service, indicating whether the API is operational.
///
/// ## Response
///
/// - **200 OK**: Service is running and healthy
///   - Content-Type: `application/json`
///   - Body: [`HealthResponse`] containing:
///     - `status`: String indicating service status ("UP")
///     - `timestamp`: ISO 8601 timestamp of the check
///
/// ## Example Success Response
/// ```json
/// {
///   "status": "UP",
///   "timestamp": "2023-10-05T14:23:45.678Z"
/// }
/// ```
///
/// [`HealthResponse`]: crate::models::health::HealthResponse
pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json(HealthResponse::up())
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, test};
    use chrono::DateTime;
    use serde_json::Value;

    #[actix_web::test]
    async fn test_health_check() {
        // Arrange
        let app = test::init_service(App::new().service(
            actix_web::web::resource("/health").route(actix_web::web::get().to(health_check)),
        ))
        .await;
        let req = test::TestRequest::get().uri("/health").to_request();

        // Act
        let resp = test::call_service(&app, req).await;

        // Assert
        assert_eq!(resp.status(), 200, "Status code should be 200 OK");

        // Verify content type is application/json
        let content_type = resp
            .headers()
            .get("content-type")
            .expect("Content-Type header should be present");
        assert_eq!(
            content_type, "application/json",
            "Content-Type should be application/json"
        );

        // Extract and validate response body
        let body = test::read_body(resp).await;
        let body_str = String::from_utf8(body.to_vec()).expect("Body should be valid UTF-8");
        let body_json: Value = serde_json::from_str(&body_str).expect("Body should be valid JSON");

        // Check JSON structure
        assert_eq!(body_json["status"], "UP", "Status should be 'UP'");

        // Verify timestamp format
        let timestamp = body_json["timestamp"]
            .as_str()
            .expect("Timestamp should be a string");

        // Make sure the timestamp is a valid ISO 8601 date
        let _dt = DateTime::parse_from_rfc3339(timestamp)
            .expect("Timestamp should be a valid RFC 3339 / ISO 8601 date");
    }

    #[actix_web::test]
    async fn test_health_response_serialization() {
        // Create a health response and verify its serialized output
        let response = HealthResponse::up();

        // Convert to JSON
        let json = serde_json::to_value(&response).expect("Should serialize to JSON");

        // Verify structure
        assert_eq!(json["status"], "UP", "Status should be 'UP'");

        // Check timestamp format
        let timestamp = json["timestamp"]
            .as_str()
            .expect("Timestamp should be a string");

        // Make sure the timestamp is a valid ISO 8601 date
        let _dt = DateTime::parse_from_rfc3339(timestamp)
            .expect("Timestamp should be a valid RFC 3339 / ISO 8601 date");
    }
}
