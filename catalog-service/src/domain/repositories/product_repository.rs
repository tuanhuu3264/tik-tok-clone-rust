use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::entities::product::Product;
use crate::domain::value_objects::{ProductId, SKU};
use crate::domain::errors::DomainError;

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn create(&self, product: &Product) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: &ProductId) -> Result<Option<Product>, DomainError>;
    async fn find_by_sku(&self, sku: &SKU) -> Result<Option<Product>, DomainError>;
    async fn find_all(&self) -> Result<Vec<Product>, DomainError>;
    async fn update(&self, product: &Product) -> Result<(), DomainError>;
    async fn delete(&self, id: &ProductId) -> Result<(), DomainError>;
}

#[async_trait]
impl<R: ProductRepository> ProductRepository for Arc<R> {
    async fn create(&self, product: &Product) -> Result<(), DomainError> {
        (**self).create(product).await
    }

    async fn find_by_id(&self, id: &ProductId) -> Result<Option<Product>, DomainError> {
        (**self).find_by_id(id).await
    }

    async fn find_by_sku(&self, sku: &SKU) -> Result<Option<Product>, DomainError> {
        (**self).find_by_sku(sku).await
    }

    async fn find_all(&self) -> Result<Vec<Product>, DomainError> {
        (**self).find_all().await
    }

    async fn update(&self, product: &Product) -> Result<(), DomainError> {
        (**self).update(product).await
    }

    async fn delete(&self, id: &ProductId) -> Result<(), DomainError> {
        (**self).delete(id).await
    }
}

