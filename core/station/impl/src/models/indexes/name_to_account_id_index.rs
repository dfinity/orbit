use crate::{
    core::utils::format_unique_string,
    models::{Account, AccountId},
};
use orbit_essentials::storable;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NameToAccountIdIndex {
    /// The name of the account.
    pub name: String,
    /// The account id, which is a UUID.
    pub account_id: AccountId,
}

#[derive(Clone, Debug)]
pub struct NameToAccountIdIndexCriteria {
    pub name: String,
}

impl Account {
    pub fn to_index_by_name(&self) -> NameToAccountIdIndex {
        NameToAccountIdIndex {
            name: format_unique_string(&self.name),
            account_id: self.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::account_test_utils::mock_account;

    #[test]
    fn test_name_to_account_id_association() {
        let mut account = mock_account();
        account.id = [0; 16];
        account.name = "testaccount".to_string();

        let index = account.to_index_by_name();

        assert_eq!(index.name, account.name);
        assert_eq!(index.account_id, account.id);
    }

    #[test]
    fn test_name_is_formatted() {
        let mut account = mock_account();
        account.id = [0; 16];
        account.name = "Test Account".to_string();

        let index = account.to_index_by_name();

        assert_eq!(index.name, "testaccount");
    }
}
