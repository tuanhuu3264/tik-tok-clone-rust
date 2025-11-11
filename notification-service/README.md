# Notification Service - DDD Architecture

Notification service với DDD architecture.

## Domain Entities

- **Notification**: Notification entity với title, message, channel
- **Subscription**: User subscriptions cho các channels
- **Channel**: Notification channels (Email, SMS, Push, InApp)

## Value Objects

- NotificationId, UserId, Channel, Title, Message
- SubscriptionId

## Use Cases

- SendNotification: Gửi notification
- GetNotifications: Lấy danh sách notifications của user

## Cấu trúc

Tương tự như user-service với các thư mục domain:
- `domain/entities/` - Notification, Subscription, Channel
- `domain/value_objects/` - Các value objects
- `domain/repositories/` - Repository traits
- `domain/services/` - NotificationDomainService
- `domain/errors.rs` - Domain errors

