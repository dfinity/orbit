use crate::models::system::SystemInfo;
use ic_canister_core::utils::{raw_rand_successful, timestamp_to_rfc3339};

impl SystemInfo {
    pub fn to_dto(&self, cycles: &u64, version: &str) -> station_api::SystemInfoDTO {
        station_api::SystemInfoDTO {
            last_upgrade_timestamp: timestamp_to_rfc3339(&self.get_last_upgrade_timestamp()),
            upgrader_id: *self.get_upgrader_canister_id(),
            cycles: *cycles,
            version: version.to_string(),
            raw_rand_successful: raw_rand_successful(),
        }
    }
}
