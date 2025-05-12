use crate::{
    core::{
        ic_cdk::api::{time, trap},
        SYSTEM_RESERVED_MEMORY_BYTES,
    },
    STABLE_MEMORY_VERSION, SYSTEM_VERSION,
};
use candid::Principal;
use ic_cdk::call;
use ic_stable_structures::{storable::Bound, Storable};
use orbit_essentials::backup_snapshots::{default_max_backup_snapshots, BackupSnapshots};
use orbit_essentials::storable;
use orbit_essentials::types::{Timestamp, UUID};
use std::borrow::Cow;

use super::{AccountId, UserGroupId};

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SystemState {
    Uninitialized, // This state is only used between wasm module instantiation and init().
    Initialized(SystemInfo),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DisasterRecoveryCommittee {
    pub user_group_id: UserGroupId,
    pub quorum: u16,
}

#[storable]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum CycleObtainStrategy {
    #[default]
    Disabled,
    MintFromNativeToken {
        account_id: AccountId,
    },
    WithdrawFromCyclesLedger {
        account_id: AccountId,
    },
}

#[storable(size = SYSTEM_RESERVED_MEMORY_BYTES)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemInfo {
    /// The system name.
    name: String,
    /// Last time the canister was upgraded or initialized.
    last_upgrade_timestamp: Timestamp,
    /// An optionally pending change canister request.
    change_canister_request: Option<UUID>,
    /// The upgrader canister id that is allowed to upgrade this canister.
    upgrader_canister_id: Option<Principal>,
    /// The upgrader canister wasm module.
    #[serde(deserialize_with = "orbit_essentials::deserialize::deserialize_option_blob")]
    upgrader_wasm_module: Option<Vec<u8>>,
    /// A rolling window of upgrader backup snapshots.
    #[serde(default)]
    upgrader_backup_snapshots: BackupSnapshots,
    /// The disaster recovery committee user group id.
    disaster_recovery_committee: Option<DisasterRecoveryCommittee>,
    /// Defines how the station tops up itself with cycles.
    #[serde(default)]
    cycle_obtain_strategy: CycleObtainStrategy,
    /// The system version.
    version: Option<String>,
    /// Last run migration version.
    stable_memory_version: Option<u32>,
    /// The maximum number of station backup snapshots to keep.
    #[serde(default = "default_max_backup_snapshots")]
    max_station_backup_snapshots: u64,
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self {
            name: "Station".to_string(),
            last_upgrade_timestamp: time(),
            change_canister_request: None,
            upgrader_canister_id: None,
            upgrader_wasm_module: None,
            upgrader_backup_snapshots: BackupSnapshots::default(),
            disaster_recovery_committee: None,
            version: Some(SYSTEM_VERSION.to_string()),
            stable_memory_version: Some(STABLE_MEMORY_VERSION),
            cycle_obtain_strategy: CycleObtainStrategy::default(),
            max_station_backup_snapshots: default_max_backup_snapshots(),
        }
    }
}

impl SystemInfo {
    pub const MAX_NAME_LENGTH: usize = 48;

    pub fn new(upgrader_canister_id: Principal, upgrader_wasm_module: Vec<u8>) -> Self {
        Self {
            upgrader_canister_id: Some(upgrader_canister_id),
            upgrader_wasm_module: Some(upgrader_wasm_module),
            ..Default::default()
        }
    }

    pub fn get_stable_memory_version(&self) -> u32 {
        self.stable_memory_version.unwrap_or(0)
    }

    pub fn set_stable_memory_version(&mut self, version: u32) {
        self.stable_memory_version = Some(version);
    }

    pub fn get_cycle_obtain_strategy(&self) -> &CycleObtainStrategy {
        &self.cycle_obtain_strategy
    }

