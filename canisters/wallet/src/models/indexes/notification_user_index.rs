use crate::models::{Notification, NotificationId, UserId};
use candid::{CandidType, Deserialize};
use ic_canister_core::types::Timestamp;
use ic_canister_macros::stable_object;

/// Index of notifications by user id.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NotificationUserIndex {
    /// The user id that is associated with this notification.
    pub user_id: UserId,
    /// The time when the notification was created.
    pub created_at: Timestamp,
    /// The notification id, which is a UUID.
    pub notification_id: NotificationId,
}

#[derive(Clone, Debug)]
pub struct NotificationUserIndexCriteria {
    pub user_id: UserId,
    pub from_dt: Option<Timestamp>,
    pub to_dt: Option<Timestamp>,
}

impl Notification {
    pub fn to_index_for_target_user(&self) -> NotificationUserIndex {
        NotificationUserIndex {
            notification_id: self.id.to_owned(),
            user_id: self.target_user_id.to_owned(),
            created_at: self.created_timestamp,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::notification_test_utils::mock_notification;
    use ic_stable_structures::Storable;

    #[test]
    fn valid_model_serialization() {
        let user_id = [0; 16];
        let notification_id = [1; 16];
        let model = NotificationUserIndex {
            user_id,
            notification_id,
            created_at: 0,
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = NotificationUserIndex::from_bytes(serialized_model);

        assert_eq!(model.notification_id, deserialized_model.notification_id);
        assert_eq!(model.user_id, deserialized_model.user_id);
    }

    #[test]
    fn correct_notification_user_index_mapping() {
        let mut notification = mock_notification();
        notification.id = [1; 16];
        notification.target_user_id = [2; 16];

        let index = notification.to_index_for_target_user();

        assert_eq!(index.notification_id, notification.id);
        assert_eq!(index.user_id, notification.target_user_id);
    }
}
