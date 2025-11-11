use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PasswordHash(String);

impl PasswordHash {
    pub fn new(hash: String) -> Self {
        Self(hash)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for PasswordHash {
    fn from(hash: String) -> Self {
        Self(hash)
    }
}

impl From<PasswordHash> for String {
    fn from(hash: PasswordHash) -> Self {
        hash.0
    }
}

