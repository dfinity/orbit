use crate::core::ic::api::time;
use crate::core::{Timestamp, MAX_BYTE_SIZE_PRINCIPAL, MAX_BYTE_SIZE_UUID, UUID};
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_canister_macros::stable_object;
use uuid::Uuid;

/// The key used to store an account identity in stable memory.
#[stable_object(size = AccountKey::MAX_BYTE_SIZE)]
#[derive(CandidType, Deserialize, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AccountKey {
    /// The UUID that identifies the account.
    pub id: UUID,
}

impl Default for AccountKey {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().as_bytes().to_owned(),
        }
    }
}

/// The identity of an account.
#[stable_object(size = Account::MAX_BYTE_SIZE)]
#[derive(CandidType, Deserialize, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Account {
    /// The UUID that identifies the account.
    pub id: UUID,
    /// The name of the account (if any).
    pub name: Option<String>,
    /// The shared bank to use for the account.
    pub main_bank: Option<Principal>,
    /// The status of the identity.
    pub banks: Vec<Principal>,
    /// The identifies associated with the account.
    pub identities: Vec<Principal>,
    /// The unconfirmed identifies associated with the account.
    pub unconfirmed_identities: Vec<Principal>,
    /// Last time the identity was updated.
    pub last_update_timestamp: Timestamp,
}

impl Default for Account {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().as_bytes().to_owned(),
            name: None,
            main_bank: None,
            banks: Vec::new(),
            identities: Vec::new(),
            unconfirmed_identities: Vec::new(),
            last_update_timestamp: time(),
        }
    }
}

impl Account {
    /// The maximum number of identities that can be associated with an account,
    /// this is limited to have a fixed size for the account in stable memory.
    pub const MAX_ACCOUNT_IDENTITIES: u32 = 10;

    /// The maximum number of unconfirmed identities at any given time with an account.
    pub const MAX_ACCOUNT_UNCONFIRMED_IDENTITIES: u32 = 5;

    /// The maximum number of banks that can be associated with an account,
    /// this is limited to have a fixed size for the account in stable memory.
    pub const MAX_ACCOUNT_BANKS: u32 = 10;

    /// The maximum size of each field in stable memory.
    pub const MAX_BYTE_SIZE_ID: u32 = MAX_BYTE_SIZE_UUID;
    pub const MAX_BYTE_SIZE_NAME: u32 = 150;
    pub const MAX_BYTE_SIZE_MAIN_BANK: u32 = MAX_BYTE_SIZE_PRINCIPAL;
    pub const MAX_BYTE_SIZE_BANKS: u32 = MAX_BYTE_SIZE_PRINCIPAL * Self::MAX_ACCOUNT_BANKS;
    pub const MAX_BYTE_SIZE_IDENTITIES: u32 =
        MAX_BYTE_SIZE_PRINCIPAL * Self::MAX_ACCOUNT_IDENTITIES;
    pub const MAX_BYTE_SIZE_UNCONFIRMED_IDENTITIES: u32 =
        MAX_BYTE_SIZE_PRINCIPAL * Self::MAX_ACCOUNT_UNCONFIRMED_IDENTITIES;
    pub const MAX_BYTE_SIZE_LAST_UPDATE_TIMESTAMP: u32 = std::mem::size_of::<u64>() as u32;

    /// The maximum size of an AccountIdentity in stable memory.
    pub const MAX_BYTE_SIZE: u32 = 4096;

    /// The number of bytes that are not used by the account and could be used to add more fields to the account
    /// without breaking the stable memory layout, if this overflows then the stable memory layout will be broken.
    pub const SPARE_BYTES: u32 = Self::MAX_BYTE_SIZE
        - Self::MAX_BYTE_SIZE_ID
        - Self::MAX_BYTE_SIZE_NAME
        - Self::MAX_BYTE_SIZE_MAIN_BANK
        - Self::MAX_BYTE_SIZE_BANKS
        - Self::MAX_BYTE_SIZE_IDENTITIES
        - Self::MAX_BYTE_SIZE_UNCONFIRMED_IDENTITIES
        - Self::MAX_BYTE_SIZE_LAST_UPDATE_TIMESTAMP;

    pub fn key(account_id: &UUID) -> AccountKey {
        AccountKey { id: *account_id }
    }
}

impl AccountKey {
    /// The maximum size of each field in stable memory.
    pub const MAX_BYTE_SIZE_ID: u32 = MAX_BYTE_SIZE_UUID;

    /// The maximum size of an AccountKey in stable memory.
    pub const MAX_BYTE_SIZE: u32 = 256;

    /// The number of bytes that are not used by the account and could be used to add more fields to the account
    /// without breaking the stable memory layout, if this overflows then the stable memory layout will be broken.
    pub const SPARE_BYTES: u32 = Self::MAX_BYTE_SIZE - Self::MAX_BYTE_SIZE_ID;
}
