use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct StatusId(Uuid);

impl StatusId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn as_uuid(&self) -> Uuid {
        self.0
    }
}

impl Default for StatusId {
    fn default() -> Self {
        Self::new()
    }
}

impl From<Uuid> for StatusId {
    fn from(uuid: Uuid) -> Self {
        Self(uuid)
    }
}

impl From<StatusId> for Uuid {
    fn from(status_id: StatusId) -> Self {
        status_id.0
    }
}

