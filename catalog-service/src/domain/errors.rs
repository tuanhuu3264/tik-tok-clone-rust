use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("Invalid product name: {0}")]
    InvalidProductName(String),

    #[error("Invalid SKU: {0}")]
    InvalidSKU(String),

    #[error("Invalid category name: {0}")]
    InvalidCategoryName(String),

    #[error("Product not found")]
    ProductNotFound,

    #[error("Category not found")]
    CategoryNotFound,

    #[error("Insufficient stock")]
    InsufficientStock,

    #[error("Product already exists")]
    ProductAlreadyExists,

    #[error("SKU already exists")]
    SKUAlreadyExists,

    #[error("Domain validation error: {0}")]
    ValidationError(String),
}

