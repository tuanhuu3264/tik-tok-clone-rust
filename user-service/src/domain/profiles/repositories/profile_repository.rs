use std::sync::Arc;
use async_trait::async_trait;
use crate::domain::profiles::entities::profile::Profile;
use crate::domain::value_objects::UserId;
use crate::domain::errors::DomainError;

#[async_trait]
pub trait ProfileRepository: Send + Sync {
    async fn create(&self, profile: &Profile) -> Result<(), DomainError>;
    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<Profile>, DomainError>;
    async fn update(&self, profile: &Profile) -> Result<(), DomainError>;
    async fn delete(&self, user_id: &UserId) -> Result<(), DomainError>;
}

#[async_trait]
impl<R: ProfileRepository> ProfileRepository for Arc<R> {
    async fn create(&self, profile: &Profile) -> Result<(), DomainError> {
        (**self).create(profile).await
    }

    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Option<Profile>, DomainError> {
        (**self).find_by_user_id(user_id).await
    }

    async fn update(&self, profile: &Profile) -> Result<(), DomainError> {
        (**self).update(profile).await
    }

    async fn delete(&self, user_id: &UserId) -> Result<(), DomainError> {
        (**self).delete(user_id).await
    }
}

