use crate::graphql::handlers;
use actix_web::web;

/// GraphQL Route Configuration
///
/// Defines and configures the web endpoints for GraphQL operations and development tools.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/graphql").route(web::post().to(handlers::graphql_handler)))
        .service(web::resource("/playground").route(web::get().to(handlers::graphql_playground)));
}
