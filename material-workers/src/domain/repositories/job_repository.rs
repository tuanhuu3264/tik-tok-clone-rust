use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::entities::material_job::MaterialJob;
use crate::domain::value_objects::JobId;
use crate::domain::errors::DomainError;

#[async_trait]
pub trait JobRepository: Send + Sync {
    async fn create(&self, job: &MaterialJob) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: &JobId) -> Result<Option<MaterialJob>, DomainError>;
    async fn find_pending_jobs(&self) -> Result<Vec<MaterialJob>, DomainError>;
    async fn update(&self, job: &MaterialJob) -> Result<(), DomainError>;
}

#[async_trait]
impl<R: JobRepository> JobRepository for Arc<R> {
    async fn create(&self, job: &MaterialJob) -> Result<(), DomainError> {
        (**self).create(job).await
    }

    async fn find_by_id(&self, id: &JobId) -> Result<Option<MaterialJob>, DomainError> {
        (**self).find_by_id(id).await
    }

    async fn find_pending_jobs(&self) -> Result<Vec<MaterialJob>, DomainError> {
        (**self).find_pending_jobs().await
    }

    async fn update(&self, job: &MaterialJob) -> Result<(), DomainError> {
        (**self).update(job).await
    }
}

