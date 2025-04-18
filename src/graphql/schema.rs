use super::health::HealthQuery;
use async_graphql::{EmptyMutation, EmptySubscription, Schema};

/// Main GraphQL Schema Definition
///
/// Combines the root query type with empty mutation and subscription types
/// to form the complete GraphQL schema for the application.
///
/// # Type Parameters
/// - `HealthQuery`: Root query type containing all available query operations
/// - `EmptyMutation`: Placeholder for mutation operations (currently unused)
/// - `EmptySubscription`: Placeholder for subscription operations (currently unused)
pub type AppSchema = Schema<HealthQuery, EmptyMutation, EmptySubscription>;

/// Creates a new GraphQL schema with configured queries and mutations.
///
/// This function combines the health check query and any future GraphQL
/// operations into a unified schema that can be used with the GraphQL handler.
///
/// # Example
///
/// ```rust,no_run
/// use email_sanitizer::graphql::schema::create_schema;
///
/// let schema = create_schema();
/// ```
pub fn create_schema() -> AppSchema {
    Schema::build(HealthQuery, EmptyMutation, EmptySubscription).finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_schema() {
        // Create schema using the function
        let schema = create_schema();

        // Execute a simple health query to verify schema works
        let query = "{ health { status } }";
        let result = tokio_test::block_on(schema.execute(query));

        // Verify no errors
        assert!(result.errors.is_empty());

        // Verify data is returned and has expected structure
        let data = result.data.into_json().unwrap();
        assert!(data["health"]["status"].is_string());
        assert_eq!(data["health"]["status"], "UP");
    }

    #[test]
    fn test_schema_type() {
        // Simply ensure we can create the schema type
        // This is mostly to verify the type alias works
        let schema: AppSchema = create_schema();

        // Execute a simple health query
        let query = "{ health { status timestamp } }";
        let result = tokio_test::block_on(schema.execute(query));

        // Verify query succeeded
        assert!(result.errors.is_empty());

        // Check data structure
        let data = result.data.into_json().unwrap();
        assert!(data["health"].is_object());
        assert!(data["health"]["status"].is_string());
        assert!(data["health"]["timestamp"].is_string());
    }
}
