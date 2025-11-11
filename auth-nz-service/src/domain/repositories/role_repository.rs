use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::entities::role::Role;
use crate::domain::value_objects::{RoleId, RoleName};
use crate::domain::errors::DomainError;

#[async_trait]
pub trait RoleRepository: Send + Sync {
    async fn create(&self, role: &Role) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: &RoleId) -> Result<Option<Role>, DomainError>;
    async fn find_by_name(&self, name: &RoleName) -> Result<Option<Role>, DomainError>;
    async fn find_all(&self) -> Result<Vec<Role>, DomainError>;
}

#[async_trait]
impl<R: RoleRepository> RoleRepository for Arc<R> {
    async fn create(&self, role: &Role) -> Result<(), DomainError> {
        (**self).create(role).await
    }

    async fn find_by_id(&self, id: &RoleId) -> Result<Option<Role>, DomainError> {
        (**self).find_by_id(id).await
    }

    async fn find_by_name(&self, name: &RoleName) -> Result<Option<Role>, DomainError> {
        (**self).find_by_name(name).await
    }

    async fn find_all(&self) -> Result<Vec<Role>, DomainError> {
        (**self).find_all().await
    }
}

