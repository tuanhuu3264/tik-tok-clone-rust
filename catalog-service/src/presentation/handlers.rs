use axum::{extract::Path, http::StatusCode, Json};
use crate::application::dto::{CreateProductDto, ProductDto};

pub async fn health_check() -> &'static str {
    "OK"
}

pub async fn create_product(
    Json(dto): Json<CreateProductDto>,
) -> Result<(StatusCode, Json<ProductDto>), (StatusCode, String)> {
    Err((StatusCode::NOT_IMPLEMENTED, "Use case not injected".to_string()))
}

pub async fn get_product(
    Path(id): Path<String>,
) -> Result<Json<ProductDto>, (StatusCode, String)> {
    Err((StatusCode::NOT_IMPLEMENTED, "Use case not injected".to_string()))
}

