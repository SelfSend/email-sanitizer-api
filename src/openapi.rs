use utoipa::OpenApi;

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
        (name = "Health Check", description = "Service health monitoring"),
        (name = "Email Validation", description = "Email verification endpoints")
    )
)]
pub struct ApiDoc;
