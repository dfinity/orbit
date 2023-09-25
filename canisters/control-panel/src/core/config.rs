use super::{Timestamp, MAX_BYTE_SIZE_PRINCIPAL, WASM_PAGE_SIZE};
use crate::core::ic::api::time;
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_cdk::trap;
use ic_stable_structures::{BoundedStorable, Storable};
use std::borrow::Cow;

#[derive(CandidType, Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct CanisterConfig {
    /// This is the shared bank canister that is user by default for all
    /// accounts that don't have a private bank canister.
    pub shared_bank_canister: Principal,

    /// Last time the canister was upgraded or initialized.
    pub last_upgrade_timestamp: Timestamp,
}

impl Default for CanisterConfig {
    fn default() -> Self {
        Self {
            shared_bank_canister: Principal::anonymous(),
            last_upgrade_timestamp: time(),
        }
    }
}

impl CanisterConfig {
    pub fn new(shared_bank_canister: Principal, last_upgrade_timestamp: Timestamp) -> Self {
        Self {
            shared_bank_canister,
            last_upgrade_timestamp,
        }
    }
}

impl CanisterConfig {
    /// The maximum size of each field in stable memory.
    pub const MAX_BYTE_SIZE_SHARED_BANK_CANISTER: u32 = MAX_BYTE_SIZE_PRINCIPAL;
    pub const MAX_BYTE_SIZE_LAST_UPGRADE_TIMESTAMP: u32 = std::mem::size_of::<u64>() as u32;

    /// The maximum size of the CanisterConfig in stable memory.
    pub const MAX_BYTE_SIZE: u32 = WASM_PAGE_SIZE;

    /// The number of bytes that are not used by the account and could be used to add more fields to the account
    /// without breaking the stable memory layout, if this overflows then the stable memory layout will be broken.
    pub const SPARE_BYTES: u32 = Self::MAX_BYTE_SIZE
        - Self::MAX_BYTE_SIZE_SHARED_BANK_CANISTER
        - Self::MAX_BYTE_SIZE_LAST_UPGRADE_TIMESTAMP;
}

/// Adds serialization and deserialization support of CanisterConfig to stable memory.
impl Storable for CanisterConfig {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

/// Represents the memory required to store a CanisterConfig in stable memory.
impl BoundedStorable for CanisterConfig {
    const MAX_SIZE: u32 = CanisterConfig::MAX_BYTE_SIZE;

    const IS_FIXED_SIZE: bool = false;
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
