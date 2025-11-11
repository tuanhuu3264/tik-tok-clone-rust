use axum::{extract::Path, http::StatusCode, Json};
use crate::application::dto::{CreateMaterialDto, MaterialDto};

pub async fn health_check() -> &'static str {
    "OK"
}

pub async fn create_material(
    Json(dto): Json<CreateMaterialDto>,
) -> Result<(StatusCode, Json<MaterialDto>), (StatusCode, String)> {
    Err((StatusCode::NOT_IMPLEMENTED, "Use case not injected".to_string()))
}

pub async fn get_material(
    Path(id): Path<String>,
) -> Result<Json<MaterialDto>, (StatusCode, String)> {
    Err((StatusCode::NOT_IMPLEMENTED, "Use case not injected".to_string()))
}

