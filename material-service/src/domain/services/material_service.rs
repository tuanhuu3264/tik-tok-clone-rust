use std::sync::Arc;
use crate::domain::repositories::MaterialRepository;
use crate::domain::value_objects::MaterialCode;
use crate::domain::errors::DomainError;

pub struct MaterialService<R: MaterialRepository> {
    material_repository: Arc<R>,
}

impl<R: MaterialRepository> MaterialService<R> {
    pub fn new(material_repository: Arc<R>) -> Self {
        Self { material_repository }
    }

    pub async fn is_code_taken(&self, code: &MaterialCode) -> Result<bool, DomainError> {
        let existing_material = self.material_repository.find_by_code(code).await?;
        Ok(existing_material.is_some())
    }

    pub async fn validate_material_creation(&self, code: &MaterialCode) -> Result<(), DomainError> {
        if self.is_code_taken(code).await? {
            return Err(DomainError::MaterialCodeAlreadyExists);
        }
        Ok(())
    }
}

