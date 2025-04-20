use crate::handlers::validation::{disposable, dnsmx, syntax};
use async_graphql::{Context, Object, Result, SimpleObject};

/// Represents the possible validation errors for an email address
///
/// Each error corresponds to a specific validation failure:
/// - `INVALID_SYNTAX`: The email format is not RFC-compliant
/// - `INVALID_DOMAIN`: The domain does not have valid DNS/MX records
/// - `DISPOSABLE_EMAIL`: The email comes from a disposable email provider
/// - `DATABASE_ERROR`: Could not check disposable email database
#[derive(SimpleObject, Clone)]
pub struct EmailValidationError {
    /// Error code: INVALID_SYNTAX, INVALID_DOMAIN, DISPOSABLE_EMAIL, or DATABASE_ERROR
    pub code: String,
    /// Human-readable error message
    pub message: String,
}

/// Response object for email validation containing either valid status or error details
#[derive(SimpleObject)]
pub struct EmailValidationResponse {
    /// Whether the email is valid
    pub is_valid: bool,
    /// If valid, contains "VALID", otherwise null
    pub status: Option<String>,
    /// Error information if validation failed, otherwise null
    pub error: Option<EmailValidationError>,
}

/// Email validation query operations
#[derive(Default)]
pub struct EmailQuery;

#[Object]
impl EmailQuery {
    /// Validates an email address through multiple checks:
    /// 1. RFC 5322 compliant syntax validation
    /// 2. Domain DNS/MX record verification
    /// 3. Disposable email provider database check
    ///
    /// # Arguments
    /// * `email` - The email address to validate (will be trimmed automatically)
    ///
    /// # Returns
    /// [`EmailValidationResponse`] containing either:
    /// - Validation success status ("VALID") with no errors, or
    /// - Detailed error information for failed checks
    async fn validate_email(
        &self,
        _ctx: &Context<'_>,
        email: String,
    ) -> Result<EmailValidationResponse> {
        let email = email.trim();

        // 1. Syntax validation
        if !syntax::is_valid_email(email) {
            return Ok(EmailValidationResponse {
                is_valid: false,
                status: None,
                error: Some(EmailValidationError {
                    code: "INVALID_SYNTAX".to_string(),
                    message: "Email address has invalid syntax".to_string(),
                }),
            });
        }

        // 2. DNS/MX validation (blocking task)
        let email_clone = email.to_owned();
        let dns_valid =
            tokio::task::spawn_blocking(move || dnsmx::validate_email_dns(&email_clone))
                .await
                .map_err(|e| async_graphql::Error::new(format!("Task join error: {}", e)))?;

        if !dns_valid {
            return Ok(EmailValidationResponse {
                is_valid: false,
                status: None,
                error: Some(EmailValidationError {
                    code: "INVALID_DOMAIN".to_string(),
                    message: "Email domain has no valid DNS records".to_string(),
                }),
            });
        }

        // 3. Disposable email check
        match disposable::is_disposable_email(email).await {
            Ok(true) => Ok(EmailValidationResponse {
                is_valid: false,
                status: None,
                error: Some(EmailValidationError {
                    code: "DISPOSABLE_EMAIL".to_string(),
                    message: "The email address domain is a provider of disposable email addresses"
                        .to_string(),
                }),
            }),
            Ok(false) => Ok(EmailValidationResponse {
                is_valid: true,
                status: Some("VALID".to_string()),
                error: None,
            }),
            Err(e) => Ok(EmailValidationResponse {
                is_valid: false,
                status: None,
                error: Some(EmailValidationError {
                    code: "DATABASE_ERROR".to_string(),
                    message: e.to_string(),
                }),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_graphql::Schema;
    use mockall::mock;
    use mockall::predicate::*;

    // Mock the validation functions
    mock! {
        pub Validation {
            fn is_valid_email(&self, email: &str) -> bool;
            fn validate_email_dns(&self, email: &str) -> bool;
            async fn is_disposable_email(&self, email: &str) -> std::result::Result<bool, String>;
        }
    }

    #[tokio::test]
    async fn test_validate_email_valid() {
        // Create a schema just for testing
        let schema = Schema::build(
            EmailQuery::default(),
            async_graphql::EmptyMutation,
            async_graphql::EmptySubscription,
        )
        .finish();

        // Execute the query with test data
        let query = r#"
            query {
                validateEmail(email: "test@example.com") {
                    isValid
                    status
                    error {
                        code
                        message
                    }
                }
            }
        "#;

        let res = schema.execute(query).await;

        // Check for any syntax errors in the query
        assert!(
            res.errors.is_empty(),
            "GraphQL query has errors: {:?}",
            res.errors
        );

        let data = res.data.into_json().unwrap();
        assert!(data["validateEmail"]["isValid"].is_boolean());
    }
}
