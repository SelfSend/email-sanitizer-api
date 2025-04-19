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

/// Constructs and configures the application's GraphQL schema
///
/// Initializes the schema with:
/// - HealthQuery as the root query type
/// - Empty mutation and subscription types
///
/// # Returns
/// Fully initialized [`AppSchema`] ready for query execution
///
/// # Example
/// ```rust
/// let schema = create_schema();
/// let query = "{ health { status timestamp } }";
/// let response = schema.execute(query).await;
/// ```
pub fn create_schema() -> AppSchema {
    Schema::build(
        HealthQuery::default(),
        EmptyMutation::default(),
        EmptySubscription::default(),
    )
    .finish()
}
