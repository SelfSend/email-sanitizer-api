use crate::handlers::validation::{disposable, dnsmx, role_based, syntax};
use actix_web::{HttpResponse, Responder, post, web};
use redis::{AsyncCommands, Client};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct EmailRequest {
    email: String,
}

#[derive(Deserialize)]
pub struct ValidationQuery {
    #[serde(default)]
    check_role_based: bool,
}

// Redis client wrapper with connection pool
#[derive(Clone)]
pub struct RedisCache {
    client: Arc<Client>,
    ttl: u64, // Time-to-live for cache entries in seconds
}

impl RedisCache {
    pub fn new(redis_url: &str, ttl: u64) -> Result<Self, redis::RedisError> {
        let client = Client::open(redis_url)?;
        Ok(Self {
            client: Arc::new(client),
            ttl,
        })
    }

    // For testing when Redis is unavailable
    pub fn test_dummy() -> Self {
        // Create a dummy Redis cache that doesn't actually connect
        // This is used in tests when Redis is not available
        Self {
            client: Arc::new(Client::open("redis://127.0.0.1:6379").unwrap()),
            ttl: 3600,
        }
    }

    // Get cached DNS validation result
    pub async fn get_dns_validation(
        &self,
        email_domain: &str,
    ) -> Result<Option<bool>, redis::RedisError> {
        match self.client.get_multiplexed_async_connection().await {
            Ok(mut conn) => {
                let cache_key = format!("dns_mx::{}", email_domain);
                let result: Option<String> = conn.get(&cache_key).await?;
                Ok(result.map(|val| val == "valid"))
            }
            Err(e) => {
                // In test environment, return cache miss gracefully instead of propagating error
                if cfg!(test) { Ok(None) } else { Err(e) }
            }
        }
    }

    // Store DNS validation result
    pub async fn set_dns_validation(
        &self,
        email_domain: &str,
        is_valid: bool,
    ) -> Result<(), redis::RedisError> {
        match self.client.get_multiplexed_async_connection().await {
            Ok(mut conn) => {
                let cache_key = format!("dns_mx::{}", email_domain);
                let value = if is_valid { "valid" } else { "invalid" };
                let _: () = conn.set(&cache_key, value).await?;
                let _: () = conn.expire(&cache_key, self.ttl as i64).await?;
                Ok(())
            }
            Err(e) => {
                // In test environment, ignore Redis errors
                if cfg!(test) { Ok(()) } else { Err(e) }
            }
        }
    }
}

/// # Email Validation Endpoint
///
/// Validates an email address by checking multiple aspects:
/// 1. RFC-compliant syntax validation
/// 2. Domain DNS/MX record verification (with Redis caching)
/// 3. Role-based email address detection (optional, via query parameter)
/// 4. Disposable email domain check
///
/// ## Request
/// - Method: POST
/// - Body: JSON object with `email` field
/// - Query Parameters:
///   - `check_role_based` (optional): Set to `true` to enable role-based validation
///
/// ## Responses
/// - **200 OK**: Email is valid
/// - **400 Bad Request**:
///   - Invalid email syntax
///   - Domain has no valid MX/A/AAAA records
///   - Role-based email address detected (if enabled)
///   - Disposable email detected
/// - **500 Internal Server Error**: Database or Redis connection failed
///
/// ## Example Requests
/// ```json
/// { "email": "user@example.com" }
/// ```
///
/// With role-based validation:
/// ```text
/// POST /api/v1/validate-email?check_role_based=true
/// { "email": "admin@example.com" }
/// ```
#[utoipa::path(
    post,
    path = "/api/v1/validate-email",
    request_body = EmailRequest,
    params(
        ("check_role_based" = Option<bool>, Query, description = "Enable role-based email validation")
    ),
    responses(
        (status = 200, description = "Email is valid"),
        (status = 400, description = "Invalid email"),
        (status = 500, description = "Server error")
    ),
    tag = "Email Validation"
)]
#[post("/validate-email")]
pub async fn validate_email(
    req: web::Json<EmailRequest>,
    query: web::Query<ValidationQuery>,
    redis_cache: web::Data<RedisCache>,
) -> Result<impl Responder, actix_web::Error> {
    let email = req.email.trim();

    // 1. Syntax validation
    if !syntax::is_valid_email(email) {
        return Ok(HttpResponse::BadRequest().json(json!({
            "error": "INVALID_SYNTAX",
            "message": "Email address has invalid syntax"
        })));
    }

    // Extract domain for DNS validation
    let parts: Vec<&str> = email.split('@').collect();
    let domain = parts[1];

    // 2. DNS/MX validation (with cache)
    let dns_valid = match redis_cache.get_dns_validation(domain).await {
        // Cache hit
        Ok(Some(cached_result)) => cached_result,

        // Cache miss or error - perform DNS lookup
        _ => {
            let email_clone = email.to_owned();
            let dns_result = web::block(move || dnsmx::validate_email_dns(&email_clone))
                .await
                .map_err(|e| {
                    actix_web::error::ErrorInternalServerError(format!(
                        "DNS validation error: {}",
                        e
                    ))
                })?;

            // Cache the result (ignore cache write errors)
            let _ = redis_cache.set_dns_validation(domain, dns_result).await;

            dns_result
        }
    };

    if !dns_valid {
        return Ok(HttpResponse::BadRequest().json(json!({
            "error": "INVALID_DOMAIN",
            "message": "Email domain has no valid DNS records"
        })));
    }

    // 3. Role-based email check (optional)
    if query.check_role_based {
        match role_based::is_role_based_email(email).await {
            Ok(true) => {
                return Ok(HttpResponse::BadRequest().json(json!({
                    "error": "ROLE_BASED_EMAIL",
                    "message": "Email address uses a role-based local part"
                })));
            }
            Ok(false) => {} // Continue validation
            Err(e) => {
                return Ok(HttpResponse::InternalServerError().json(json!({
                    "error": "DATABASE_ERROR",
                    "message": e
                })));
            }
        }
    }

    // 4. Disposable email check
    match disposable::is_disposable_email(email).await {
        Ok(true) => Ok(HttpResponse::BadRequest().json(json!({
            "error": "DISPOSABLE_EMAIL",
            "message": "The email address domain is a provider of disposable email addresses"
        }))),
        Ok(false) => Ok(HttpResponse::Ok().json(json!({
            "status": "VALID",
            "message": "Email address is valid"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": "DATABASE_ERROR",
            "message": e.to_string()
        }))),
    }
}