    pub fn set_cycle_obtain_strategy(&mut self, strategy: CycleObtainStrategy) {
        self.cycle_obtain_strategy = strategy;
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_version(&self) -> &str {
        self.version.as_deref().unwrap_or("0.0.0")
    }

    pub fn set_version(&mut self, version: String) {
        self.version = Some(version);
    }

    pub fn set_name(&mut self, name: String) {
        let mut name = name.trim().to_string();
        if name.is_empty() {
            name = "Station".to_string();
        }

        if name.len() > Self::MAX_NAME_LENGTH {
            name = name.chars().take(Self::MAX_NAME_LENGTH).collect();
        }

        self.name = name;
    }

    pub fn get_last_upgrade_timestamp(&self) -> Timestamp {
        self.last_upgrade_timestamp
    }

    pub fn get_change_canister_request(&self) -> Option<&UUID> {
        self.change_canister_request.as_ref()
    }

    pub fn get_upgrader_canister_id(&self) -> &Principal {
        self.upgrader_canister_id
            .as_ref()
            .expect("upgrader_canister_id is not set")
    }

    pub fn get_upgrader_wasm_module(&self) -> &[u8] {
        self.upgrader_wasm_module
            .as_deref()
            .expect("upgrader_wasm_module is not set")
    }

    pub fn get_upgrader_backup_snapshot_to_replace(&self) -> Option<Vec<u8>> {
        self.upgrader_backup_snapshots.get_snapshot_to_replace()
    }

    pub fn insert_upgrader_backup_snapshot(&mut self, snapshot_id: Vec<u8>) {
        self.upgrader_backup_snapshots.insert_snapshot(snapshot_id);
    }

    pub fn set_change_canister_request(&mut self, request: UUID) {
        self.change_canister_request = Some(request);
    }

    pub fn set_upgrader_canister_id(&mut self, canister_id: Principal) {
        self.upgrader_canister_id = Some(canister_id);
    }

    pub fn set_upgrader_wasm_module(&mut self, wasm_module: Vec<u8>) {
        self.upgrader_wasm_module = Some(wasm_module);
    }

    pub fn update_last_upgrade_timestamp(&mut self) {
        self.last_upgrade_timestamp = time();
    }

    pub fn clear_change_canister_request(&mut self) {
        self.change_canister_request = None;
    }

    pub fn set_disaster_recovery_committee(
        &mut self,
        committee: Option<DisasterRecoveryCommittee>,
    ) {
        self.disaster_recovery_committee = committee;
    }

    pub fn get_disaster_recovery_committee(&self) -> Option<&DisasterRecoveryCommittee> {
        self.disaster_recovery_committee.as_ref()
    }

    pub fn get_max_station_backup_snapshots(&self) -> u64 {
        self.max_station_backup_snapshots
    }

    pub async fn set_max_station_backup_snapshots(
        &mut self,
        max_backup_snapshots: u64,
    ) -> Result<(), String> {
        let upgrader_id = *self.get_upgrader_canister_id();
        call::<_, (Result<(), String>,)>(
            upgrader_id,
            "set_max_backup_snapshots",
            (max_backup_snapshots,),
        )
        .await
        .map_err(|(_, err)| err)?
        .0?;
        self.max_station_backup_snapshots = max_backup_snapshots;
        Ok(())
    }

    pub fn get_max_upgrader_backup_snapshots(&self) -> u64 {
        self.upgrader_backup_snapshots.get_max_backup_snapshots()
    }

    pub async fn set_max_upgrader_backup_snapshots(
        &mut self,
        max_backup_snapshots: u64,
    ) -> Result<(), String> {
        let upgrader_id = *self.get_upgrader_canister_id();
        self.upgrader_backup_snapshots
            .set_max_backup_snapshots(max_backup_snapshots, upgrader_id)
            .await
    }
}

impl SystemState {
    pub fn get(&self) -> &SystemInfo {
        match &self {
            SystemState::Uninitialized => trap("canister not initialized"),
            SystemState::Initialized(info) => info,
        }
    }

    pub fn is_initialized(&self) -> bool {
        matches!(self, SystemState::Initialized(_))
    }
}

/// Adds serialization and deserialization support of SystemState to stable memory.
impl Storable for SystemState {
    fn to_bytes(&self) -> Cow<[u8]> {
        match &self {
            SystemState::Uninitialized => Cow::Borrowed(&[]),
            SystemState::Initialized(info) => info.to_bytes(),
        }
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        if bytes.len() == 0 {
            return SystemState::Uninitialized;
        }
        SystemState::Initialized(SystemInfo::from_bytes(bytes))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: SYSTEM_RESERVED_MEMORY_BYTES,
        is_fixed_size: false,
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_info_name_validation() {
        let mut info = SystemInfo::default();
        assert_eq!(info.name, "Station");

        info.set_name("test".to_string());
        assert_eq!(info.name, "test");

        info.set_name("".to_string());
        assert_eq!(info.name, "Station");

        info.set_name("a".repeat(SystemInfo::MAX_NAME_LENGTH + 1));
        assert_eq!(info.name, "a".repeat(SystemInfo::MAX_NAME_LENGTH));

        info.set_name("  test  ".to_string());
        assert_eq!(info.name, "test");

        info.set_name("test  ".to_string());
        assert_eq!(info.name, "test");

        info.set_name("  test".to_string());
        assert_eq!(info.name, "test");
    }
}
