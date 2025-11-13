use crate::domain::profiles::repositories::ProfileRepository;
use crate::domain::value_objects::UserId;
use crate::domain::errors::DomainError;
use std::sync::Arc;

pub struct GetProfileUseCase {
    profile_repository: Arc<dyn ProfileRepository>,
}

impl GetProfileUseCase {
    pub fn new(profile_repository: Arc<dyn ProfileRepository>) -> Self {
        Self { profile_repository }
    }

    pub async fn execute(&self, user_id: UserId) -> Result<GetProfileResult, DomainError> {
        let profile = self.profile_repository
            .find_by_user_id(&user_id)
            .await?
            .ok_or(DomainError::ProfileNotFound)?;

        Ok(GetProfileResult { profile })
    }
}

pub struct GetProfileResult {
    pub profile: crate::domain::profiles::entities::profile::Profile,
}

