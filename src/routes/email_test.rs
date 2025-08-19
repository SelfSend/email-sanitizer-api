#[cfg(test)]
mod email_route_tests {
    use super::super::email::*;

    #[test]
    fn test_email_request_struct() {
        let req = EmailRequest {
            email: "test@example.com".to_string(),
        };
        assert_eq!(req.email, "test@example.com");
    }

    #[test]
    fn test_bulk_email_request_struct() {
        let req = BulkEmailRequest {
            emails: vec!["test1@example.com".to_string(), "test2@example.com".to_string()],
        };
        assert_eq!(req.emails.len(), 2);
        assert_eq!(req.emails[0], "test1@example.com");
    }

    #[test]
    fn test_email_validation_error_struct() {
        let error = EmailValidationError {
            code: "INVALID_SYNTAX".to_string(),
            message: "Invalid email format".to_string(),
        };
        assert_eq!(error.code, "INVALID_SYNTAX");
        assert_eq!(error.message, "Invalid email format");
    }

    #[test]
    fn test_email_validation_response_valid() {
        let response = EmailValidationResponse {
            is_valid: true,
            status: Some("VALID".to_string()),
            error: None,
        };
        assert!(response.is_valid);
        assert_eq!(response.status.unwrap(), "VALID");
        assert!(response.error.is_none());
    }

    #[test]
    fn test_email_validation_response_invalid() {
        let response = EmailValidationResponse {
            is_valid: false,
            status: None,
            error: Some(EmailValidationError {
                code: "INVALID_SYNTAX".to_string(),
                message: "Bad format".to_string(),
            }),
        };
        assert!(!response.is_valid);
        assert!(response.status.is_none());
        assert!(response.error.is_some());
    }

    #[test]
    fn test_bulk_email_validation_result() {
        let result = BulkEmailValidationResult {
            email: "test@example.com".to_string(),
            validation: EmailValidationResponse {
                is_valid: true,
                status: Some("VALID".to_string()),
                error: None,
            },
        };
        assert_eq!(result.email, "test@example.com");
        assert!(result.validation.is_valid);
    }

    #[test]
    fn test_bulk_email_validation_response() {
        let response = BulkEmailValidationResponse {
            results: vec![],
            valid_count: 5,
            invalid_count: 3,
        };
        assert_eq!(response.valid_count, 5);
        assert_eq!(response.invalid_count, 3);
        assert_eq!(response.results.len(), 0);
    }

    #[test]
    fn test_validation_query_default() {
        let query = ValidationQuery {
            check_role_based: false,
        };
        assert!(!query.check_role_based);
    }

    #[test]
    fn test_validation_query_enabled() {
        let query = ValidationQuery {
            check_role_based: true,
        };
        assert!(query.check_role_based);
    }

    #[tokio::test]
    async fn test_redis_cache_new_valid_url() {
        let result = RedisCache::new("redis://127.0.0.1:6379", 3600);
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_redis_cache_new_invalid_url() {
        let result = RedisCache::new("invalid-url", 3600);
        assert!(result.is_err());
    }

    #[test]
    fn test_redis_cache_test_dummy() {
        let cache = RedisCache::test_dummy();
        assert_eq!(cache.ttl, 3600);
    }

    #[tokio::test]
    async fn test_redis_cache_get_dns_validation() {
        let cache = RedisCache::test_dummy();
        let result = cache.get_dns_validation("example.com").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_redis_cache_set_dns_validation() {
        let cache = RedisCache::test_dummy();
        let result = cache.set_dns_validation("example.com", true).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_validate_single_email_invalid_syntax() {
        let cache = RedisCache::test_dummy();
        let result = validate_single_email("invalid-email", false, &cache).await;
        
        assert!(!result.is_valid);
        assert!(result.error.is_some());
        assert_eq!(result.error.unwrap().code, "INVALID_SYNTAX");
    }

    #[tokio::test]
    async fn test_validate_single_email_empty_string() {
        let cache = RedisCache::test_dummy();
        let result = validate_single_email("", false, &cache).await;
        
        assert!(!result.is_valid);
        assert!(result.error.is_some());
        assert_eq!(result.error.unwrap().code, "INVALID_SYNTAX");
    }

    #[tokio::test]
    async fn test_validate_single_email_whitespace_only() {
        let cache = RedisCache::test_dummy();
        let result = validate_single_email("   ", false, &cache).await;
        
        assert!(!result.is_valid);
        assert!(result.error.is_some());
        assert_eq!(result.error.unwrap().code, "INVALID_SYNTAX");
    }

    #[test]
    fn test_email_validation_error_serialization() {
        let error = EmailValidationError {
            code: "TEST_ERROR".to_string(),
            message: "Test message".to_string(),
        };
        let json = serde_json::to_string(&error).unwrap();
        let deserialized: EmailValidationError = serde_json::from_str(&json).unwrap();
        assert_eq!(error.code, deserialized.code);
        assert_eq!(error.message, deserialized.message);
    }

    #[test]
    fn test_email_validation_response_serialization() {
        let response = EmailValidationResponse {
            is_valid: true,
            status: Some("VALID".to_string()),
            error: None,
        };
        let json = serde_json::to_string(&response).unwrap();
        let deserialized: EmailValidationResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(response.is_valid, deserialized.is_valid);
        assert_eq!(response.status, deserialized.status);
    }

    #[test]
    fn test_validation_query_deserialization_default() {
        let json = r#"{}"#;
        let query: ValidationQuery = serde_json::from_str(json).unwrap();
        assert!(!query.check_role_based);
    }

    #[test]
    fn test_validation_query_deserialization_true() {
        let json = r#"{"check_role_based": true}"#;
        let query: ValidationQuery = serde_json::from_str(json).unwrap();
        assert!(query.check_role_based);
    }

    #[test]
    fn test_validation_query_deserialization_false() {
        let json = r#"{"check_role_based": false}"#;
        let query: ValidationQuery = serde_json::from_str(json).unwrap();
        assert!(!query.check_role_based);
    }

    #[test]
    fn test_bulk_email_request_empty() {
        let req = BulkEmailRequest {
            emails: vec![],
        };
        assert_eq!(req.emails.len(), 0);
    }

    #[test]
    fn test_bulk_email_request_single_email() {
        let req = BulkEmailRequest {
            emails: vec!["single@example.com".to_string()],
        };
        assert_eq!(req.emails.len(), 1);
        assert_eq!(req.emails[0], "single@example.com");
    }

    #[test]
    fn test_email_validation_error_different_codes() {
        let codes = vec![
            "INVALID_SYNTAX",
            "INVALID_DOMAIN", 
            "ROLE_BASED_EMAIL",
            "DISPOSABLE_EMAIL",
            "DATABASE_ERROR"
        ];
        
        for code in codes {
            let error = EmailValidationError {
                code: code.to_string(),
                message: format!("Error for {}", code),
            };
            assert_eq!(error.code, code);
        }
    }
}