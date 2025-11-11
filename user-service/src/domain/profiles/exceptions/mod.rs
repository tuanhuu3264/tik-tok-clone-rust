use thiserror::Error;
use crate::domain::errors::DomainError;

/// Profile-specific domain exceptions
#[derive(Debug, Error)]
pub enum ProfileException {
    #[error("Profile domain error: {0}")]
    Domain(#[from] DomainError),

    #[error("Profile not found for user ID: {0}")]
    ProfileNotFound(String),

    #[error("Profile already exists for user ID: {0}")]
    ProfileAlreadyExists(String),

    #[error("Invalid profile operation: {0}")]
    InvalidOperation(String),

    #[error("Profile update failed: {0}")]
    UpdateFailed(String),
}

