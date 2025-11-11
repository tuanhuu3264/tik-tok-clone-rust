use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::entities::inventory::Inventory;
use crate::domain::value_objects::ProductId;
use crate::domain::errors::DomainError;

#[async_trait]
pub trait InventoryRepository: Send + Sync {
    async fn create(&self, inventory: &Inventory) -> Result<(), DomainError>;
    async fn find_by_product_id(&self, product_id: &ProductId) -> Result<Option<Inventory>, DomainError>;
    async fn update(&self, inventory: &Inventory) -> Result<(), DomainError>;
}

#[async_trait]
impl<R: InventoryRepository> InventoryRepository for Arc<R> {
    async fn create(&self, inventory: &Inventory) -> Result<(), DomainError> {
        (**self).create(inventory).await
    }

    async fn find_by_product_id(&self, product_id: &ProductId) -> Result<Option<Inventory>, DomainError> {
        (**self).find_by_product_id(product_id).await
    }

    async fn update(&self, inventory: &Inventory) -> Result<(), DomainError> {
        (**self).update(inventory).await
    }
}

