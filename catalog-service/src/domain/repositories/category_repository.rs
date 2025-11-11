use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::entities::category::Category;
use crate::domain::value_objects::CategoryId;
use crate::domain::errors::DomainError;

#[async_trait]
pub trait CategoryRepository: Send + Sync {
    async fn create(&self, category: &Category) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: &CategoryId) -> Result<Option<Category>, DomainError>;
    async fn find_all(&self) -> Result<Vec<Category>, DomainError>;
    async fn update(&self, category: &Category) -> Result<(), DomainError>;
}

#[async_trait]
impl<R: CategoryRepository> CategoryRepository for Arc<R> {
    async fn create(&self, category: &Category) -> Result<(), DomainError> {
        (**self).create(category).await
    }

    async fn find_by_id(&self, id: &CategoryId) -> Result<Option<Category>, DomainError> {
        (**self).find_by_id(id).await
    }

    async fn find_all(&self) -> Result<Vec<Category>, DomainError> {
        (**self).find_all().await
    }

    async fn update(&self, category: &Category) -> Result<(), DomainError> {
        (**self).update(category).await
    }
}

