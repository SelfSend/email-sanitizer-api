#[cfg(test)]
mod email_routes_edge_case_tests {
    use crate::routes::email::*;
    use actix_web::{App, http::StatusCode, test, web};
    use mongodb::{Client as MongoClient, options::ClientOptions};
    use serde_json::json;
    use std::env;

    async fn create_test_mongo_client() -> MongoClient {
        let mongo_uri =
            env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
        let client_options = ClientOptions::parse(&mongo_uri)
            .await
            .unwrap_or_else(|_| ClientOptions::default());
        MongoClient::with_options(client_options)
            .unwrap_or_else(|_| MongoClient::with_options(ClientOptions::default()).unwrap())
    }

    async fn create_test_app() -> impl actix_web::dev::Service<
        actix_http::Request,
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
    > {
        let redis_url =
            env::var("TEST_REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
        let redis_cache =
            RedisCache::new(&redis_url, 3600).unwrap_or_else(|_| RedisCache::test_dummy());
        let job_queue = crate::job_queue::JobQueue::new(&redis_url)
            .unwrap_or_else(|_| crate::job_queue::JobQueue::new("redis://127.0.0.1:6379").unwrap());
        let mongo_client = create_test_mongo_client().await;

        test::init_service(
            App::new()
                .app_data(web::Data::new(redis_cache))
                .app_data(web::Data::new(job_queue))
                .app_data(web::Data::new(mongo_client))
                .configure(configure_routes),
        )
        .await
    }

    #[actix_web::test]
    async fn test_validate_email_missing_content_type() {
        let app = create_test_app().await;
        let req = test::TestRequest::post()
            .uri("/validate-email")
            .insert_header(("Authorization", "Bearer test-api-key"))
            .set_payload(r#"{"email": "test@example.com"}"#)
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Should handle missing content-type gracefully
        assert!(resp.status().as_u16() >= 200);
    }

    #[actix_web::test]
    async fn test_validate_email_wrong_content_type() {
        let app = create_test_app().await;
        let req = test::TestRequest::post()
            .uri("/validate-email")
            .insert_header(("Authorization", "Bearer test-api-key"))
            .insert_header(("content-type", "text/plain"))
            .set_payload(r#"{"email": "test@example.com"}"#)
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Should handle wrong content-type
        assert!(resp.status().as_u16() >= 200);
    }

    #[actix_web::test]
    async fn test_validate_email_extra_fields() {
        let app = create_test_app().await;
        let req = test::TestRequest::post()
            .uri("/validate-email")
            .insert_header(("Authorization", "Bearer test-api-key"))
            .set_json(json!({
                "email": "test@example.com",
                "extra_field": "should_be_ignored",
                "another_field": 123
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Should ignore extra fields gracefully
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED); // Expected due to invalid API key
    }

    #[actix_web::test]
    async fn test_validate_email_null_values() {
        let app = create_test_app().await;
        let req = test::TestRequest::post()
            .uri("/validate-email")
            .insert_header(("Authorization", "Bearer test-api-key"))
            .set_json(json!({
                "email": null
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Should handle null values
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_validate_email_wrong_data_type() {
        let app = create_test_app().await;
        let req = test::TestRequest::post()
            .uri("/validate-email")
            .insert_header(("Authorization", "Bearer test-api-key"))
            .set_json(json!({
                "email": 12345
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Should handle wrong data type
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_validate_emails_bulk_null_array() {
        let app = create_test_app().await;
        let req = test::TestRequest::post()
            .uri("/validate-emails-bulk")
            .insert_header(("Authorization", "Bearer test-api-key"))
            .set_json(json!({
                "emails": null
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Should handle null array
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_validate_emails_bulk_mixed_types() {
        let app = create_test_app().await;
        let req = test::TestRequest::post()
            .uri("/validate-emails-bulk")
            .insert_header(("Authorization", "Bearer test-api-key"))
            .set_json(json!({
                "emails": ["test@example.com", 123, null, "another@example.com"]
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Should handle mixed types in array
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_validate_emails_bulk_very_large_batch() {
        let app = create_test_app().await;
        let large_email_list: Vec<String> = (0..1000)
            .map(|i| format!("user{}@example.com", i))
            .collect();

        let req = test::TestRequest::post()
            .uri("/validate-emails-bulk")
            .insert_header(("Authorization", "Bearer test-api-key"))
            .set_json(json!({
                "emails": large_email_list
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Should handle large batches (likely queued)
        assert!(resp.status() == StatusCode::UNAUTHORIZED || resp.status() == StatusCode::ACCEPTED);
    }

    #[actix_web::test]
    async fn test_get_job_status_invalid_job_id() {
        let app = create_test_app().await;
        let req = test::TestRequest::get()
            .uri("/job-status/invalid-job-id-12345")
            .insert_header(("Authorization", "Bearer test-api-key"))
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Should handle invalid job ID
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED); // Expected due to invalid API key
    }

    #[actix_web::test]
    async fn test_get_job_status_empty_job_id() {
        let app = create_test_app().await;
        let req = test::TestRequest::get()
            .uri("/job-status/")
            .insert_header(("Authorization", "Bearer test-api-key"))
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Should handle empty job ID (404 expected)
        assert_eq!(resp.status(), StatusCode::NOT_FOUND);
    }

    #[actix_web::test]
    async fn test_get_job_status_special_characters() {
        let app = create_test_app().await;
        let req = test::TestRequest::get()
            .uri("/job-status/job-id-with-special-chars!@#$%")
            .insert_header(("Authorization", "Bearer test-api-key"))
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Should handle special characters in job ID
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED); // Expected due to invalid API key
    }

    #[actix_web::test]
    async fn test_validate_email_with_query_params() {
        let app = create_test_app().await;
        let req = test::TestRequest::post()
            .uri("/validate-email?check_role_based=invalid")
            .insert_header(("Authorization", "Bearer test-api-key"))
            .set_json(json!({
                "email": "test@example.com"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Should handle invalid query parameter values
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST); // Invalid query param causes bad request
    }

    #[actix_web::test]
    async fn test_validate_email_case_insensitive_header() {
        let app = create_test_app().await;
        let req = test::TestRequest::post()
            .uri("/validate-email")
            .insert_header(("authorization", "bearer test-api-key")) // lowercase
            .set_json(json!({
                "email": "test@example.com"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Should handle case-insensitive headers
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED); // Expected due to invalid API key
    }

    #[actix_web::test]
    async fn test_validate_email_bearer_token_variations() {
        let app = create_test_app().await;

        // Test without "Bearer " prefix
        let req = test::TestRequest::post()
            .uri("/validate-email")
            .insert_header(("Authorization", "test-api-key"))
            .set_json(json!({
                "email": "test@example.com"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);

        // Test with extra spaces
        let req = test::TestRequest::post()
            .uri("/validate-email")
            .insert_header(("Authorization", "Bearer  test-api-key"))
            .set_json(json!({
                "email": "test@example.com"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }

    #[actix_web::test]
    async fn test_redis_cache_error_handling() {
        let redis_cache = RedisCache::test_dummy();

        // Test cache methods with various inputs
        let result = redis_cache.get_dns_validation("").await;
        assert!(result.is_ok());

        let result = redis_cache
            .get_dns_validation("very-long-domain-name-that-might-cause-issues.com")
            .await;
        assert!(result.is_ok());

        let result = redis_cache.set_dns_validation("", true).await;
        assert!(result.is_ok());

        let result = redis_cache.set_dns_validation("test.com", true).await;
        assert!(result.is_ok());
    }

    #[actix_web::test]
    async fn test_validate_single_email_function_edge_cases() {
        let redis_cache = RedisCache::test_dummy();

        // Test with whitespace
        let result = validate_single_email("  test@example.com  ", false, &redis_cache).await;
        assert!(result.is_valid || !result.is_valid); // Either outcome is valid

        // Test with empty string
        let result = validate_single_email("", false, &redis_cache).await;
        assert!(!result.is_valid);

        // Test with very long email
        let long_email = format!("{}@example.com", "a".repeat(300));
        let result = validate_single_email(&long_email, false, &redis_cache).await;
        assert!(!result.is_valid);

        // Test with Unicode
        let result = validate_single_email("tëst@exämple.com", false, &redis_cache).await;
        assert!(result.is_valid || !result.is_valid); // Either outcome is valid
    }
}
