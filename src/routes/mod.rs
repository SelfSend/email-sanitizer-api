use actix_web::web;

/// # Health Check Endpoint
///
/// Returns the current health status of the service along with a timestamp.
///
/// ## Response
///
/// - **200 OK**: Service is healthy
///   - Body: JSON object with `status` ("UP") and `timestamp` in ISO 8601 format
///
/// ## Example Response
///
/// ```json
/// {
///   "status": "UP",
///   "timestamp": "2023-10-05T12:34:56.789Z"
/// }
/// ```
pub mod health;

/// # Email Validation Endpoint
///
/// Validates an email address by checking three aspects:
/// 1. RFC-compliant syntax validation
/// 2. Domain DNS/MX record verification
/// 3. Disposable email domain check
///
/// ## Request
/// - Method: POST
/// - Body: JSON object with `email` field
///
/// ## Responses
/// - **200 OK**: Email is valid
/// - **400 Bad Request**:
///   - Invalid email syntax
///   - Domain has no valid MX/A/AAAA records
///   - Disposable email detected
/// - **500 Internal Server Error**: Database connection failed
///
/// ## Example Request
/// ```json
/// { "email": "user@example.com" }
/// ```
pub mod email;

/// # API Route Configuration
///
/// Sets up versioned API endpoints under the `/api/v1` base path.
///
/// ## API Version
/// - Version: 1.0
/// - Base Path: `/api/v1`
///
/// ## Mounted Services
/// - Health check endpoints (see [`health::configure_routes`] for details)
/// - Email validation endpoints (see [`email::configure_routes`] for details)
///
/// ## Example Endpoints
///
/// ```text
/// GET /api/v1/health - Service health status
/// POST /api/v1/validate-email - Email validation endpoint
/// ```
///
/// [`health::configure_routes`]: crate::routes::health::configure_routes
/// [`email::configure_routes`]: crate::routes::email::configure_routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(health::configure_routes)
            .configure(email::configure_routes),
    );
}
