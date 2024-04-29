use crate::models::NotificationStatus;
use station_api::NotificationStatusDTO;

impl From<NotificationStatus> for NotificationStatusDTO {
    fn from(status: NotificationStatus) -> Self {
        match status {
            NotificationStatus::Sent => NotificationStatusDTO::Sent,
            NotificationStatus::Read => NotificationStatusDTO::Read,
        }
    }
}

impl From<NotificationStatusDTO> for NotificationStatus {
    fn from(status: NotificationStatusDTO) -> Self {
        match status {
            NotificationStatusDTO::Sent => NotificationStatus::Sent,
            NotificationStatusDTO::Read => NotificationStatus::Read,
        }
    }
}
