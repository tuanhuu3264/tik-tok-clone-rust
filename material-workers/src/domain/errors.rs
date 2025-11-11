use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("Material not found")]
    MaterialNotFound,

    #[error("Job not found")]
    JobNotFound,

    #[error("Job processing failed")]
    JobProcessingFailed,

    #[error("Max retries exceeded")]
    MaxRetriesExceeded,

    #[error("Domain validation error: {0}")]
    ValidationError(String),
}

