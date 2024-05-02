use super::CANISTER_CONFIG_STATE_SIZE;
use crate::core::ic_cdk::api::time;
use ic_stable_structures::{storable::Bound, Storable};
use orbit_essentials::storable;
use orbit_essentials::types::Timestamp;
use std::borrow::Cow;

#[storable]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanisterConfig {
    /// The upgrader canister wasm module that will be used to upgrade the station canister.
    pub upgrader_wasm_module: Vec<u8>,

    /// The station canister wasm module that will be used to deploy new stations.
    pub station_wasm_module: Vec<u8>,

    /// Last time the canister was upgraded or initialized.
    pub last_upgrade_timestamp: Timestamp,
}

impl Default for CanisterConfig {
    fn default() -> Self {
        Self {
            upgrader_wasm_module: vec![],
            station_wasm_module: vec![],
            last_upgrade_timestamp: time(),
        }
    }
}

impl CanisterConfig {
    pub fn new(upgrader_wasm_module: Vec<u8>, station_wasm_module: Vec<u8>) -> Self {
        Self {
            upgrader_wasm_module,
            station_wasm_module,
            last_upgrade_timestamp: time(),
        }
    }
}

/// Configuration state of the canister.
pub enum CanisterState {
    Uninitialized, // This state is only used between wasm module initialization and the first call to `upload_canister_modules`.
    Initialized(CanisterConfig),
}

impl CanisterState {
    pub fn get(&self) -> Option<&CanisterConfig> {
        match &self {
            CanisterState::Uninitialized => None,
            CanisterState::Initialized(config) => Some(config),
        }
    }
}

/// Adds serialization and deserialization support of CanisterState to stable memory.
impl Storable for CanisterState {
    fn to_bytes(&self) -> Cow<[u8]> {
        match &self {
            CanisterState::Uninitialized => Cow::Borrowed(&[]),
            CanisterState::Initialized(config) => config.to_bytes(),
        }
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        if bytes.len() == 0 {
            return CanisterState::Uninitialized;
        }
        CanisterState::Initialized(CanisterConfig::from_bytes(bytes))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: CANISTER_CONFIG_STATE_SIZE,
        is_fixed_size: false,
    };
}
