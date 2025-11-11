use std::sync::Arc;
use std::str::FromStr;
use crate::application::dto::{CreateProductDto, ProductDto};
use crate::application::errors::ApplicationError;
use crate::domain::repositories::ProductRepository;
use crate::domain::services::CatalogService;
use crate::domain::entities::product::Product;
use crate::domain::value_objects::{ProductId, ProductName, SKU, Description, Price};
use rust_decimal::Decimal;

pub struct CreateProductUseCase<R: ProductRepository> {
    product_repository: Arc<R>,
    catalog_service: CatalogService<R>,
}

impl<R: ProductRepository> CreateProductUseCase<R> {
    pub fn new(product_repository: Arc<R>) -> Self {
        let catalog_service = CatalogService::new(Arc::clone(&product_repository));
        Self {
            product_repository,
            catalog_service,
        }
    }

    pub async fn execute(&self, dto: CreateProductDto) -> Result<ProductDto, ApplicationError> {
        let name = ProductName::new(dto.name)?;
        let sku = SKU::new(dto.sku)?;
        let description = Description::new(dto.description);
        
        let price_decimal = Decimal::from_str(&dto.price)
            .map_err(|_| ApplicationError::Validation("Invalid price format".to_string()))?;
        let price = Price::new(price_decimal);

        self.catalog_service.validate_product_creation(&sku).await?;

        let product = Product::new(ProductId::new(), name, sku, description, price);
        self.product_repository.create(&product).await
            .map_err(|e| ApplicationError::Repository(e.to_string()))?;

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

