use axum::{Router, routing::get, routing::post};
use crate::presentation::handlers;

pub async fn create_router() -> Result<Router, Box<dyn std::error::Error>> {
    let router = Router::new()
        .route("/health", get(handlers::health_check))
        .route("/register", post(handlers::register))
        .route("/login", post(handlers::login))
        .route("/logout", post(handlers::logout));

    Ok(router)
}

