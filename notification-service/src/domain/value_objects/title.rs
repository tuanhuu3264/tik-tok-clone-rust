use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Title(String);

impl Title {
    pub fn new(title: String) -> Self {
        Self(title)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Title {
    fn from(title: String) -> Self {
        Self(title)
    }
}

impl From<Title> for String {
    fn from(title: Title) -> Self {
        title.0
    }
}

