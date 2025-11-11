use std::sync::Arc;
use crate::domain::repositories::ProductRepository;
use crate::domain::value_objects::SKU;
use crate::domain::errors::DomainError;

pub struct CatalogService<R: ProductRepository> {
    product_repository: Arc<R>,
}

impl<R: ProductRepository> CatalogService<R> {
    pub fn new(product_repository: Arc<R>) -> Self {
        Self { product_repository }
    }

    pub async fn is_sku_taken(&self, sku: &SKU) -> Result<bool, DomainError> {
        let existing_product = self.product_repository.find_by_sku(sku).await?;
        Ok(existing_product.is_some())
    }

    pub async fn validate_product_creation(&self, sku: &SKU) -> Result<(), DomainError> {
        if self.is_sku_taken(sku).await? {
            return Err(DomainError::SKUAlreadyExists);
        }
        Ok(())
    }
}

