use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::entities::material_type::MaterialType;
use crate::domain::value_objects::MaterialTypeId;
use crate::domain::errors::DomainError;

#[async_trait]
pub trait MaterialTypeRepository: Send + Sync {
    async fn create(&self, material_type: &MaterialType) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: &MaterialTypeId) -> Result<Option<MaterialType>, DomainError>;
    async fn find_all(&self) -> Result<Vec<MaterialType>, DomainError>;
}

#[async_trait]
impl<R: MaterialTypeRepository> MaterialTypeRepository for Arc<R> {
    async fn create(&self, material_type: &MaterialType) -> Result<(), DomainError> {
        (**self).create(material_type).await
    }

    async fn find_by_id(&self, id: &MaterialTypeId) -> Result<Option<MaterialType>, DomainError> {
        (**self).find_by_id(id).await
    }

    async fn find_all(&self) -> Result<Vec<MaterialType>, DomainError> {
        (**self).find_all().await
    }
}

