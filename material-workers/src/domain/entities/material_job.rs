use crate::domain::value_objects::{JobId, MaterialId};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MaterialJobType {
    ProcessMaterial,
    UpdateInventory,
    SyncSupplier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialJob {
    pub id: JobId,
    pub job_type: MaterialJobType,
    pub material_id: Option<MaterialId>,
    pub payload: serde_json::Value,
    pub status: JobStatus,
    pub retry_count: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

impl MaterialJob {
    pub fn new(
        id: JobId,
        job_type: MaterialJobType,
        material_id: Option<MaterialId>,
        payload: serde_json::Value,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            job_type,
            material_id,
            payload,
            status: JobStatus::Pending,
            retry_count: 0,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn mark_processing(&mut self) {
        self.status = JobStatus::Processing;
        self.updated_at = Utc::now();
    }

    pub fn mark_completed(&mut self) {
        self.status = JobStatus::Completed;
        self.updated_at = Utc::now();
    }

    pub fn mark_failed(&mut self) {
        self.status = JobStatus::Failed;
        self.retry_count += 1;
        self.updated_at = Utc::now();
    }

    pub fn can_retry(&self, max_retries: u32) -> bool {
        self.retry_count < max_retries
    }
}

