use axum::{extract::Path, http::StatusCode, Json};
use crate::application::dto::{CreateUserDto, UserDto};

/// Health check endpoint
pub async fn health_check() -> &'static str {
    "OK"
}

/// Create user handler
pub async fn create_user(
    Json(_dto): Json<CreateUserDto>,
) -> Result<(StatusCode, Json<UserDto>), (StatusCode, String)> {
    // TODO: Inject use case here
    // For now, return error as this needs proper dependency injection setup
    Err((StatusCode::NOT_IMPLEMENTED, "Use case not injected".to_string()))
}

/// Get user handler
pub async fn get_user(
    Path(_id): Path<String>,
) -> Result<Json<UserDto>, (StatusCode, String)> {
    // TODO: Inject use case here
    Err((StatusCode::NOT_IMPLEMENTED, "Use case not injected".to_string()))
}

/// Update user handler
pub async fn update_user(
    Path(_id): Path<String>,
    Json(_dto): Json<serde_json::Value>,
) -> Result<Json<UserDto>, (StatusCode, String)> {
    // TODO: Inject use case here
    Err((StatusCode::NOT_IMPLEMENTED, "Use case not injected".to_string()))
}

/// Delete user handler
pub async fn delete_user(
    Path(_id): Path<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    // TODO: Inject use case here
    Err((StatusCode::NOT_IMPLEMENTED, "Use case not injected".to_string()))
}

