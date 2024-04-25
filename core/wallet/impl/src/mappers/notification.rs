use crate::models::Notification;
use ic_canister_core::{types::UUID, utils::timestamp_to_rfc3339};
use uuid::Uuid;
use wallet_api::NotificationDTO;

pub enum NotificationMapperError {
    ProposalNotFound { proposal_id: UUID },
}

impl TryFrom<Notification> for NotificationDTO {
    type Error = NotificationMapperError;
    fn try_from(notification: Notification) -> Result<NotificationDTO, NotificationMapperError> {
        Ok(NotificationDTO {
            id: Uuid::from_bytes(notification.id).hyphenated().to_string(),
            target_user_id: Uuid::from_bytes(notification.target_user_id)
                .hyphenated()
                .to_string(),
            status: notification.status.into(),
            title: notification.title,
            message: notification.message,
            notification_type: notification.notification_type.try_into()?,
            created_at: timestamp_to_rfc3339(&notification.created_timestamp),
        })
    }
}
