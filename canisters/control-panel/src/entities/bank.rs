use crate::core::MAX_BYTE_SIZE_PRINCIPAL;
use candid::{CandidType, Decode, Deserialize, Encode, Principal};
use ic_stable_structures::{BoundedStorable, Storable};
use std::borrow::Cow;

#[derive(CandidType, Deserialize, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Bank {
    id: Principal,
    name: String,
    is_main: bool,
}

impl Bank {
    pub fn new(id: Principal, name: String, is_main: bool) -> Self {
        Self { id, name, is_main }
    }
}

impl Bank {
    /// The maximum size of each field in stable memory.
    pub const MAX_BYTE_SIZE_ID: u32 = MAX_BYTE_SIZE_PRINCIPAL;
    pub const MAX_BYTE_SIZE_NAME: u32 = 150;
    pub const MAX_BYTE_SIZE_IS_MAIN: u32 = 1;

    /// The maximum size of the Bank information in stable memory.
    pub const MAX_BYTE_SIZE: u32 = 512;

    /// The number of bytes that are not used by the account and could be used to add more fields to the account
    /// without breaking the stable memory layout, if this overflows then the stable memory layout will be broken.
    pub const SPARE_BYTES: u32 = Self::MAX_BYTE_SIZE
        - Self::MAX_BYTE_SIZE_ID
        - Self::MAX_BYTE_SIZE_NAME
        - Self::MAX_BYTE_SIZE_IS_MAIN;
}

/// Adds serialization and deserialization support to AccountIdentity to stable memory.
impl Storable for Bank {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

/// Represents the memory required to store a AccountIdentity in stable memory.
impl BoundedStorable for Bank {
    const MAX_SIZE: u32 = Bank::MAX_BYTE_SIZE;

    const IS_FIXED_SIZE: bool = false;
}
