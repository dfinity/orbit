use crate::{models::system::SystemInfo, repositories::USER_GROUP_REPOSITORY};
use orbit_essentials::{
    repository::Repository,
    utils::{raw_rand_successful, timestamp_to_rfc3339},
};
use station_api::DisasterRecoveryDTO;

impl SystemInfo {
    pub fn to_dto(&self, cycles: &u64, version: &str) -> station_api::SystemInfoDTO {
        station_api::SystemInfoDTO {
            name: self.get_name().to_string(),
            last_upgrade_timestamp: timestamp_to_rfc3339(&self.get_last_upgrade_timestamp()),
            upgrader_id: *self.get_upgrader_canister_id(),
            cycles: *cycles,
            version: version.to_string(),
            raw_rand_successful: raw_rand_successful(),
            disaster_recovery: self.get_disaster_recovery_committee().map(|dr| {
                DisasterRecoveryDTO {
                    committee: dr.clone().into(),
                    user_group_name: USER_GROUP_REPOSITORY
                        .get(&dr.user_group_id)
                        .map(|g| g.name.clone()),
                }
            }),
            cycle_obtain_strategy: self.get_cycle_obtain_strategy().map(|s| (*s).into()),
        }
    }
}
