use thiserror::Error;
use crate::domain::errors::DomainError;

/// User-specific domain exceptions
#[derive(Debug, Error)]
pub enum UserException {
    #[error("User domain error: {0}")]
    Domain(#[from] DomainError),

    #[error("User not found with ID: {0}")]
    UserNotFound(String),

    #[error("User already exists with email: {0}")]
    UserAlreadyExists(String),

    #[error("User already exists with username: {0}")]
    UsernameAlreadyExists(String),

    #[error("Invalid user operation: {0}")]
    InvalidOperation(String),
}

