use thiserror::Error;

/// Domain Layer Errors
#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("Invalid email: {0}")]
    InvalidEmail(String),

    #[error("Invalid username: {0}")]
    InvalidUsername(String),

    #[error("User not found")]
    UserNotFound,

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Profile not found")]
    ProfileNotFound,

    #[error("Profile already exists")]
    ProfileAlreadyExists,

    #[error("Domain validation error: {0}")]
    ValidationError(String),
}

