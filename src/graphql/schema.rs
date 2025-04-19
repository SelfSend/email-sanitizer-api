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
    Schema::build(
        HealthQuery::default(),
        EmptyMutation::default(),
        EmptySubscription::default(),
    )
    .finish()
}
