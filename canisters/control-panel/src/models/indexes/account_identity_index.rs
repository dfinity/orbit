use crate::models::{Account, AccountId};
use candid::{CandidType, Deserialize, Principal};
use ic_canister_macros::stable_object;

/// Represents an account identity index within the system.
#[stable_object(size = 128)]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccountIdentityIndex {
    /// The identity associated with the account.
    pub identity_id: Principal,
    /// The account id, which is a UUID.
    pub account_id: AccountId,
}

#[derive(Clone, Debug)]
pub struct AccountIdentityIndexCriteria {
    pub identity_id: Principal,
}

impl Account {
    pub fn to_index_for_identities(&self) -> Vec<AccountIdentityIndex> {
        self.identities
            .iter()
            .map(|identity| AccountIdentityIndex {
                identity_id: identity.identity.to_owned(),
                account_id: self.id,
            })
            .collect::<Vec<AccountIdentityIndex>>()
    }
}
