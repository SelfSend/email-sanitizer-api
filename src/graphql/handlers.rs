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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graphql::schema::create_schema;
    use actix_web::test;
    use actix_web::{
        App,
        http::{StatusCode, header::ContentType},
        test::{TestRequest, call_service, init_service},
    };
    use serde_json::{Value, json};

    // Test for graphql_handler
    #[actix_web::test]
    async fn test_graphql_handler() {
        // Create a real AppSchema
        let schema = create_schema();

        // Build test app with the handler
        let app = App::new()
            .app_data(web::Data::new(schema))
            .route("/graphql", web::post().to(graphql_handler));

        let app = init_service(app).await;

        // Test with a valid health query
        let req = TestRequest::post()
            .uri("/graphql")
            .insert_header(ContentType::json())
            .set_json(json!({
                "query": "query { health { status timestamp } }"
            }))
            .to_request();

        let resp = call_service(&app, req).await;

        // Check response
        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        let resp_body: Value =
            serde_json::from_slice(&body).expect("Failed to parse response body");

        // Check the health query returned expected fields
        assert!(resp_body["data"]["health"]["status"].is_string());
        assert!(resp_body["data"]["health"]["timestamp"].is_string());
        assert_eq!(resp_body["data"]["health"]["status"], "UP");

        // Test with an invalid query
        let req = TestRequest::post()
            .uri("/graphql")
            .insert_header(ContentType::json())
            .set_json(json!({
                "query": "query { invalid_field }"
            }))
            .to_request();

        let resp = call_service(&app, req).await;

        // Check response
        assert_eq!(resp.status(), StatusCode::OK); // GraphQL still returns 200 with errors

        let body = test::read_body(resp).await;
        let resp_body: Value =
            serde_json::from_slice(&body).expect("Failed to parse response body");

        assert!(resp_body["errors"].as_array().unwrap().len() > 0);
    }

    // Test for graphql_playground
    #[actix_web::test]
    async fn test_graphql_playground() {
        let app = App::new()
            .service(web::resource("/graphql/playground").route(web::get().to(graphql_playground)));
        let app = init_service(app).await;

        // Create test request
        let req = TestRequest::get().uri("/graphql/playground").to_request();

        // Execute request
        let resp = call_service(&app, req).await;

        // Assert response status code
        assert_eq!(resp.status(), StatusCode::OK);

        // Assert content type
        let content_type = resp
            .headers()
            .get("content-type")
            .unwrap()
            .to_str()
            .unwrap();
        assert_eq!(content_type, "text/html; charset=utf-8");

        // Assert response body contains playground HTML
        let body_bytes = test::read_body(resp).await;
        let body = std::str::from_utf8(&body_bytes).unwrap();

        // Check for key elements of the GraphQL playground HTML
        assert!(body.contains("GraphQL Playground"));
        assert!(body.contains("/api/v1/graphql"));
    }

    // Test integration with actual Actix Web app
    #[actix_web::test]
    async fn test_graphql_handler_integration() {
        // Create a real AppSchema
        let schema = create_schema();

        // Build test app with both handlers
        let app = App::new()
            .app_data(web::Data::new(schema))
            .route("/api/v1/graphql", web::post().to(graphql_handler))
            .route("/playground", web::get().to(graphql_playground));

        let app = test::init_service(app).await;

        // Test GraphQL endpoint with health query
        let req = test::TestRequest::post()
            .uri("/api/v1/graphql")
            .insert_header(ContentType::json())
            .set_json(json!({
                "query": "{ health { status timestamp } }"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        let body_json: Value = serde_json::from_slice(&body).unwrap();

        // Check the health query returned data
        assert!(body_json["data"]["health"]["status"].is_string());
        assert!(body_json["data"]["health"]["timestamp"].is_string());

        // Test playground endpoint
        let req = test::TestRequest::get().uri("/playground").to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        let body = test::read_body(resp).await;
        let body_str = std::str::from_utf8(&body).unwrap();
        assert!(body_str.contains("GraphQL Playground"));
    }
}
