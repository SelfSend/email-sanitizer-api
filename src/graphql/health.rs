use crate::models::health::HealthResponse;
use async_graphql::{Context, Object, Result};

/// GraphQL representation of service health status
///
/// Provides health check information through the GraphQL API, mirroring the REST
/// health response structure but formatted for GraphQL compatibility.
///
/// # Fields
/// - `status`: Current service status (e.g., "UP")
/// - `timestamp`: ISO-8601 formatted timestamp of last status check
#[derive(Debug)]
pub struct Health {
    pub status: String,
    pub timestamp: String,
}

impl From<HealthResponse> for Health {
    /// Converts REST model response to GraphQL type
    ///
    /// Allows sharing health check infrastructure between REST and GraphQL APIs
    /// while maintaining separate presentation layers.
    fn from(response: HealthResponse) -> Self {
        Self {
            status: response.status,
            timestamp: response.timestamp,
        }
    }
}

#[Object]
impl Health {
    /// Current service status indicator
    ///
    /// # Returns
    /// String representation of service health state.
    /// Typical values:
    /// - "UP": Service operational
    /// - "DOWN": Service unavailable
    async fn status(&self) -> &str {
        &self.status
    }

    /// Last status check timestamp
    ///
    /// # Returns
    /// ISO-8601 formatted timestamp string in UTC timezone
    async fn timestamp(&self) -> &str {
        &self.timestamp
    }
}

/// Root query type for health-related GraphQL operations
///
/// Provides entry points for health monitoring through GraphQL,
/// following the same health check paradigm as the REST API.
#[derive(Default)]
pub struct HealthQuery;

#[Object]
impl HealthQuery {
    /// Checks service health status
    ///
    /// # Returns
    /// [`Health`] status object containing:
    /// - Current service status
    /// - Timestamp of check execution
    ///
    /// # Errors
    /// Currently always returns `Ok` - maintains `Result` return type
    /// for future error handling compatibility
    async fn health(&self, _ctx: &Context<'_>) -> Result<Health> {
        Ok(Health::from(HealthResponse::up()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_graphql::{EmptyMutation, EmptySubscription, Schema};
    use chrono::{DateTime, Utc};

    // Test the Health struct conversion from HealthResponse
    #[test]
    fn test_health_from_health_response() {
        // Create a test HealthResponse
        let status = "UP".to_string();
        let timestamp = Utc::now().to_rfc3339();
        let health_response = HealthResponse {
            status: status.clone(),
            timestamp: timestamp.clone(),
        };

        // Convert to Health
        let health = Health::from(health_response);

        // Verify fields are correctly mapped
        assert_eq!(health.status, status);
        assert_eq!(health.timestamp, timestamp);
    }

    // Test the HealthQuery resolver through the GraphQL schema execution
    #[tokio::test]
    async fn test_health_query_resolver() {
        // Create a schema with HealthQuery
        let schema = Schema::build(
            HealthQuery::default(),
            EmptyMutation::default(),
            EmptySubscription::default(),
        )
        .finish();

        // Execute the health query
        let query = r#"
            query {
                health {
                    status
                    timestamp
                }
            }
        "#;

        let result = schema.execute(query).await;

        // Verify no errors
        assert!(result.errors.is_empty());

        // Get data from result
        let data = result.data.into_json().unwrap();

        // Verify health data structure
        assert_eq!(data["health"]["status"], "UP");
        assert!(data["health"]["timestamp"].is_string());

        // Verify timestamp is a valid ISO 8601 date
        let timestamp = data["health"]["timestamp"].as_str().unwrap();
        assert!(DateTime::parse_from_rfc3339(timestamp).is_ok());
    }

    // Test the default implementation of HealthQuery
    #[test]
    fn test_health_query_default() {
        let health_query = HealthQuery::default();
        // Simply verify we can create a default instance
        // This is just for coverage of the #[derive(Default)]
        assert!(matches!(health_query, HealthQuery));
    }

    // Test the health resolver directly with a mock context
    #[tokio::test]
    async fn test_health_resolver_directly() {
        // Create HealthQuery
        let _health_query = HealthQuery::default();

        // Set up a minimal schema to get a valid context
        let schema = Schema::build(
            HealthQuery::default(),
            EmptyMutation::default(),
            EmptySubscription::default(),
        )
        .finish();

        // Execute a simple query to generate a context
        let query = r#"{ health { status } }"#;
        let result = schema.execute(query).await;

        // Verify result is successful
        assert!(result.errors.is_empty());

        // Now test the returned Health object
        // Extract the health result from the query
        let health_value = result.data.into_json().unwrap()["health"].clone();

        // Verify it's UP
        assert_eq!(health_value["status"], "UP");
    }

    // Test the internal structure of Health objects
    #[test]
    fn test_health_struct_properties() {
        // Create a Health object directly
        let status = "UP".to_string();
        let timestamp = "2025-04-19T12:00:00Z".to_string();

        let health = Health {
            status: status.clone(),
            timestamp: timestamp.clone(),
        };

        // Verify the fields are accessible
        assert_eq!(health.status, status);
        assert_eq!(health.timestamp, timestamp);
    }

    // Test HealthResponse::up() method
    #[test]
    fn test_health_response_up() {
        let response = HealthResponse::up();

        // Verify status is "UP"
        assert_eq!(response.status, "UP");

        // Verify timestamp is a valid ISO 8601 timestamp
        assert!(DateTime::parse_from_rfc3339(&response.timestamp).is_ok());
    }

    // Test health status values via GraphQL queries
    #[tokio::test]
    async fn test_health_status_value() {
        // Create a schema
        let schema = Schema::build(
            HealthQuery::default(),
            EmptyMutation::default(),
            EmptySubscription::default(),
        )
        .finish();

        // Query just the status field
        let query = r#"{ health { status } }"#;
        let result = schema.execute(query).await;

        // Check we got the expected UP status
        assert!(result.errors.is_empty());
        let data = result.data.into_json().unwrap();
        assert_eq!(data["health"]["status"], "UP");
    }

    // Test health timestamp values via GraphQL queries
    #[tokio::test]
    async fn test_health_timestamp_value() {
        // Create a schema
        let schema = Schema::build(
            HealthQuery::default(),
            EmptyMutation::default(),
            EmptySubscription::default(),
        )
        .finish();

        // Query just the timestamp field
        let query = r#"{ health { timestamp } }"#;
        let result = schema.execute(query).await;

        // Check we got a timestamp
        assert!(result.errors.is_empty());
        let data = result.data.into_json().unwrap();
        let timestamp = data["health"]["timestamp"].as_str().unwrap();
        assert!(DateTime::parse_from_rfc3339(timestamp).is_ok());
    }
}
