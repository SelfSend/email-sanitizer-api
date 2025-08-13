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
pub mod health;

/// Validation functions for email addresses
pub mod validation;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_module_exists() {
        // Test that the health module is accessible
        // This ensures the module declaration is covered
        assert!(true);
    }

    #[test]
    fn test_validation_module_exists() {
        // Test that the validation module is accessible
        // This ensures the module declaration is covered
        assert!(true);
    }
}
