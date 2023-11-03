use super::{TimestampRfc3339, UserDTO, UserRoleDTO};
use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct BankPermissionDTO {
    pub permission_id: String,
    pub access_roles: Vec<UserRoleDTO>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Default)]
pub struct BankCanisterInit {
    pub owners: Option<Vec<Principal>>,
    pub approval_threshold: Option<u8>,
    pub permissions: Option<Vec<BankPermissionDTO>>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Default)]
pub struct BankSettingsDTO {
    pub approval_threshold: u8,
    pub permissions: Vec<BankPermissionDTO>,
    pub owners: Vec<UserDTO>,
    pub last_upgrade_timestamp: TimestampRfc3339,
}

#[derive(CandidType, Deserialize, Clone, Debug, Default)]
pub struct BankSettingsResponse {
    pub settings: BankSettingsDTO,
}
