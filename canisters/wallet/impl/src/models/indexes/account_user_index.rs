use crate::models::{Account, AccountId, UserId};
use ic_canister_macros::storable;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct AccountUserIndex {
    /// The user id, which is a UUID.
    pub user_id: UserId,
    /// The account id, which is a UUID.
    pub account_id: AccountId,
}

#[derive(Clone, Debug)]
pub struct AccountUserIndexCriteria {
    pub user_id: UserId,
}

impl Account {
    pub fn to_index_by_users(&self) -> Vec<AccountUserIndex> {
        self.owners
            .iter()
            .map(|owner| AccountUserIndex {
                user_id: owner.to_owned(),
                account_id: self.id,
            })
            .collect::<Vec<AccountUserIndex>>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{AccountPolicies, Blockchain, BlockchainStandard, Metadata};

    #[test]
    fn test_account_to_user_association() {
        let account = Account {
            id: [0; 16],
            address: "0x1234".to_string(),
            balance: None,
            blockchain: Blockchain::InternetComputer,
            decimals: 0u32,
            policies: AccountPolicies {
                transfer_policy_id: None,
                edit_policy_id: None,
            },
            name: "Private".to_string(),
            owners: vec![[1; 16], [2; 16]],
            standard: BlockchainStandard::Native,
            last_modification_timestamp: 0,
            metadata: Metadata::default(),
            symbol: "ICP".to_string(),
        };

        let indexes = account.to_index_by_users();

        assert_eq!(indexes.len(), 2);
        assert_eq!(indexes[0].user_id, [1; 16]);
        assert_eq!(indexes[0].account_id, [0; 16]);
        assert_eq!(indexes[1].user_id, [2; 16]);
        assert_eq!(indexes[1].account_id, [0; 16]);
    }
}
