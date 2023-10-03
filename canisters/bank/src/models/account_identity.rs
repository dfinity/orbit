use super::AccountId;
use candid::{CandidType, Deserialize, Principal};
use ic_canister_core::types::Timestamp;
use ic_canister_macros::stable_object;

/// Represents an account identity within the system.
#[stable_object(size = 256)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccountIdentity {
    /// The account id, which is a UUID.
    pub account_id: AccountId,
    /// The identity associated with the account.
    pub identity_id: Principal,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
}

#[stable_object(size = 64)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccountIdentityKey {
    /// The identity id, which is a UUID.
    pub identity_id: Principal,
    /// The account id, which is a UUID.
    pub account_id: AccountId,
}

impl AccountIdentity {
    /// Creates a new account identity key from the given key components.
    pub fn key(identity_id: &Principal, account_id: &AccountId) -> AccountIdentityKey {
        AccountIdentityKey {
            identity_id: *identity_id,
            account_id: *account_id,
        }
    }

    pub fn as_key(&self) -> AccountIdentityKey {
        AccountIdentity::key(&self.identity_id, &self.account_id)
    }
}
