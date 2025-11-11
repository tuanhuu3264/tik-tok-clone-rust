use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum DomainError {
    #[error("Notification not found")]
    NotificationNotFound,

    #[error("Subscription not found")]
    SubscriptionNotFound,

    #[error("Invalid channel")]
    InvalidChannel,

    #[error("Domain validation error: {0}")]
    ValidationError(String),
}

