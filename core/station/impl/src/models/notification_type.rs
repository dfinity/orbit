use orbit_essentials::storable;
use orbit_essentials::types::UUID;
use station_api::{REQUEST_CREATED_NOTIFICATION_TYPE, SYSTEM_MESSAGE_NOTIFICATION_TYPE};
use std::fmt::{Display, Formatter};

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NotificationType {
    SystemMessage,
    RequestCreated(RequestCreatedNotification),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RequestCreatedNotification {
    pub request_id: UUID,
}

impl Display for NotificationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationType::SystemMessage => write!(f, "{}", SYSTEM_MESSAGE_NOTIFICATION_TYPE),
            NotificationType::RequestCreated(_) => {
                write!(f, "{}", REQUEST_CREATED_NOTIFICATION_TYPE)
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
    }
}
