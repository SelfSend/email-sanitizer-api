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

/// # API Route Configuration
///
/// Sets up versioned API endpoints under the `/api/v1` base pathw.
///
/// ## API Version
/// - Version: 1.0
/// - Base Path: `/api/v1`
///
/// ## Mounted Services
/// - Health check endpoints (see [`health::configure_routes`] for details)
///
/// ## Example Endpoints
///
/// ```text
/// GET /api/v1/health - Service health status
/// ```
///
/// [`health::configure_routes`]: crate::routes::health::configure_routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/api/v1").configure(health::configure_routes));
}
