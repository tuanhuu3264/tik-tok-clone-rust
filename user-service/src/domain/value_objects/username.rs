use crate::domain::errors::DomainError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Username(String);

impl Username {
    pub fn new(username: String) -> Result<Self, DomainError> {
        if username.is_empty() {
            return Err(DomainError::InvalidUsername("Username cannot be empty".to_string()));
        }
        if username.len() > 50 {
            return Err(DomainError::InvalidUsername("Username too long".to_string()));
        }
        Ok(Self(username))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<Username> for String {
    fn from(username: Username) -> Self {
        username.0
    }
}

