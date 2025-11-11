use std::sync::Arc;
use crate::domain::entities::material::Material;
use crate::domain::repositories::MaterialRepository;
use crate::domain::errors::DomainError;

pub struct MaterialProcessor<R: MaterialRepository> {
    material_repository: Arc<R>,
}

impl<R: MaterialRepository> MaterialProcessor<R> {
    pub fn new(material_repository: Arc<R>) -> Self {
        Self { material_repository }
    }

    pub async fn process_material(&self, material: &mut Material) -> Result<(), DomainError> {
        // Business logic để xử lý material
        // Ví dụ: validate, transform, enrich data
        material.updated_at = chrono::Utc::now();
        self.material_repository.update(material).await?;
        Ok(())
    }

    pub async fn update_inventory(&self, material: &mut Material) -> Result<(), DomainError> {
        // Business logic để update inventory
        material.updated_at = chrono::Utc::now();
        self.material_repository.update(material).await?;
        Ok(())
    }
}

