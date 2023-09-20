use crate::core::{ic::api::time, Timestamp, MAX_BYTE_SIZE_PRINCIPAL, MAX_BYTE_SIZE_UUID, UUID};
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::{BoundedStorable, Storable};
use std::borrow::Cow;

/// The key used to store an account identity in stable memory.
#[derive(CandidType, Deserialize, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AccountBankKey {
    pub account_id: UUID,
    pub canister_id: Principal,
}

impl Default for AccountBankKey {
    fn default() -> Self {
        AccountBank::key(&Principal::anonymous(), &UUID::default())
    }
}

#[derive(CandidType, Deserialize, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AccountBank {
    pub account_id: UUID,
    pub canister_id: Principal,
    pub name: Option<String>,
    pub last_update_timestamp: Timestamp,
}

impl Default for AccountBank {
    fn default() -> Self {
        Self {
            account_id: UUID::default(),
            name: None,
            canister_id: Principal::anonymous(),
            last_update_timestamp: time(),
        }
    }
}

impl AccountBankKey {
    /// The maximum size of each field in stable memory.
    pub const MAX_BYTE_SIZE_CANISTER_ID: u32 = AccountBank::MAX_BYTE_SIZE_CANISTER_ID;
    pub const MAX_BYTE_SIZE_ACCOUNT_ID: u32 = AccountBank::MAX_BYTE_SIZE_ACCOUNT_ID;

    /// The maximum size of an AccountIdentityKey in stable memory.
    pub const MAX_BYTE_SIZE: u32 = 256;

    /// The number of bytes that are not used by the account and could be used to add more fields to the account
    /// without breaking the stable memory layout, if this overflows then the stable memory layout will be broken.
    pub const SPARE_BYTES: u32 =
        Self::MAX_BYTE_SIZE - Self::MAX_BYTE_SIZE_CANISTER_ID - Self::MAX_BYTE_SIZE_ACCOUNT_ID;
}

impl AccountBank {
    /// The maximum size of each field in stable memory.
    pub const MAX_BYTE_SIZE_CANISTER_ID: u32 = MAX_BYTE_SIZE_PRINCIPAL;
    pub const MAX_BYTE_SIZE_ACCOUNT_ID: u32 = MAX_BYTE_SIZE_UUID;
    pub const MAX_BYTE_SIZE_NAME: u32 = 150;

    /// The maximum size of the Bank information in stable memory.
    pub const MAX_BYTE_SIZE: u32 = 512;

    /// The number of bytes that are not used by the account and could be used to add more fields to the account
    /// without breaking the stable memory layout, if this overflows then the stable memory layout will be broken.
    pub const SPARE_BYTES: u32 = Self::MAX_BYTE_SIZE
        - Self::MAX_BYTE_SIZE_CANISTER_ID
        - Self::MAX_BYTE_SIZE_NAME
        - Self::MAX_BYTE_SIZE_ACCOUNT_ID;

    pub fn new(canister_id: Principal, name: Option<String>, account_id: UUID) -> Self {
        Self {
            account_id,
            canister_id,
            name,
            last_update_timestamp: time(),
        }
    }

    pub fn key(canister_id: &Principal, account_id: &UUID) -> AccountBankKey {
        AccountBankKey {
            canister_id: *canister_id,
            account_id: *account_id,
        }
    }
}

/// Adds serialization and deserialization support to AccountIdentity to stable memory.
impl Storable for AccountBank {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

/// Represents the memory required to store a AccountIdentity in stable memory.
impl BoundedStorable for AccountBank {
    const MAX_SIZE: u32 = AccountBank::MAX_BYTE_SIZE;

    const IS_FIXED_SIZE: bool = false;
}

/// Adds serialization and deserialization support to AccountBankKey to stable memory.
impl Storable for AccountBankKey {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

/// Represents the memory required to store a AccountBankKey in stable memory.
impl BoundedStorable for AccountBankKey {
    const MAX_SIZE: u32 = AccountBankKey::MAX_BYTE_SIZE;

    const IS_FIXED_SIZE: bool = false;
}
