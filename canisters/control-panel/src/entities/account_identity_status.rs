use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::{BoundedStorable, Storable};
use std::{borrow::Cow, fmt::Display};

/// Represents the possible states of an account identity.
#[derive(CandidType, Deserialize, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum AccountIdentityStatus {
    /// The identity is active within the account.
    Active,
    /// The identity is pending activation, and is waiting for the user with access to the identity to activate it.
    PendingActivation,
}

impl Display for AccountIdentityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AccountIdentityStatus::Active => write!(f, "active"),
            AccountIdentityStatus::PendingActivation => write!(f, "pending-activation"),
        }
    }
}

impl AccountIdentityStatus {
    /// The maximum size of an AccountIdentityStatus in stable memory.
    pub const MAX_BYTE_SIZE: u32 = 50;
}

/// Adds serialization and deserialization support to AccountIdentityStatus to stable memory.
impl Storable for AccountIdentityStatus {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

/// Represents the memory required to store a AccountIdentityStatus in stable memory.
impl BoundedStorable for AccountIdentityStatus {
    const MAX_SIZE: u32 = AccountIdentityStatus::MAX_BYTE_SIZE;

    const IS_FIXED_SIZE: bool = false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_types_match_string_representation() {
        assert_eq!(AccountIdentityStatus::Active.to_string(), "active");
        assert_eq!(
            AccountIdentityStatus::PendingActivation.to_string(),
            "pending-activation"
        );
    }
}
