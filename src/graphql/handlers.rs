use actix_web::{HttpResponse, Responder, web};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::graphql::schema::AppSchema;

/// Handles incoming GraphQL requests.
///
/// This endpoint processes GraphQL queries, mutations, and subscriptions using the provided schema.
///
/// # Arguments
/// - `schema`: The application's GraphQL schema, provided as shared data through Actix-web's state management.
/// - `req`: The incoming GraphQL request containing the query, variables, and operation name.
///
/// # Returns
/// A [`GraphQLResponse`] containing the execution result of the GraphQL operation.
pub async fn graphql_handler(schema: web::Data<AppSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

/// Serves the GraphQL Playground interface for interactive query testing.
///
/// This handler responds with an HTML page that provides a graphical interface (Playground)
/// to construct and execute GraphQL queries against the API. The Playground is configured
/// to send requests to the `/api/v1/graphql` endpoint.
///
/// # Note
/// This endpoint is typically used during development and should be disabled in production
/// environments for security reasons.
///
/// # Returns
/// An [`HttpResponse`] with HTML content rendering the GraphQL Playground interface.
pub async fn graphql_playground() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(playground_source(GraphQLPlaygroundConfig::new(
            "/api/v1/graphql",
        )))
}
