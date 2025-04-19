use utoipa::OpenApi;

/// OpenAPI Specification Documentation
///
/// Defines the API contract using OpenAPI 3.0 format with utoipa procedural macros.
/// This documentation serves as the source of truth for both API consumers and
/// automated documentation generators.
///
/// # Endpoints
/// - Health Check: `GET /health`
/// - Email Validation: `POST /validate-email`
///
/// # Schemas
/// - `HealthResponse`: Service status payload
/// - `EmailRequest`: Email validation input structure
///
/// # Tags
/// 1. **Health Check**: Service monitoring endpoints
/// 2. **Email Validation**: Email sanitization operations
/// 3. **GraphQL**: Unified query interface
///
/// # API Information
/// - **Title**: Email Sanitizer API  
/// - **Version**: 0.6.0+sprint-3  
/// - **Description**: Combined REST and GraphQL interface for email processing  
///
/// # Note
/// The OpenAPI spec is generated at compile time from these annotations. Any changes
/// to the API surface should be reflected here first to maintain documentation accuracy.
#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::health::health,
        crate::routes::email::validate_email,
    ),
    components(
        schemas(
            crate::models::health::HealthResponse,
            crate::routes::email::EmailRequest
        )
    ),
    tags(
        (name = "Health Check", description = "Service health monitoring endpoints"),
        (name = "Email Validation", description = "Email address validation endpoints"),
        (name = "GraphQL", description = "GraphQL API for interacting with all service features")
    ),
    info(
        description = "API for email validation and sanitization with both REST and GraphQL interfaces",
        title = "Email Sanitizer API",
        version = "0.6.0+sprint-3",
    )
)]
pub struct ApiDoc;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use std::collections::HashMap;

    #[test]
    fn test_openapi_spec_is_valid() {
        // Generate OpenAPI spec as JSON
        let openapi_json = serde_json::to_string_pretty(&ApiDoc::openapi())
            .expect("Failed to serialize OpenAPI spec");

        // Parse it back to verify it's valid JSON
        let openapi_value: Value =
            serde_json::from_str(&openapi_json).expect("OpenAPI spec is not valid JSON");

        // Verify it's an object
        assert!(
            openapi_value.is_object(),
            "OpenAPI spec should be a JSON object"
        );
    }

    #[test]
    fn test_openapi_spec_contains_required_fields() {
        let openapi = ApiDoc::openapi();
        let json = serde_json::to_value(&openapi).expect("Failed to convert OpenAPI to JSON");

        // Check basic OpenAPI structure
        assert!(
            json.get("openapi").is_some(),
            "Missing 'openapi' version field"
        );
        assert!(json.get("info").is_some(), "Missing 'info' section");
        assert!(json.get("paths").is_some(), "Missing 'paths' section");
        assert!(
            json.get("components").is_some(),
            "Missing 'components' section"
        );
    }

    #[test]
    fn test_openapi_info_content() {
        let openapi = ApiDoc::openapi();
        let json = serde_json::to_value(&openapi).expect("Failed to convert OpenAPI to JSON");

        let info = json.get("info").expect("No info section found");

        // Check specific info contents
        assert_eq!(
            info.get("title").and_then(Value::as_str),
            Some("Email Sanitizer API"),
            "Incorrect API title"
        );

        assert_eq!(
            info.get("version").and_then(Value::as_str),
            Some("0.6.0+sprint-3"),
            "Incorrect API version"
        );

        assert_eq!(
            info.get("description").and_then(Value::as_str),
            Some("API for email validation and sanitization with both REST and GraphQL interfaces"),
            "Incorrect API description"
        );
    }

    #[test]
    fn test_openapi_paths() {
        let openapi = ApiDoc::openapi();
        let json = serde_json::to_value(&openapi).expect("Failed to convert OpenAPI to JSON");

        let paths = json.get("paths").expect("No paths section found");

        // Check that our paths exist
        assert!(
            paths.get("/api/v1/health").is_some(),
            "Missing health endpoint path"
        );
        assert!(
            paths.get("/api/v1/validate-email").is_some(),
            "Missing email validation endpoint path"
        );

        // Check health endpoint methods
        let health_path = paths.get("/api/v1/health").expect("No health path found");
        assert!(
            health_path.get("get").is_some(),
            "Health endpoint should support GET method"
        );

        // Check email validation endpoint methods
        let email_path = paths
            .get("/api/v1/validate-email")
            .expect("No email validation path found");
        assert!(
            email_path.get("post").is_some(),
            "Email validation endpoint should support POST method"
        );
    }

    #[test]
    fn test_openapi_components_schemas() {
        let openapi = ApiDoc::openapi();
        let json = serde_json::to_value(&openapi).expect("Failed to convert OpenAPI to JSON");

        let schemas = json
            .get("components")
            .and_then(|c| c.get("schemas"))
            .expect("No component schemas found");

        // Check that our schemas exist
        assert!(
            schemas.get("HealthResponse").is_some(),
            "Missing HealthResponse schema"
        );
        assert!(
            schemas.get("EmailRequest").is_some(),
            "Missing EmailRequest schema"
        );

        // Check HealthResponse schema properties
        let health_schema = schemas
            .get("HealthResponse")
            .expect("No HealthResponse schema found");
        let health_props = health_schema
            .get("properties")
            .expect("HealthResponse schema has no properties");

        assert!(
            health_props.get("status").is_some(),
            "HealthResponse missing 'status' property"
        );
        assert!(
            health_props.get("timestamp").is_some(),
            "HealthResponse missing 'timestamp' property"
        );

        // Check EmailRequest schema properties
        let email_schema = schemas
            .get("EmailRequest")
            .expect("No EmailRequest schema found");
        let email_props = email_schema
            .get("properties")
            .expect("EmailRequest schema has no properties");

        assert!(
            email_props.get("email").is_some(),
            "EmailRequest missing 'email' property"
        );
    }

    #[test]
    fn test_openapi_tags() {
        let openapi = ApiDoc::openapi();
        let json = serde_json::to_value(&openapi).expect("Failed to convert OpenAPI to JSON");

        let tags = json.get("tags").expect("No tags section found");
        let tags_array = tags.as_array().expect("Tags section is not an array");

        // Convert to a map for easier testing
        let mut tag_map: HashMap<String, String> = HashMap::new();
        for tag in tags_array {
            let name = tag
                .get("name")
                .and_then(Value::as_str)
                .expect("Tag missing name");
            let desc = tag
                .get("description")
                .and_then(Value::as_str)
                .expect("Tag missing description");
            tag_map.insert(name.to_string(), desc.to_string());
        }

        // Check all expected tags exist
        assert!(
            tag_map.contains_key("Health Check"),
            "Missing 'Health Check' tag"
        );
        assert!(
            tag_map.contains_key("Email Validation"),
            "Missing 'Email Validation' tag"
        );
        assert!(tag_map.contains_key("GraphQL"), "Missing 'GraphQL' tag");

        // Check tag descriptions
        assert_eq!(
            tag_map.get("Health Check"),
            Some(&"Service health monitoring endpoints".to_string()),
            "Incorrect description for Health Check tag"
        );

        assert_eq!(
            tag_map.get("Email Validation"),
            Some(&"Email address validation endpoints".to_string()),
            "Incorrect description for Email Validation tag"
        );

        assert_eq!(
            tag_map.get("GraphQL"),
            Some(&"GraphQL API for interacting with all service features".to_string()),
            "Incorrect description for GraphQL tag"
        );
    }
}
