use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("Invalid material name: {0}")]
    InvalidMaterialName(String),

    #[error("Invalid material code: {0}")]
    InvalidMaterialCode(String),

    #[error("Invalid material type name: {0}")]
    InvalidMaterialTypeName(String),

    #[error("Invalid supplier name: {0}")]
    InvalidSupplierName(String),

    #[error("Invalid email: {0}")]
    InvalidEmail(String),

    #[error("Material not found")]
    MaterialNotFound,

    #[error("Material code already exists")]
    MaterialCodeAlreadyExists,

    #[error("Domain validation error: {0}")]
    ValidationError(String),
}

