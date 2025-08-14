use crate::graphql::handlers;
use actix_web::web;

/// GraphQL Route Configuration
///
/// Defines and configures the web endpoints for GraphQL operations and development tools.
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/graphql").route(web::post().to(handlers::graphql_handler)))
        .service(web::resource("/playground").route(web::get().to(handlers::graphql_playground)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphql::schema::create_schema;
    use actix_web::{App, test, web};

    #[actix_web::test]
    async fn test_configure_routes() {
        let schema = create_schema();

        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(schema))
                .configure(configure_routes),
        )
        .await;

        // Test GraphQL endpoint
        let req = test::TestRequest::post()
            .uri("/graphql")
            .set_json(serde_json::json!({"query": "{ __typename }"}))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);

        // Test playground endpoint
        let req = test::TestRequest::get().uri("/playground").to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 200);
    }
}
