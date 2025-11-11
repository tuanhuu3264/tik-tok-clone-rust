use std::sync::Arc;
use crate::application::dto::{CreateNotificationDto, NotificationDto};
use crate::application::errors::ApplicationError;
use crate::domain::repositories::{NotificationRepository, SubscriptionRepository};
use crate::domain::services::NotificationDomainService;
use crate::domain::entities::notification::Notification;
use crate::domain::value_objects::{NotificationId, UserId, Channel, Title, Message};

pub struct SendNotificationUseCase<NR: NotificationRepository, SR: SubscriptionRepository> {
    notification_repository: Arc<NR>,
    subscription_repository: Arc<SR>,
    notification_service: NotificationDomainService<SR>,
}

impl<NR: NotificationRepository, SR: SubscriptionRepository> SendNotificationUseCase<NR, SR> {
    pub fn new(notification_repository: Arc<NR>, subscription_repository: Arc<SR>) -> Self {
        let notification_service = NotificationDomainService::new(Arc::clone(&subscription_repository));
        Self {
            notification_repository,
            subscription_repository,
            notification_service,
        }
    }

    pub async fn execute(&self, dto: CreateNotificationDto) -> Result<NotificationDto, ApplicationError> {
        let user_uuid = uuid::Uuid::parse_str(&dto.user_id)
            .map_err(|_| ApplicationError::Validation("Invalid user ID format".to_string()))?;
        let user_id = UserId::from_uuid(user_uuid);

        let channel = match dto.channel.as_str() {
            "email" => Channel::Email,
            "sms" => Channel::SMS,
            "push" => Channel::Push,
            "in_app" => Channel::InApp,
            _ => return Err(ApplicationError::Validation("Invalid channel".to_string())),
        };

        // Check if user is subscribed to this channel
        let can_send = self.notification_service.can_send_notification(&user_id, &channel).await?;
        if !can_send {
            return Err(ApplicationError::Validation("User is not subscribed to this channel".to_string()));
        }

        let title = Title::new(dto.title);
        let message = Message::new(dto.message);

        let notification = Notification::new(
            NotificationId::new(),
            user_id,
            channel,
            title,
            message,
        );

        self.notification_repository.create(&notification).await
            .map_err(|e| ApplicationError::Repository(e.to_string()))?;

        Ok(NotificationDto {
            id: notification.id.as_uuid().to_string(),
            user_id: notification.user_id.as_uuid().to_string(),
            channel: notification.channel.as_str().to_string(),
            title: notification.title.as_str().to_string(),
            message: notification.message.as_str().to_string(),
            is_read: notification.is_read,
            created_at: notification.created_at.to_rfc3339(),
        })
    }
}

