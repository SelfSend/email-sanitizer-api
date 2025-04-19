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
