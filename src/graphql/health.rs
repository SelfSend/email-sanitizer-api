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
