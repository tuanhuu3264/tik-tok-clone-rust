use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::entities::permission::Permission;
use crate::domain::value_objects::{PermissionId, PermissionName};
use crate::domain::errors::DomainError;

#[async_trait]
pub trait PermissionRepository: Send + Sync {
    async fn create(&self, permission: &Permission) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: &PermissionId) -> Result<Option<Permission>, DomainError>;
    async fn find_by_name(&self, name: &PermissionName) -> Result<Option<Permission>, DomainError>;
    async fn find_all(&self) -> Result<Vec<Permission>, DomainError>;
}

#[async_trait]
impl<R: PermissionRepository> PermissionRepository for Arc<R> {
    async fn create(&self, permission: &Permission) -> Result<(), DomainError> {
        (**self).create(permission).await
    }

    async fn find_by_id(&self, id: &PermissionId) -> Result<Option<Permission>, DomainError> {
        (**self).find_by_id(id).await
    }

    async fn find_by_name(&self, name: &PermissionName) -> Result<Option<Permission>, DomainError> {
        (**self).find_by_name(name).await
    }

    async fn find_all(&self) -> Result<Vec<Permission>, DomainError> {
        (**self).find_all().await
    }
}

