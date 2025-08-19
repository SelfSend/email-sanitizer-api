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
        let json = r#"{"email": "user@example.com"}"#;
        let email_request: EmailRequest = serde_json::from_str(json).unwrap();
        assert_eq!(email_request.email, "user@example.com");
    }

    #[test]
    fn test_missing_email_field() {
        let json = r#"{}"#;
        let result: Result<EmailRequest, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_email_type() {
        let json = r#"{"email": 123}"#;
        let result: Result<EmailRequest, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_empty_email_string() {
        let json = r#"{"email": ""}"#;
        let email_request: EmailRequest = serde_json::from_str(json).unwrap();
        assert_eq!(email_request.email, "");
    }

    #[test]
    fn test_email_with_whitespace() {
        let json = r#"{"email": "  user@example.com  "}"#;
        let email_request: EmailRequest = serde_json::from_str(json).unwrap();
        assert_eq!(email_request.email, "  user@example.com  ");
    }

    #[test]
    fn test_email_with_special_characters() {
        let json = r#"{"email": "test+tag@example.com"}"#;
        let email_request: EmailRequest = serde_json::from_str(json).unwrap();
        assert_eq!(email_request.email, "test+tag@example.com");
    }

    #[test]
    fn test_email_with_unicode() {
        let json = r#"{"email": "tëst@example.com"}"#;
        let email_request: EmailRequest = serde_json::from_str(json).unwrap();
        assert_eq!(email_request.email, "tëst@example.com");
    }

    #[test]
    fn test_null_email_field() {
        let json = r#"{"email": null}"#;
        let result: Result<EmailRequest, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_extra_fields_ignored() {
        let json = r#"{"email": "user@example.com", "extra": "ignored"}"#;
        let email_request: EmailRequest = serde_json::from_str(json).unwrap();
        assert_eq!(email_request.email, "user@example.com");
    }

    #[test]
    fn test_malformed_json() {
        let json = r#"{"email": "user@example.com""#; // Missing closing brace
        let result: Result<EmailRequest, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_array_instead_of_object() {
        let json = r#"["user@example.com"]"#;
        let result: Result<EmailRequest, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_boolean_email_field() {
        let json = r#"{"email": true}"#;
        let result: Result<EmailRequest, _> = serde_json::from_str(json);
        assert!(result.is_err());
    }

    #[test]
    fn test_very_long_email() {
        let long_email = "a".repeat(1000) + "@example.com";
        let json = format!(r#"{{"email": "{}"}}", long_email);
        let email_request: EmailRequest = serde_json::from_str(&json).unwrap();
        assert_eq!(email_request.email, long_email);
    }
}
