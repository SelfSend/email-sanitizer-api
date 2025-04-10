use chrono::Utc;
use serde::{Deserialize, Serialize};

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
