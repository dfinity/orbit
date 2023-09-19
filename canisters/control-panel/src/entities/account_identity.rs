use super::AccountIdentityStatus;
use crate::core::{Timestamp, MAX_BYTE_SIZE_PRINCIPAL};
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_cdk::api::time;
use ic_stable_structures::{BoundedStorable, Storable};
use std::borrow::Cow;

/// The key used to store an account identity in stable memory.
#[derive(CandidType, Deserialize, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AccountIdentityKey {
    /// The principal ID of the identity.
    pub id: Principal,
}

/// The identity of an account.
#[derive(CandidType, Deserialize, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AccountIdentity {
    /// The principal ID of the identity.
    pub id: Principal,
    /// The name of the identity (if any).
    pub name: Option<String>,
    /// The status of the identity.
    pub status: AccountIdentityStatus,
    /// Last time the identity was updated.
    pub last_update_timestamp: Timestamp,
}

impl Default for AccountIdentity {
    fn default() -> Self {
        Self {
            id: Principal::anonymous(),
            name: None,
            status: AccountIdentityStatus::PendingActivation,
            last_update_timestamp: time(),
        }
    }
}

impl AccountIdentity {
    /// The maximum size of each field in stable memory.
    pub const MAX_BYTE_SIZE_ID: u32 = MAX_BYTE_SIZE_PRINCIPAL;
    pub const MAX_BYTE_SIZE_NAME: u32 = 100;
    pub const MAX_BYTE_SIZE_STATUS: u32 = AccountIdentityStatus::MAX_BYTE_SIZE;
    pub const MAX_BYTE_SIZE_LAST_UPDATE_TIMESTAMP: u32 = std::mem::size_of::<u64>() as u32;

    /// The maximum size of an AccountIdentity in stable memory.
    pub const MAX_BYTE_SIZE: u32 = 1024;

    /// The number of bytes that are not used by the account and could be used to add more fields to the account
    /// without breaking the stable memory layout, if this overflows then the stable memory layout will be broken.
    pub const SPARE_BYTES: u32 = Self::MAX_BYTE_SIZE
        - Self::MAX_BYTE_SIZE_ID
        - Self::MAX_BYTE_SIZE_NAME
        - Self::MAX_BYTE_SIZE_STATUS;
}

impl AccountIdentityKey {
    /// The maximum size of each field in stable memory.
    pub const MAX_BYTE_SIZE_ID: u32 = AccountIdentity::MAX_BYTE_SIZE_ID;

    /// The maximum size of an AccountIdentityKey in stable memory.
    pub const MAX_BYTE_SIZE: u32 = 256;

    /// The number of bytes that are not used by the account and could be used to add more fields to the account
    /// without breaking the stable memory layout, if this overflows then the stable memory layout will be broken.
    pub const SPARE_BYTES: u32 = Self::MAX_BYTE_SIZE - Self::MAX_BYTE_SIZE_ID;
}

/// Adds serialization and deserialization support to AccountIdentity to stable memory.
impl Storable for AccountIdentity {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

/// Represents the memory required to store a AccountIdentity in stable memory.
impl BoundedStorable for AccountIdentity {
    const MAX_SIZE: u32 = AccountIdentity::MAX_BYTE_SIZE;

    const IS_FIXED_SIZE: bool = false;
}

/// Adds serialization and deserialization support to AccountIdentityKey to stable memory.
impl Storable for AccountIdentityKey {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

/// Represents the memory required to store a AccountIdentityKey in stable memory.
impl BoundedStorable for AccountIdentityKey {
    const MAX_SIZE: u32 = AccountIdentityKey::MAX_BYTE_SIZE;

    const IS_FIXED_SIZE: bool = false;
}
