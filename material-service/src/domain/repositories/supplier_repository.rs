use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::entities::supplier::Supplier;
use crate::domain::value_objects::SupplierId;
use crate::domain::errors::DomainError;

#[async_trait]
pub trait SupplierRepository: Send + Sync {
    async fn create(&self, supplier: &Supplier) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: &SupplierId) -> Result<Option<Supplier>, DomainError>;
    async fn find_all(&self) -> Result<Vec<Supplier>, DomainError>;
    async fn update(&self, supplier: &Supplier) -> Result<(), DomainError>;
}

#[async_trait]
impl<R: SupplierRepository> SupplierRepository for Arc<R> {
    async fn create(&self, supplier: &Supplier) -> Result<(), DomainError> {
        (**self).create(supplier).await
    }

    async fn find_by_id(&self, id: &SupplierId) -> Result<Option<Supplier>, DomainError> {
        (**self).find_by_id(id).await
    }

    async fn find_all(&self) -> Result<Vec<Supplier>, DomainError> {
        (**self).find_all().await
    }

    async fn update(&self, supplier: &Supplier) -> Result<(), DomainError> {
        (**self).update(supplier).await
    }
}

