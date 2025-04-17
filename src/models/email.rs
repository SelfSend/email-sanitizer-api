use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct EmailRequest {
    pub email: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_valid_email_deserialization() {
        // Valid email format
        let json = r#"{"email": "user@example.com"}"#;
        let email_request: EmailRequest = serde_json::from_str(json).unwrap();
        assert_eq!(email_request.email, "user@example.com");
    }

    #[test]
    fn test_missing_email_field() {
        // Missing "email" field
        let json = r#"{}"#;
        let result: Result<EmailRequest, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_email_type() {
        // Non-string email value
        let json = r#"{"email": 123}"#;
        let result: Result<EmailRequest, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_email_string() {
        // Empty string (valid serialization but semantically invalid)
        let json = r#"{"email": ""}"#;
        let email_request: EmailRequest = serde_json::from_str(json).unwrap();
        assert_eq!(email_request.email, "");
    }
}
