use std::sync::Arc;
use crate::application::dto::{CreateMaterialDto, MaterialDto};
use crate::application::errors::ApplicationError;
use crate::domain::repositories::MaterialRepository;
use crate::domain::services::MaterialService;
use crate::domain::entities::material::Material;
use crate::domain::value_objects::{MaterialId, MaterialName, MaterialCode, Unit};

pub struct CreateMaterialUseCase<R: MaterialRepository> {
    material_repository: Arc<R>,
    material_service: MaterialService<R>,
}

impl<R: MaterialRepository> CreateMaterialUseCase<R> {
    pub fn new(material_repository: Arc<R>) -> Self {
        let material_service = MaterialService::new(Arc::clone(&material_repository));
        Self {
            material_repository,
            material_service,
        }
    }

    pub async fn execute(&self, dto: CreateMaterialDto) -> Result<MaterialDto, ApplicationError> {
        let name = MaterialName::new(dto.name)?;
        let code = MaterialCode::new(dto.code)?;
        
        let unit = match dto.unit.as_str() {
            "kg" => Unit::Kilogram,
            "g" => Unit::Gram,
            "L" => Unit::Liter,
            "mL" => Unit::Milliliter,
            "pcs" => Unit::Piece,
            "m" => Unit::Meter,
            "cm" => Unit::Centimeter,
            _ => return Err(ApplicationError::Validation("Invalid unit".to_string())),
        };

        self.material_service.validate_material_creation(&code).await?;

        let material = Material::new(MaterialId::new(), name, code, unit);
        self.material_repository.create(&material).await
            .map_err(|e| ApplicationError::Repository(e.to_string()))?;

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

