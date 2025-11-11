use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateCategoryDto {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CategoryDto {
    pub id: String,
    pub name: String,
    pub is_active: bool,
    pub created_at: String,
}