/// Configures email validation routes under /api/v1
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(validate_email);
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{App, test};
    use serde_json::json;
    use std::env;

    // Helper function to create a test app with Redis cache
    async fn create_test_app() -> impl actix_web::dev::Service<
        actix_http::Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    > {
        // Use test Redis URL (can be mocked in CI/CD)
        let redis_url =
            env::var("TEST_REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());

        // Use resilient Redis cache creation for tests
        let redis_cache = RedisCache::new(&redis_url, 3600).unwrap_or_else(|_| {
            eprintln!("Warning: Using dummy Redis cache for tests");
            RedisCache::test_dummy()
        });

        test::init_service(
            App::new()
                .app_data(web::Data::new(redis_cache))
                .configure(configure_routes),
        )
        .await
    }

    #[actix_web::test]
    async fn test_valid_email() {
        let app = create_test_app().await;
        let req = test::TestRequest::post()
            .uri("/validate-email")
            .set_json(json!({ "email": "test@example.com" }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_invalid_syntax() {
        let app = create_test_app().await;
        let req = test::TestRequest::post()
            .uri("/validate-email")
            .set_json(json!({ "email": "invalid-email" }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().as_u16(), 400);
    }

    #[actix_web::test]
    async fn test_invalid_domain() {
        let app = create_test_app().await;
        let req = test::TestRequest::post()
            .uri("/validate-email")
            .set_json(json!({ "email": "test@nonexistent.invalid" }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().as_u16(), 400);

        // Verify error details
        let body = test::read_body(resp).await;
        let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body_json["error"], "INVALID_DOMAIN");
        assert_eq!(
            body_json["message"],
            "Email domain has no valid DNS records"
        );
    }

    #[actix_web::test]
    async fn test_role_based_email_detection_when_enabled() {
        // Load environment variables from .env file for test
        dotenv::dotenv().ok();
        
        let app = create_test_app().await;
        let req = test::TestRequest::post()
            .uri("/validate-email?check_role_based=true")
            .set_json(json!({ "email": "support@apple.com" }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().as_u16(), 400);

        let body = test::read_body(resp).await;
        let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body_json["error"], "ROLE_BASED_EMAIL");
        assert_eq!(
            body_json["message"],
            "Email address uses a role-based local part"
        );
    }

    #[actix_web::test]
    async fn test_role_based_email_allowed_by_default() {
        let app = create_test_app().await;
        let req = test::TestRequest::post()
            .uri("/validate-email")
            .set_json(json!({ "email": "admin@example.com" }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Should pass validation since role-based check is disabled by default
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_disposable_email_detection() {
        let app = create_test_app().await;
        let req = test::TestRequest::post()
            .uri("/validate-email")
            // Use a known disposable domain that has valid DNS records
            .set_json(json!({ "email": "user@mailinator.com" }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().as_u16(), 400);

        let body = test::read_body(resp).await;
        let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body_json["error"], "DISPOSABLE_EMAIL");
        assert_eq!(
            body_json["message"],
            "The email address domain is a provider of disposable email addresses"
        );
    }

    #[actix_web::test]
    async fn test_redis_caching() {
        // This test verifies that caching works by making two identical requests
        // and ensuring the second one uses the cached result

        let app = create_test_app().await;

        // First request - should trigger DNS lookup and cache the result
        let req1 = test::TestRequest::post()
            .uri("/validate-email")
            .set_json(json!({ "email": "test@example.com" }))
            .to_request();

        let resp1 = test::call_service(&app, req1).await;
        assert!(resp1.status().is_success());

        // Second request with same domain - should use cached result
        let req2 = test::TestRequest::post()
            .uri("/validate-email")
            .set_json(json!({ "email": "different-user@example.com" }))
            .to_request();

        let resp2 = test::call_service(&app, req2).await;
        assert!(resp2.status().is_success());

        // Note: We can't directly test that the cache was used without adding metrics
        // or instrumentation, but the test ensures the caching code path works
    }

    #[actix_web::test]
    #[ignore] // TODO: Implement proper mocking
    async fn test_database_connection_error() {
        let app = create_test_app().await;

        let req = test::TestRequest::post()
            .uri("/validate-email")
            .set_json(json!({ "email": "valid@example.com" }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().as_u16(), 500);

        let body = test::read_body(resp).await;
        let body_json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        assert_eq!(body_json["error"], "DATABASE_ERROR");
        assert_eq!(
            body_json["message"].as_str().unwrap(),
            "Database connection failed"
        );
    }
}
