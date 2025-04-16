use chrono::Utc;
use serde::{Deserialize, Serialize};

/// # Health Status Response
///
/// Represents the operational status of the service with a timestamp.
/// Used as the response format for health check endpoints.
///
/// ## Fields
/// - `status`: String indicating service availability ("UP" or "DOWN")
/// - `timestamp`: ISO 8601 formatted timestamp of the status check
///
/// ## Serialization
/// Automatically implements `Serialize` and `Deserialize` for JSON format.
///
/// ## Example JSON
/// ```json
/// {
///   "status": "UP",
///   "timestamp": "2024-03-10T15:30:45.123456789Z"
/// }
/// ```
#[derive(Serialize, Debug, PartialEq, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub timestamp: String,
}

impl HealthResponse {
    pub fn up() -> Self {
        Self {
            status: "UP".to_string(),
            timestamp: Utc::now().to_rfc3339(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::DateTime;

    #[test]
    fn test_health_response_up() {
        let response = HealthResponse::up();

        // Verify status
        assert_eq!(response.status, "UP");

        // Verify timestamp is valid ISO 8601 format
        let parsed_time = DateTime::parse_from_rfc3339(&response.timestamp);
        assert!(
            parsed_time.is_ok(),
            "Timestamp should be valid RFC3339 format"
        );
    }
}
