use super::{NotificationStatus, NotificationType, UserId};
use crate::errors::NotificationError;
use orbit_essentials::storable;
use orbit_essentials::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};

/// The notification id, which is a UUID.
pub type NotificationId = UUID;

/// Represents a notification within the system.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Notification {
    pub id: NotificationId,
    pub notification_type: NotificationType,
    pub status: NotificationStatus,
    /// The user that the notification is targeted to.
    pub target_user_id: UserId,
    /// The title of the notification set in a single locale.
    pub title: String,
    /// The message of the notification set in a single locale.
    pub message: Option<String>,
    pub created_timestamp: Timestamp,
    pub last_modification_timestamp: Timestamp,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NotificationKey {
    pub id: NotificationId,
}

fn validate_title(title: &str) -> ModelValidatorResult<NotificationError> {
    if title.len() > Notification::MAX_TITLE_LEN as usize {
        return Err(NotificationError::ValidationError {
            info: format!(
                "Notification title exceeds the maximum allowed: {}",
                Notification::MAX_TITLE_LEN
            ),
        });
    }

    Ok(())
}

fn validate_message(message: &Option<String>) -> ModelValidatorResult<NotificationError> {
    if let Some(message) = message {
        if message.len() > Notification::MAX_MESSAGE_LEN as usize {
            return Err(NotificationError::ValidationError {
                info: format!(
                    "Notification message exceeds the maximum allowed: {}",
                    Notification::MAX_MESSAGE_LEN
                ),
            });
        }
    }

    Ok(())
}

impl ModelValidator<NotificationError> for Notification {
    fn validate(&self) -> ModelValidatorResult<NotificationError> {
        validate_title(&self.title)?;
        validate_message(&self.message)?;

        Ok(())
    }
}

impl Notification {
    pub const MAX_TITLE_LEN: u8 = 255;
    pub const MAX_MESSAGE_LEN: u32 = 4096;

    pub fn key(id: NotificationId) -> NotificationKey {
        NotificationKey { id }
    }

    pub fn to_key(&self) -> NotificationKey {
        Notification::key(self.id.to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::notification_test_utils::mock_notification;

    #[test]
    fn fail_notification_title_too_long() {
        let mut notitication = mock_notification();
        notitication.title = "a".repeat(Notification::MAX_TITLE_LEN as usize + 1);

        let result = validate_title(&notitication.title);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            NotificationError::ValidationError {
                info: format!(
                    "Notification title exceeds the maximum allowed: {}",
                    Notification::MAX_TITLE_LEN
                )
            }
        );
    }

    #[test]
    fn fail_notification_message_too_long() {
        let mut notitication = mock_notification();
        notitication.message = Some("a".repeat(Notification::MAX_MESSAGE_LEN as usize + 1));

        let result = validate_message(&notitication.message);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            NotificationError::ValidationError {
                info: format!(
                    "Notification message exceeds the maximum allowed: {}",
                    Notification::MAX_MESSAGE_LEN
                )
            }
        );
    }

    #[test]
    fn test_notification_validation() {
        let notitication = mock_notification();

        let result = notitication.validate();

        assert!(result.is_ok());
    }
}

#[cfg(test)]
pub mod notification_test_utils {
    use super::*;
    use crate::models::{NotificationStatus, NotificationType};

    pub fn mock_notification() -> Notification {
        Notification {
            id: [0; 16],
            status: NotificationStatus::Sent,
            target_user_id: [1; 16],
            message: Some("message".to_string()),
            title: "title".to_string(),
            notification_type: NotificationType::SystemMessage,
            created_timestamp: 0,
            last_modification_timestamp: 0,
        }
    }
}
