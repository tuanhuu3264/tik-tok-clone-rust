use std::sync::Arc;
use crate::application::dto::NotificationDto;
use crate::application::errors::ApplicationError;
use crate::domain::repositories::NotificationRepository;
use crate::domain::value_objects::UserId;

pub struct GetNotificationsUseCase<R: NotificationRepository> {
    notification_repository: Arc<R>,
}

impl<R: NotificationRepository> GetNotificationsUseCase<R> {
    pub fn new(notification_repository: Arc<R>) -> Self {
        Self { notification_repository }
    }

    pub async fn execute(&self, user_id: &str) -> Result<Vec<NotificationDto>, ApplicationError> {
        let user_uuid = uuid::Uuid::parse_str(user_id)
            .map_err(|_| ApplicationError::Validation("Invalid user ID format".to_string()))?;
        let user_id = UserId::from_uuid(user_uuid);

        let notifications = self.notification_repository.find_by_user_id(&user_id).await
            .map_err(|e| ApplicationError::Repository(e.to_string()))?;

        Ok(notifications.into_iter().map(|n| NotificationDto {
            id: n.id.as_uuid().to_string(),
            user_id: n.user_id.as_uuid().to_string(),
            channel: n.channel.as_str().to_string(),
            title: n.title.as_str().to_string(),
            message: n.message.as_str().to_string(),
            is_read: n.is_read,
            created_at: n.created_at.to_rfc3339(),
        }).collect())
    }
}

