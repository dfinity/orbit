use super::CANISTER_CONFIG_STATE_SIZE;
use crate::core::ic_cdk::api::{time, trap};
use ic_canister_core::types::Timestamp;
use ic_canister_macros::storable;
use ic_stable_structures::{storable::Bound, Storable};
use std::borrow::Cow;

#[storable]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanisterConfig {
    /// The upgrader canister wasm module that will be used to upgrade the wallet canister.
    pub upgrader_wasm_module: Vec<u8>,

    /// The wallet canister wasm module that will be used to deploy new wallets.
    pub wallet_wasm_module: Vec<u8>,

    /// Last time the canister was upgraded or initialized.
    pub last_upgrade_timestamp: Timestamp,
}

impl Default for CanisterConfig {
    fn default() -> Self {
        Self {
            upgrader_wasm_module: vec![],
            wallet_wasm_module: vec![],
            last_upgrade_timestamp: time(),
        }
    }
}

impl CanisterConfig {
    pub fn new(upgrader_wasm_module: Vec<u8>, wallet_wasm_module: Vec<u8>) -> Self {
        Self {
            upgrader_wasm_module,
            wallet_wasm_module,
            last_upgrade_timestamp: time(),
        }
    }
}

/// Configuration state of the canister.
pub enum CanisterState {
    Uninitialized, // This state is only used between wasm module initialization and init().
    Initialized(CanisterConfig),
}

impl CanisterState {
    pub fn get(&self) -> &CanisterConfig {
        match &self {
            CanisterState::Uninitialized => trap("canister config not initialized"),
            CanisterState::Initialized(config) => config,
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
