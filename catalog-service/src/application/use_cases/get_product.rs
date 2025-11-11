use std::sync::Arc;
use crate::application::dto::ProductDto;
use crate::application::errors::ApplicationError;
use crate::domain::repositories::ProductRepository;
use crate::domain::value_objects::ProductId;

pub struct GetProductUseCase<R: ProductRepository> {
    product_repository: Arc<R>,
}

impl<R: ProductRepository> GetProductUseCase<R> {
    pub fn new(product_repository: Arc<R>) -> Self {
        Self { product_repository }
    }

    pub async fn execute(&self, product_id: &str) -> Result<ProductDto, ApplicationError> {
        let uuid = uuid::Uuid::parse_str(product_id)
            .map_err(|_| ApplicationError::Validation("Invalid product ID format".to_string()))?;
        
        let product_id = ProductId::from_uuid(uuid);
        let product = self.product_repository.find_by_id(&product_id).await?
            .ok_or(ApplicationError::Domain(crate::domain::errors::DomainError::ProductNotFound))?;

        Ok(ProductDto {
            id: product.id.as_uuid().to_string(),
            name: product.name.as_str().to_string(),
            sku: product.sku.as_str().to_string(),
            description: product.description.as_str().to_string(),
            price: product.price.value().to_string(),
            is_active: product.is_active,
            created_at: product.created_at.to_rfc3339(),
        })
    }
}

