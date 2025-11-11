use axum::{Router, routing::get, routing::post};
use crate::presentation::handlers;

pub async fn create_router() -> Result<Router, Box<dyn std::error::Error>> {
    let router = Router::new()
        .route("/health", get(handlers::health_check))
        .route("/notifications", post(handlers::send_notification))
        .route("/notifications/:user_id", get(handlers::get_notifications));

    Ok(router)
}

