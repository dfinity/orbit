use crate::models::system::SystemInfo;
use ic_canister_core::utils::timestamp_to_rfc3339;

impl SystemInfo {
    pub fn to_dto(&self, cycles: &u64, version: &str) -> wallet_api::SystemInfoDTO {
        wallet_api::SystemInfoDTO {
            last_upgrade_timestamp: timestamp_to_rfc3339(&self.last_upgrade_timestamp),
            upgrader_id: self.upgrader_canister_id,
            cycles: *cycles,
            version: version.to_string(),
        }
    }
}
