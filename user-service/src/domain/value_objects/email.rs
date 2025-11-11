use crate::domain::errors::DomainError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Email(String);

impl Email {
    pub fn new(email: String) -> Result<Self, DomainError> {
        if email.is_empty() {
            return Err(DomainError::InvalidEmail("Email cannot be empty".to_string()));
        }
        if !email.contains('@') {
            return Err(DomainError::InvalidEmail("Email must contain @".to_string()));
        }
        Ok(Self(email))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<Email> for String {
    fn from(email: Email) -> Self {
        email.0
    }
}

