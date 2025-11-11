use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Description(String);

impl Description {
    pub fn new(description: String) -> Self {
        Self(description)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Description {
    fn from(description: String) -> Self {
        Self(description)
    }
}

impl From<Description> for String {
    fn from(description: Description) -> Self {
        description.0
    }
}

