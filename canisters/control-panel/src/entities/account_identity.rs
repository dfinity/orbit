use crate::core::ic::api::time;
use crate::core::{Timestamp, MAX_BYTE_SIZE_PRINCIPAL, MAX_BYTE_SIZE_UUID, UUID};
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_canister_macros::stable_object;
use ic_stable_structures::{BoundedStorable, Storable};

/// The key used to store an account identity in stable memory.
#[stable_object(size = AccountIdentityKey::MAX_BYTE_SIZE)]
#[derive(CandidType, Deserialize, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AccountIdentityKey {
    pub identity: Principal,
    pub account_id: UUID,
}

impl Default for AccountIdentityKey {
    fn default() -> Self {
        AccountIdentity::key(&Principal::anonymous(), &UUID::default())
    }
}

/// The identity of an account.
#[stable_object(size = AccountIdentity::MAX_BYTE_SIZE)]
#[derive(CandidType, Deserialize, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AccountIdentity {
    /// The principal ID of the identity.
    pub identity: Principal,
    /// The account_id of the identity.
    pub account_id: UUID,
    /// The name of the identity (if any).
    pub name: Option<String>,
    /// Last time the identity was updated.
    pub last_update_timestamp: Timestamp,
}

impl Default for AccountIdentity {
    fn default() -> Self {
        Self {
            identity: Principal::anonymous(),
            account_id: UUID::default(),
            name: None,
            last_update_timestamp: time(),
        }
    }
}

impl AccountIdentity {
    /// The maximum size of each field in stable memory.
    pub const MAX_BYTE_SIZE_IDENTITY: u32 = MAX_BYTE_SIZE_PRINCIPAL;
    pub const MAX_BYTE_SIZE_NAME: u32 = 100;
    pub const MAX_BYTE_SIZE_ACCOUNT_ID: u32 = MAX_BYTE_SIZE_UUID;
    pub const MAX_BYTE_SIZE_LAST_UPDATE_TIMESTAMP: u32 = std::mem::size_of::<u64>() as u32;

    /// The maximum size of an AccountIdentity in stable memory.
    pub const MAX_BYTE_SIZE: u32 = 1024;

    /// The number of bytes that are not used by the account and could be used to add more fields to the account
    /// without breaking the stable memory layout, if this overflows then the stable memory layout will be broken.
    pub const SPARE_BYTES: u32 = Self::MAX_BYTE_SIZE
        - Self::MAX_BYTE_SIZE_IDENTITY
        - Self::MAX_BYTE_SIZE_NAME
        - Self::MAX_BYTE_SIZE_ACCOUNT_ID;

    pub fn key(identity: &Principal, account_id: &UUID) -> AccountIdentityKey {
        AccountIdentityKey {
            identity: *identity,
            account_id: *account_id,
        }
    }
}

impl AccountIdentityKey {
    /// The maximum size of each field in stable memory.
    pub const MAX_BYTE_SIZE_IDENTITY: u32 = AccountIdentity::MAX_BYTE_SIZE_IDENTITY;
    pub const MAX_BYTE_SIZE_ACCOUNT_ID: u32 = AccountIdentity::MAX_BYTE_SIZE_ACCOUNT_ID;

    /// The maximum size of an AccountIdentityKey in stable memory.
    pub const MAX_BYTE_SIZE: u32 = 256;

    /// The number of bytes that are not used by the account and could be used to add more fields to the account
    /// without breaking the stable memory layout, if this overflows then the stable memory layout will be broken.
    pub const SPARE_BYTES: u32 =
        Self::MAX_BYTE_SIZE - Self::MAX_BYTE_SIZE_IDENTITY - Self::MAX_BYTE_SIZE_ACCOUNT_ID;
}
