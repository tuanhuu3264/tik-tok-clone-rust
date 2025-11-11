use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Message(String);

impl Message {
    pub fn new(message: String) -> Self {
        Self(message)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for Message {
    fn from(message: String) -> Self {
        Self(message)
    }
}

impl From<Message> for String {
    fn from(message: Message) -> Self {
        message.0
    }
}

