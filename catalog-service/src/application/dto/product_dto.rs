use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreateProductDto {
    pub name: String,
    pub sku: String,
    pub description: String,
    pub price: String,
}

#[derive(Debug, Serialize)]
pub struct ProductDto {
    pub id: String,
    pub name: String,
    pub sku: String,
    pub description: String,
    pub price: String,
    pub is_active: bool,
    pub created_at: String,
}

