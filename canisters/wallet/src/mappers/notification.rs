use crate::{
    models::Notification,
    transport::{NotificationDTO, NotificationLocalizedTextDTO},
};
use ic_canister_core::utils::timestamp_to_rfc3339;
use uuid::Uuid;

impl From<Notification> for NotificationDTO {
    fn from(notification: Notification) -> NotificationDTO {
        NotificationDTO {
            id: Uuid::from_bytes(notification.id).hyphenated().to_string(),
            target_user_id: Uuid::from_bytes(notification.target_user_id)
                .hyphenated()
                .to_string(),
            status: notification.status.into(),
            title: NotificationLocalizedTextDTO {
                body: notification.title.0,
                locale_key: notification.title.1,
            },
            message: NotificationLocalizedTextDTO {
                body: notification.message.0,
                locale_key: notification.message.1,
            },
            notification_type: notification.notification_type.into(),
            created_at: timestamp_to_rfc3339(&notification.created_timestamp),
        }
    }
}
