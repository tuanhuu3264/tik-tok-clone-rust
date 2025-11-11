use thiserror::Error;
use crate::domain::errors::DomainError;

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),

    #[error("Repository error: {0}")]
    Repository(String),

    #[error("Validation error: {0}")]
    Validation(String),
}

