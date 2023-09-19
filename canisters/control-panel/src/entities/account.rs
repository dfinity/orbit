use crate::core::{MAX_PRINCIPAL_BYTE_SIZE, MAX_UUID_BYTE_SIZE, UUID};
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::{BoundedStorable, Storable};
use std::borrow::Cow;
use uuid::Uuid;

/// The identity of an account.
#[derive(CandidType, Deserialize, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Account {
    /// The UUID that identifies the account.
    id: UUID,
    /// The name of the account (if any).
    name: Option<String>,
    /// The status of the identity.
    main_bank: Option<Principal>,
    /// The identifies associated with the account.
    identities: Vec<Principal>,
}

impl Default for Account {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().as_bytes().to_vec(),
            name: None,
            main_bank: None,
            identities: Vec::new(),
        }
    }
}

impl Account {
    /// The maximum number of identities that can be associated with an account,
    /// this is limited to have a fixed size for the account in stable memory.
    pub const MAX_ACCOUNT_IDENTITIES: u32 = 10;

    /// The maximum size of each field in stable memory.
    pub const MAX_BYTE_SIZE_ID: u32 = MAX_UUID_BYTE_SIZE;
    pub const MAX_BYTE_SIZE_NAME: u32 = 150;
    pub const MAX_BYTE_SIZE_MAIN_BANK: u32 = MAX_PRINCIPAL_BYTE_SIZE;
    pub const MAX_BYTE_SIZE_MAIN_IDENTITIES: u32 =
        MAX_PRINCIPAL_BYTE_SIZE * Self::MAX_ACCOUNT_IDENTITIES;

    /// The maximum size of an AccountIdentity in stable memory.
    pub const MAX_BYTE_SIZE: u32 = 4096;

    /// The number of bytes that are not used by the account and could be used to add more fields to the account
    /// without breaking the stable memory layout, if this overflows then the stable memory layout will be broken.
    pub const SPARE_BYTES: u32 = Self::MAX_BYTE_SIZE
        - Self::MAX_BYTE_SIZE_ID
        - Self::MAX_BYTE_SIZE_NAME
        - Self::MAX_BYTE_SIZE_MAIN_BANK
        - Self::MAX_BYTE_SIZE_MAIN_IDENTITIES;
}

/// Adds serialization and deserialization support to AccountIdentity to stable memory.
impl Storable for Account {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

/// Represents the memory required to store a AccountIdentity in stable memory.
impl BoundedStorable for Account {
    const MAX_SIZE: u32 = Account::MAX_BYTE_SIZE;

    const IS_FIXED_SIZE: bool = false;
}
