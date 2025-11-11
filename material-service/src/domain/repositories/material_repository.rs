use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::entities::material::Material;
use crate::domain::value_objects::{MaterialId, MaterialCode};
use crate::domain::errors::DomainError;

#[async_trait]
pub trait MaterialRepository: Send + Sync {
    async fn create(&self, material: &Material) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: &MaterialId) -> Result<Option<Material>, DomainError>;
    async fn find_by_code(&self, code: &MaterialCode) -> Result<Option<Material>, DomainError>;
    async fn find_all(&self) -> Result<Vec<Material>, DomainError>;
    async fn update(&self, material: &Material) -> Result<(), DomainError>;
}

#[async_trait]
impl<R: MaterialRepository> MaterialRepository for Arc<R> {
    async fn create(&self, material: &Material) -> Result<(), DomainError> {
        (**self).create(material).await
    }

    async fn find_by_id(&self, id: &MaterialId) -> Result<Option<Material>, DomainError> {
        (**self).find_by_id(id).await
    }

    async fn find_by_code(&self, code: &MaterialCode) -> Result<Option<Material>, DomainError> {
        (**self).find_by_code(code).await
    }

    async fn find_all(&self) -> Result<Vec<Material>, DomainError> {
        (**self).find_all().await
    }

    async fn update(&self, material: &Material) -> Result<(), DomainError> {
        (**self).update(material).await
    }
}

