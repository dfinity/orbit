use orbit_essentials::storable;
use orbit_essentials::types::UUID;
use station_api::{
    REQUEST_CREATED_NOTIFICATION_TYPE, REQUEST_FAILED_NOTIFICATION_TYPE,
    REQUEST_REJECTED_NOTIFICATION_TYPE, SYSTEM_MESSAGE_NOTIFICATION_TYPE,
};
use std::fmt::{Display, Formatter};

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NotificationType {
    SystemMessage,
    RequestCreated(RequestCreatedNotification),
    RequestFailed(RequestFailedNotification),
    RequestRejected(RequestRejectedNotification),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RequestNotification {
    pub request_id: UUID,
}

pub type RequestCreatedNotification = RequestNotification;
pub type RequestFailedNotification = RequestNotification;
pub type RequestRejectedNotification = RequestNotification;

impl Display for NotificationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationType::SystemMessage => write!(f, "{}", SYSTEM_MESSAGE_NOTIFICATION_TYPE),
            NotificationType::RequestCreated(_) => {
                write!(f, "{}", REQUEST_CREATED_NOTIFICATION_TYPE)
            }
            NotificationType::RequestFailed(_) => {
                write!(f, "{}", REQUEST_FAILED_NOTIFICATION_TYPE)
            }
            NotificationType::RequestRejected(_) => {
                write!(f, "{}", REQUEST_REJECTED_NOTIFICATION_TYPE)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_string_representation() {
        assert_eq!(
            NotificationType::SystemMessage.to_string(),
            "system-message"
        );
        assert_eq!(
            NotificationType::RequestCreated(RequestCreatedNotification {
                request_id: [0; 16]
            })
            .to_string(),
            "request-created"
        );

        assert_eq!(
            NotificationType::RequestFailed(RequestFailedNotification {
                request_id: [0; 16]
            })
            .to_string(),
            "request-failed"
        );

        assert_eq!(
            NotificationType::RequestRejected(RequestRejectedNotification {
                request_id: [0; 16]
            })
            .to_string(),
            "request-rejected"
        );
    }
}
