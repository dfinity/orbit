use super::{TimestampRfc3339, UserDTO, UserRoleDTO};
use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct WalletPermissionDTO {
    pub permission_id: String,
    pub access_roles: Vec<UserRoleDTO>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Default)]
pub struct WalletCanisterInit {
    pub owners: Option<Vec<Principal>>,
    pub approval_threshold: Option<u8>,
    pub permissions: Option<Vec<WalletPermissionDTO>>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Default)]
pub struct WalletSettingsDTO {
    pub approval_threshold: u8,
    pub permissions: Vec<WalletPermissionDTO>,
    pub owners: Vec<UserDTO>,
    pub last_upgrade_timestamp: TimestampRfc3339,
}

#[derive(CandidType, Deserialize, Clone, Debug, Default)]
pub struct WalletSettingsResponse {
    pub settings: WalletSettingsDTO,
}
