use super::WASM_PAGE_SIZE;
use candid::{CandidType, Decode, Deserialize, Encode};
use ic_canister_core::{
    cdk::api::{time, trap},
    types::Timestamp,
};
use ic_canister_macros::stable_object;
use ic_stable_structures::Storable;
use std::borrow::Cow;

#[stable_object(size = WASM_PAGE_SIZE)]
#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CanisterConfig {
    /// Last time the canister was upgraded or initialized.
    pub last_upgrade_timestamp: Timestamp,
    /// The threshold of approvals required for operations to be executed.
    pub approval_threshold: u8,
}

impl Default for CanisterConfig {
    fn default() -> Self {
        Self {
            last_upgrade_timestamp: time(),
            /// By default, the bank canister requires 100% of the votes to approve operations.
            approval_threshold: 100u8,
        }
    }
}

impl CanisterConfig {
    pub fn new(approval_threshold: u8, last_upgrade_timestamp: Timestamp) -> Self {
        Self {
            last_upgrade_timestamp,
            approval_threshold,
        }
    }
}

impl CanisterConfig {
    /// The maximum size of each field in stable memory.
    pub const MAX_BYTE_SIZE_LAST_UPGRADE_TIMESTAMP: u32 = std::mem::size_of::<u64>() as u32;
    pub const MAX_BYTE_SIZE_APPROVAL_THRESHOLD: u32 = std::mem::size_of::<u8>() as u32;

    /// The maximum size of the CanisterConfig in stable memory.
    pub const MAX_BYTE_SIZE: u32 = WASM_PAGE_SIZE;

    /// The number of bytes that are not used by the account and could be used to add more fields to the account
    /// without breaking the stable memory layout, if this overflows then the stable memory layout will be broken.
    pub const SPARE_BYTES: u32 = Self::MAX_BYTE_SIZE
        - Self::MAX_BYTE_SIZE_LAST_UPGRADE_TIMESTAMP
        - Self::MAX_BYTE_SIZE_APPROVAL_THRESHOLD;
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
}
