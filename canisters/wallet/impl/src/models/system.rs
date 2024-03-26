use crate::core::{
    ic_cdk::api::{time, trap},
    WASM_PAGE_SIZE,
};
use candid::Principal;
use ic_canister_core::types::{Timestamp, UUID};
use ic_canister_macros::storable;
use ic_stable_structures::{storable::Bound, Storable};
use std::borrow::Cow;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SystemState {
    Uninitialized, // This state is only used between wasm module instantiation and init().
    Initialized(SystemInfo),
}

// Reserves 40 pages of memory for the system state in stable memory.
pub const SYSTEM_STATE_WASM_PAGES: u64 = 40;
pub const SYSTEM_STATE_MEMORY_SIZE: u32 = WASM_PAGE_SIZE * SYSTEM_STATE_WASM_PAGES as u32;

#[storable(size = SYSTEM_STATE_MEMORY_SIZE)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SystemInfo {
    /// Last time the canister was upgraded or initialized.
    pub last_upgrade_timestamp: Timestamp,
    /// An optionally pending change canister proposal.
    pub change_canister_proposal: Option<UUID>,
    /// The upgrader canister id that is allowed to upgrade this canister.
    pub upgrader_canister_id: Principal,
    /// The upgrader canister wasm module.
    pub upgrader_wasm_module: Vec<u8>,
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self {
            last_upgrade_timestamp: time(),
            change_canister_proposal: None,
            upgrader_canister_id: Principal::management_canister(),
            upgrader_wasm_module: Vec::new(),
        }
    }
}

impl SystemInfo {
    /// The maximum size of each field in stable memory.
    pub const MAX_BYTE_SIZE_LAST_UPGRADE_TIMESTAMP: u32 = std::mem::size_of::<u64>() as u32;

    /// The maximum size of the system information in stable memory.
    pub const MAX_BYTE_SIZE: u32 = SYSTEM_STATE_MEMORY_SIZE;

    /// If this overflows then the stable memory layout will be broken.
    pub const SPARE_BYTES: u32 = Self::MAX_BYTE_SIZE - Self::MAX_BYTE_SIZE_LAST_UPGRADE_TIMESTAMP;
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
        max_size: SYSTEM_STATE_MEMORY_SIZE,
        is_fixed_size: false,
    };
}
