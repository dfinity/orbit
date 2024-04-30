use crate::models::UserStatus;
use station_api::UserStatusDTO;

impl From<UserStatus> for UserStatusDTO {
    fn from(status: UserStatus) -> Self {
        match status {
            UserStatus::Active => UserStatusDTO::Active,
            UserStatus::Inactive => UserStatusDTO::Inactive,
        }
    }
}

impl From<UserStatusDTO> for UserStatus {
    fn from(status: UserStatusDTO) -> Self {
        match status {
            UserStatusDTO::Active => UserStatus::Active,
            UserStatusDTO::Inactive => UserStatus::Inactive,
        }
    }
}
