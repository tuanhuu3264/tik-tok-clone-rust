use std::sync::Arc;
use crate::application::dto::MaterialDto;
use crate::application::errors::ApplicationError;
use crate::domain::repositories::MaterialRepository;
use crate::domain::value_objects::MaterialId;

pub struct GetMaterialUseCase<R: MaterialRepository> {
    material_repository: Arc<R>,
}

impl<R: MaterialRepository> GetMaterialUseCase<R> {
    pub fn new(material_repository: Arc<R>) -> Self {
        Self { material_repository }
    }

    pub async fn execute(&self, material_id: &str) -> Result<MaterialDto, ApplicationError> {
        let uuid = uuid::Uuid::parse_str(material_id)
            .map_err(|_| ApplicationError::Validation("Invalid material ID format".to_string()))?;
        
        let material_id = MaterialId::from_uuid(uuid);
        let material = self.material_repository.find_by_id(&material_id).await?
            .ok_or(ApplicationError::Domain(crate::domain::errors::DomainError::MaterialNotFound))?;

        Ok(MaterialDto {
            id: material.id.as_uuid().to_string(),
            name: material.name.as_str().to_string(),
            code: material.code.as_str().to_string(),
            unit: material.unit.as_str().to_string(),
            is_active: material.is_active,
            created_at: material.created_at.to_rfc3339(),
        })
    }
}

