use axum::{http::StatusCode, Json};
use crate::application::dto::{RegisterDto, LoginDto, AuthResponseDto};

pub async fn health_check() -> &'static str {
    "OK"
}

pub async fn register(
    Json(dto): Json<RegisterDto>,
) -> Result<(StatusCode, Json<AuthResponseDto>), (StatusCode, String)> {
    Err((StatusCode::NOT_IMPLEMENTED, "Use case not injected".to_string()))
}

pub async fn login(
    Json(dto): Json<LoginDto>,
) -> Result<(StatusCode, Json<AuthResponseDto>), (StatusCode, String)> {
    Err((StatusCode::NOT_IMPLEMENTED, "Use case not injected".to_string()))
}

pub async fn logout(
    Json(token): Json<String>,
) -> Result<StatusCode, (StatusCode, String)> {
    Err((StatusCode::NOT_IMPLEMENTED, "Use case not injected".to_string()))
}

