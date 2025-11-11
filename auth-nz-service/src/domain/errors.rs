use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("Invalid email: {0}")]
    InvalidEmail(String),

    #[error("Invalid password")]
    InvalidPassword,

    #[error("User not found")]
    UserNotFound,

    #[error("User already exists")]
    UserAlreadyExists,

    #[error("Session not found")]
    SessionNotFound,

    #[error("Session expired")]
    SessionExpired,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Invalid role name: {0}")]
    InvalidRoleName(String),

    #[error("Invalid permission name: {0}")]
    InvalidPermissionName(String),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Domain validation error: {0}")]
    ValidationError(String),
}

