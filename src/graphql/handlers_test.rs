#[cfg(test)]
mod graphql_handlers_tests {
    use crate::graphql::handlers::*;
    use crate::graphql::schema::create_schema;
    use actix_web::{App, http::StatusCode, test, web};
    use serde_json::json;

    #[actix_web::test]
    async fn test_graphql_handler_valid_query() {
        let schema = create_schema();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(schema))
                .route("/graphql", web::post().to(graphql_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/graphql")
            .set_json(json!({
                "query": "{ __schema { types { name } } }"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_graphql_handler_invalid_query() {
        let schema = create_schema();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(schema))
                .route("/graphql", web::post().to(graphql_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/graphql")
            .set_json(json!({
                "query": "{ invalidField }"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Should return 200 but with GraphQL errors in response
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_graphql_handler_malformed_json() {
        let schema = create_schema();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(schema))
                .route("/graphql", web::post().to(graphql_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/graphql")
            .insert_header(("content-type", "application/json"))
            .set_payload("{invalid json}")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_graphql_handler_empty_body() {
        let schema = create_schema();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(schema))
                .route("/graphql", web::post().to(graphql_handler)),
        )
        .await;

        let req = test::TestRequest::post().uri("/graphql").to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_graphql_handler_with_variables() {
        let schema = create_schema();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(schema))
                .route("/graphql", web::post().to(graphql_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/graphql")
            .set_json(json!({
                "query": "query GetType($name: String!) { __type(name: $name) { name } }",
                "variables": { "name": "String" }
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_graphql_playground() {
        let app =
            test::init_service(App::new().route("/playground", web::get().to(graphql_playground)))
                .await;

        let req = test::TestRequest::get().uri("/playground").to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);

        // Check that it returns HTML content
        let content_type = resp.headers().get("content-type");
        assert!(content_type.is_some());
        let content_type_str = content_type.unwrap().to_str().unwrap();
        assert!(content_type_str.contains("text/html"));
    }

    #[actix_web::test]
    async fn test_graphql_playground_post_method() {
        let app =
            test::init_service(App::new().route("/playground", web::get().to(graphql_playground)))
                .await;

        // POST should not be allowed for playground
        let req = test::TestRequest::post().uri("/playground").to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_graphql_handler_introspection() {
        let schema = create_schema();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(schema))
                .route("/graphql", web::post().to(graphql_handler)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/graphql")
            .set_json(json!({
                "query": "{ __schema { queryType { name } } }"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_graphql_handler_mutation() {
        let schema = create_schema();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(schema))
                .route("/graphql", web::post().to(graphql_handler)),
        )
        .await;

        // Test if mutations are supported (depends on schema implementation)
        let req = test::TestRequest::post()
            .uri("/graphql")
            .set_json(json!({
                "query": "{ __schema { mutationType { name } } }"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_graphql_handler_subscription() {
        let schema = create_schema();
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(schema))
                .route("/graphql", web::post().to(graphql_handler)),
        )
        .await;

        // Test if subscriptions are supported (depends on schema implementation)
        let req = test::TestRequest::post()
            .uri("/graphql")
            .set_json(json!({
                "query": "{ __schema { subscriptionType { name } } }"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
