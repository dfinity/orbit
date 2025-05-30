use super::CANISTER_CONFIG_STATE_SIZE;
use crate::core::ic_cdk::api::time;
use crate::models::RateLimiter;
use crate::SYSTEM_VERSION;
use ic_stable_structures::{storable::Bound, Storable};
use orbit_essentials::storable;
use orbit_essentials::types::{Timestamp, WasmModuleExtraChunks};
use std::borrow::Cow;

#[storable]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanisterConfig {
    /// The upgrader canister wasm module that will be used to upgrade the station canister.
    pub upgrader_wasm_module: Vec<u8>,

    /// The station canister wasm module that will be used to deploy new stations.
    pub station_wasm_module: Vec<u8>,

    /// Optional extra chunks of the station canister wasm module.
    pub station_wasm_module_extra_chunks: Option<WasmModuleExtraChunks>,

    /// Last time the canister was upgraded or initialized.
    pub last_upgrade_timestamp: Timestamp,

    /// The version of the canister.
    pub version: Option<String>,

    /// Used to rate limit the number of deployed stations per day by the control panel.
    #[serde(default = "RateLimiter::new_global")]
    pub global_rate_limiter: RateLimiter,
}

impl Default for CanisterConfig {
    fn default() -> Self {
        Self {
            upgrader_wasm_module: vec![],
            station_wasm_module: vec![],
            station_wasm_module_extra_chunks: None,
            last_upgrade_timestamp: time(),
            version: Some(SYSTEM_VERSION.to_string()),
            global_rate_limiter: RateLimiter::new_global(),
        }
    }
}

impl CanisterConfig {
    pub fn new(
        upgrader_wasm_module: Vec<u8>,
        station_wasm_module: Vec<u8>,
        station_wasm_module_extra_chunks: Option<WasmModuleExtraChunks>,
    ) -> Self {
        Self {
            upgrader_wasm_module,
            station_wasm_module,
            station_wasm_module_extra_chunks,
            last_upgrade_timestamp: time(),
            version: Some(SYSTEM_VERSION.to_string()),
            global_rate_limiter: RateLimiter::new_global(),
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
        if bytes.is_empty() {
            return CanisterState::Uninitialized;
        }
        CanisterState::Initialized(CanisterConfig::from_bytes(bytes))
    }

    const BOUND: Bound = Bound::Bounded {
        max_size: CANISTER_CONFIG_STATE_SIZE,
        is_fixed_size: false,
    };
}
