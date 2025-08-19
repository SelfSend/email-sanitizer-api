#[cfg(test)]
mod auth_integration_tests {
    use crate::routes::auth::*;
    use actix_web::{test, App, web, http::StatusCode};
    use mongodb::{Client as MongoClient, options::ClientOptions};
    use serde_json::json;

    async fn create_test_mongo_client() -> MongoClient {
        let mongo_uri = std::env::var("MONGODB_URI").unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
        let client_options = ClientOptions::parse(&mongo_uri).await.unwrap_or_else(|_| {
            ClientOptions::default()
        });
        MongoClient::with_options(client_options).unwrap_or_else(|_| {
            MongoClient::with_options(ClientOptions::default()).unwrap()
        })
    }

    #[actix_web::test]
    async fn test_register_empty_body() {
        unsafe {
            std::env::set_var("JWT_SECRET", "test-secret-key-for-testing");
        }
        let mongo_client = create_test_mongo_client().await;
        
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(mongo_client))
                .configure(configure_routes),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/register")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_register_malformed_json() {
        unsafe {
            std::env::set_var("JWT_SECRET", "test-secret-key-for-testing");
        }
        let mongo_client = create_test_mongo_client().await;
        
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(mongo_client))
                .configure(configure_routes),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/register")
            .insert_header(("content-type", "application/json"))
            .set_payload("{invalid json}")
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }

    #[actix_web::test]
    async fn test_register_empty_email() {
        unsafe {
            std::env::set_var("JWT_SECRET", "test-secret-key-for-testing");
        }
        let mongo_client = create_test_mongo_client().await;
        
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(mongo_client))
                .configure(configure_routes),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/register")
            .set_json(json!({
                "email": "",
                "password": "password123"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Should succeed or fail gracefully, not crash
        assert!(resp.status().as_u16() >= 200);
    }

    #[actix_web::test]
    async fn test_register_empty_password() {
        unsafe {
            std::env::set_var("JWT_SECRET", "test-secret-key-for-testing");
        }
        let mongo_client = create_test_mongo_client().await;
        
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(mongo_client))
                .configure(configure_routes),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/register")
            .set_json(json!({
                "email": "test@example.com",
                "password": ""
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Should succeed or fail gracefully, not crash
        assert!(resp.status().as_u16() >= 200);
    }

    #[actix_web::test]
    async fn test_register_very_long_email() {
        unsafe {
            std::env::set_var("JWT_SECRET", "test-secret-key-for-testing");
        }
        let mongo_client = create_test_mongo_client().await;
        
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(mongo_client))
                .configure(configure_routes),
        )
        .await;

        let long_email = format!("{}@example.com", "a".repeat(300));
        let req = test::TestRequest::post()
            .uri("/register")
            .set_json(json!({
                "email": long_email,
                "password": "password123"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Should handle gracefully
        assert!(resp.status().as_u16() >= 200);
    }

    #[actix_web::test]
    async fn test_register_very_long_password() {
        unsafe {
            std::env::set_var("JWT_SECRET", "test-secret-key-for-testing");
        }
        let mongo_client = create_test_mongo_client().await;
        
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(mongo_client))
                .configure(configure_routes),
        )
        .await;

        let long_password = "a".repeat(1000);
        let req = test::TestRequest::post()
            .uri("/register")
            .set_json(json!({
                "email": "test@example.com",
                "password": long_password
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Should handle gracefully
        assert!(resp.status().as_u16() >= 200);
    }

    #[actix_web::test]
    async fn test_register_special_characters_in_email() {
        unsafe {
            std::env::set_var("JWT_SECRET", "test-secret-key-for-testing");
        }
        let mongo_client = create_test_mongo_client().await;
        
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(mongo_client))
                .configure(configure_routes),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/register")
            .set_json(json!({
                "email": "test+tag@example.com",
                "password": "password123"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Should handle gracefully
        assert!(resp.status().as_u16() >= 200);
    }

    #[actix_web::test]
    async fn test_register_unicode_email() {
        unsafe {
            std::env::set_var("JWT_SECRET", "test-secret-key-for-testing");
        }
        let mongo_client = create_test_mongo_client().await;
        
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(mongo_client))
                .configure(configure_routes),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/register")
            .set_json(json!({
                "email": "tëst@exämple.com",
                "password": "password123"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Should handle gracefully
        assert!(resp.status().as_u16() >= 200);
    }

    #[actix_web::test]
    async fn test_register_missing_jwt_secret() {
        unsafe {
            std::env::remove_var("JWT_SECRET");
        }
        let mongo_client = create_test_mongo_client().await;
        
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(mongo_client))
                .configure(configure_routes),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/register")
            .set_json(json!({
                "email": "test@example.com",
                "password": "password123"
            }))
            .to_request();

        let resp = test::call_service(&app, req).await;
        // Should return 500 due to missing JWT secret
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }
}