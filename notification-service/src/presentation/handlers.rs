use axum::{extract::Path, http::StatusCode, Json};
use crate::application::dto::{CreateNotificationDto, NotificationDto};

pub async fn health_check() -> &'static str {
    "OK"
}

pub async fn send_notification(
    Json(dto): Json<CreateNotificationDto>,
) -> Result<(StatusCode, Json<NotificationDto>), (StatusCode, String)> {
    Err((StatusCode::NOT_IMPLEMENTED, "Use case not injected".to_string()))
}

pub async fn get_notifications(
    Path(user_id): Path<String>,
) -> Result<Json<Vec<NotificationDto>>, (StatusCode, String)> {
    Err((StatusCode::NOT_IMPLEMENTED, "Use case not injected".to_string()))
}

