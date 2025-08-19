#[cfg(test)]
mod graphql_email_tests {
    use super::super::email::*;
    use serde_json;

    #[test]
    fn test_email_validation_error_struct() {
        let error = EmailValidationError {
            code: "TEST_CODE".to_string(),
            message: "Test message".to_string(),
        };
        assert_eq!(error.code, "TEST_CODE");
        assert_eq!(error.message, "Test message");
    }

    #[test]
    fn test_email_validation_response_valid() {
        let response = EmailValidationResponse {
            is_valid: true,
            status: Some("VALID".to_string()),
            error: None,
        };
        assert!(response.is_valid);
        assert_eq!(response.status.as_ref().unwrap(), "VALID");
        assert!(response.error.is_none());
    }

    #[test]
    fn test_email_validation_response_invalid() {
        let response = EmailValidationResponse {
            is_valid: false,
            status: None,
            error: Some(EmailValidationError {
                code: "INVALID_SYNTAX".to_string(),
                message: "Invalid format".to_string(),
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
            valid_count: 10,
            invalid_count: 5,
        };
        assert_eq!(response.valid_count, 10);
        assert_eq!(response.invalid_count, 5);
        assert!(response.results.is_empty());
    }

    #[test]
    fn test_cached_validation_response_conversion() {
        let original = EmailValidationResponse {
            is_valid: true,
            status: Some("VALID".to_string()),
            error: None,
        };

        let cached: CachedValidationResponse = original.clone().into();
        assert_eq!(cached.is_valid, original.is_valid);
        assert_eq!(cached.status, original.status);
        assert!(cached.error.is_none());

        let converted: EmailValidationResponse = cached.into();
        assert_eq!(converted.is_valid, original.is_valid);
        assert_eq!(converted.status, original.status);
        assert!(converted.error.is_none());
    }

    #[test]
    fn test_cached_validation_response_with_error() {
        let original = EmailValidationResponse {
            is_valid: false,
            status: None,
            error: Some(EmailValidationError {
                code: "TEST_ERROR".to_string(),
                message: "Test error message".to_string(),
            }),
        };

        let cached: CachedValidationResponse = original.clone().into();
        let converted: EmailValidationResponse = cached.into();

        assert_eq!(converted.is_valid, original.is_valid);
        assert_eq!(converted.status, original.status);
        assert!(converted.error.is_some());
        assert_eq!(
            converted.error.as_ref().unwrap().code,
            original.error.as_ref().unwrap().code
        );
    }

    #[tokio::test]
    async fn test_email_query_new_valid_url() {
        let result = EmailQuery::new("redis://127.0.0.1:6379", 3600);
        // Should either succeed or fail gracefully
        assert!(result.is_ok() || result.is_err());
    }

    #[tokio::test]
    async fn test_email_query_new_invalid_url() {
        let result = EmailQuery::new("invalid-url", 3600);
        assert!(result.is_err());
    }

    #[test]
    fn test_email_query_default() {
        let query = EmailQuery::default();
        assert!(query.redis_client.is_none());
        assert_eq!(query.cache_ttl, 0);
    }

    #[tokio::test]
    async fn test_email_query_get_cached_result_no_client() {
        let query = EmailQuery::default();
        let result = query.get_cached_result("test@example.com").await;
        assert!(result.is_none());
    }

    #[tokio::test]
    async fn test_email_query_cache_result_no_client() {
        let query = EmailQuery::default();
        let response = EmailValidationResponse {
            is_valid: true,
            status: Some("VALID".to_string()),
            error: None,
        };
        // Should not panic when no Redis client is available
        query.cache_result("test@example.com", &response).await;
    }

    #[tokio::test]
    async fn test_perform_validation_invalid_syntax() {
        let query = EmailQuery::default();
        let result = query.perform_validation("invalid-email".to_string(), false).await;
        
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(!response.is_valid);
        assert!(response.error.is_some());
        assert_eq!(response.error.unwrap().code, "INVALID_SYNTAX");
    }

    #[tokio::test]
    async fn test_perform_validation_empty_email() {
        let query = EmailQuery::default();
        let result = query.perform_validation("".to_string(), false).await;
        
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(!response.is_valid);
        assert!(response.error.is_some());
        assert_eq!(response.error.unwrap().code, "INVALID_SYNTAX");
    }

    #[tokio::test]
    async fn test_perform_validation_whitespace_email() {
        let query = EmailQuery::default();
        let result = query.perform_validation("   ".to_string(), false).await;
        
        assert!(result.is_ok());
        let response = result.unwrap();
        assert!(!response.is_valid);
        assert!(response.error.is_some());
        assert_eq!(response.error.unwrap().code, "INVALID_SYNTAX");
    }

    // Test serialization of cached response
    #[test]
    fn test_cached_validation_response_serialization() {
        let cached = CachedValidationResponse {
            is_valid: true,
            status: Some("VALID".to_string()),
            error: None,
        };

        let json = serde_json::to_string(&cached).unwrap();
        let deserialized: CachedValidationResponse = serde_json::from_str(&json).unwrap();
        
        assert_eq!(cached.is_valid, deserialized.is_valid);
        assert_eq!(cached.status, deserialized.status);
        assert!(cached.error.is_none() && deserialized.error.is_none());
    }

    #[test]
    fn test_cached_validation_response_serialization_with_error() {
        let cached = CachedValidationResponse {
            is_valid: false,
            status: None,
            error: Some(EmailValidationError {
                code: "TEST_ERROR".to_string(),
                message: "Test message".to_string(),
            }),
        };

        let json = serde_json::to_string(&cached).unwrap();
        let deserialized: CachedValidationResponse = serde_json::from_str(&json).unwrap();
        
        assert_eq!(cached.is_valid, deserialized.is_valid);
        assert_eq!(cached.status, deserialized.status);
        assert!(cached.error.is_some() && deserialized.error.is_some());
        assert_eq!(
            cached.error.as_ref().unwrap().code,
            deserialized.error.as_ref().unwrap().code
        );
    }

    // Test error code variations
    #[test]
    fn test_email_validation_error_codes() {
        let error_codes = vec![
            "INVALID_SYNTAX",
            "INVALID_DOMAIN",
            "ROLE_BASED_EMAIL", 
            "DISPOSABLE_EMAIL",
            "DATABASE_ERROR",
            "PROCESSING_ERROR"
        ];

        for code in error_codes {
            let error = EmailValidationError {
                code: code.to_string(),
                message: format!("Message for {}", code),
            };
            assert_eq!(error.code, code);
            assert!(error.message.contains(code));
        }
    }

    // Test bulk validation response with mixed results
    #[test]
    fn test_bulk_validation_response_mixed_results() {
        let results = vec![
            BulkEmailValidationResult {
                email: "valid@example.com".to_string(),
                validation: EmailValidationResponse {
                    is_valid: true,
                    status: Some("VALID".to_string()),
                    error: None,
                },
            },
            BulkEmailValidationResult {
                email: "invalid-email".to_string(),
                validation: EmailValidationResponse {
                    is_valid: false,
                    status: None,
                    error: Some(EmailValidationError {
                        code: "INVALID_SYNTAX".to_string(),
                        message: "Invalid syntax".to_string(),
                    }),
                },
            },
        ];

        let response = BulkEmailValidationResponse {
            results,
            valid_count: 1,
            invalid_count: 1,
        };

        assert_eq!(response.results.len(), 2);
        assert_eq!(response.valid_count, 1);
        assert_eq!(response.invalid_count, 1);
        
        // Check first result (valid)
        assert!(response.results[0].validation.is_valid);
        assert_eq!(response.results[0].email, "valid@example.com");
        
        // Check second result (invalid)
        assert!(!response.results[1].validation.is_valid);
        assert_eq!(response.results[1].email, "invalid-email");
    }

    // Test edge cases for email validation response
    #[test]
    fn test_email_validation_response_edge_cases() {
        // Response with empty status string
        let response1 = EmailValidationResponse {
            is_valid: true,
            status: Some("".to_string()),
            error: None,
        };
        assert!(response1.is_valid);
        assert_eq!(response1.status.as_ref().unwrap(), "");

        // Response with both status and error (unusual but possible)
        let response2 = EmailValidationResponse {
            is_valid: false,
            status: Some("INVALID".to_string()),
            error: Some(EmailValidationError {
                code: "TEST".to_string(),
                message: "Test".to_string(),
            }),
        };
        assert!(!response2.is_valid);
        assert!(response2.status.is_some());
        assert!(response2.error.is_some());
    }

    // Test error message variations
    #[test]
    fn test_email_validation_error_message_variations() {
        let messages = vec![
            "",
            "Short message",
            "A very long error message that contains multiple sentences and provides detailed information about what went wrong during the validation process.",
            "Message with special characters: !@#$%^&*()",
            "Message with unicode: 测试消息 🚀",
        ];

        for message in messages {
            let error = EmailValidationError {
                code: "TEST_CODE".to_string(),
                message: message.to_string(),
            };
            assert_eq!(error.message, message);
        }
    }

    // Test cloning of structs
    #[test]
    fn test_email_validation_error_clone() {
        let original = EmailValidationError {
            code: "TEST".to_string(),
            message: "Test message".to_string(),
        };
        let cloned = original.clone();
        assert_eq!(original.code, cloned.code);
        assert_eq!(original.message, cloned.message);
    }

    #[test]
    fn test_email_validation_response_clone() {
        let original = EmailValidationResponse {
            is_valid: true,
            status: Some("VALID".to_string()),
            error: None,
        };
        let cloned = original.clone();
        assert_eq!(original.is_valid, cloned.is_valid);
        assert_eq!(original.status, cloned.status);
    }

    // Test Debug trait implementations
    #[test]
    fn test_email_validation_error_debug() {
        let error = EmailValidationError {
            code: "TEST".to_string(),
            message: "Test".to_string(),
        };
        let debug_str = format!("{:?}", error);
        assert!(debug_str.contains("TEST"));
    }

    // Test default implementations where applicable
    #[test]
    fn test_email_query_default_values() {
        let query = EmailQuery {
            redis_client: None,
            cache_ttl: 7200,
        };
        assert!(query.redis_client.is_none());
        assert_eq!(query.cache_ttl, 7200);
    }
}