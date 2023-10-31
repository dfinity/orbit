use crate::models::{Account, AccountId};
use candid::{CandidType, Deserialize, Principal};
use ic_canister_macros::stable_object;

/// Represents an account identity index within the system.
#[stable_object]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::AccountIdentity;
    use ic_stable_structures::Storable;

    #[test]
    fn valid_model_serialization() {
        let model = AccountIdentityIndex {
            identity_id: Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").unwrap(),
            account_id: [u8::MAX; 16],
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = AccountIdentityIndex::from_bytes(serialized_model);

        assert_eq!(model.identity_id, deserialized_model.identity_id);
        assert_eq!(model.account_id, deserialized_model.account_id);
    }

    #[test]
    fn valid_account_identities_to_indexes() {
        let account = Account {
            id: [u8::MAX; 16],
            identities: vec![
                AccountIdentity {
                    identity: Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").unwrap(),
                    name: None,
                },
                AccountIdentity {
                    identity: Principal::anonymous(),
                    name: None,
                },
            ],
            unconfirmed_identities: vec![],
            banks: vec![],
            main_bank: None,
            last_update_timestamp: 0,
            name: None,
        };

        let indexes = account.to_index_for_identities();

        assert_eq!(indexes.len(), 2);
        assert_eq!(
            indexes[0].identity_id,
            Principal::from_text("2chl6-4hpzw-vqaaa-aaaaa-c").unwrap()
        );
        assert_eq!(indexes[0].account_id, [u8::MAX; 16]);
        assert_eq!(indexes[1].identity_id, Principal::anonymous());
        assert_eq!(indexes[1].account_id, [u8::MAX; 16]);
    }
}
