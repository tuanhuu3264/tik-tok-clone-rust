use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateMaterialDto {
    pub name: String,
    pub code: String,
    pub unit: String,
}

#[derive(Debug, Serialize)]
pub struct MaterialDto {
    pub id: String,
    pub name: String,
    pub code: String,
    pub unit: String,
    pub is_active: bool,
    pub created_at: String,
}

