use super::{TimestampRfc3339, UserDTO};
use candid::{CandidType, Deserialize, Principal};

#[derive(CandidType, Deserialize, Clone, Debug, Default)]
pub struct WalletCanisterInit {
    pub owners: Option<Vec<Principal>>,
}

#[derive(CandidType, Deserialize, Clone, Debug, Default)]
pub struct WalletSettingsDTO {
    pub owners: Vec<UserDTO>,
    pub last_upgrade_timestamp: TimestampRfc3339,
}

#[derive(CandidType, Deserialize, Clone, Debug, Default)]
pub struct WalletSettingsResponse {
    pub settings: WalletSettingsDTO,
}
