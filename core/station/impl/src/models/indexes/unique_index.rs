use crate::{
    core::utils::format_unique_string,
    models::{Account, AddressBookEntry, Asset, ExternalCanister, User, UserGroup},
};
use candid::Principal;
use orbit_essentials::{storable, types::UUID};

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum UniqueIndexKey {
    AccountName(String),
    AddressBookBlockchainAddress(
        String, // Blockchain
        String, // Address
    ),
    ExternalCanisterId(Principal),
    ExternalCanisterName(String),
    UserGroupName(String),
    UserIdentity(Principal),
    UserName(String),
    AssetSymbolBlockchain(
        String, // Blockchain
        String, // Symbol
    ),
}

impl AddressBookEntry {
    /// Converts the address book entry to it's unique index by address.
    fn to_unique_index_by_address(&self) -> (UniqueIndexKey, UUID) {
        (
            UniqueIndexKey::AddressBookBlockchainAddress(
                self.blockchain.to_string().to_lowercase(),
                self.address.to_string(),
            ),
            self.id,
        )
    }

    /// Extracts all unique indexes for the address book entry.
    pub fn to_unique_indexes(&self) -> Vec<(UniqueIndexKey, UUID)> {
        vec![self.to_unique_index_by_address()]
    }
}

impl User {
    /// Converts the user to it's unique index by name.
    fn to_unique_index_by_name(&self) -> (UniqueIndexKey, UUID) {
        (
            UniqueIndexKey::UserName(format_unique_string(&self.name)),
            self.id,
        )
    }

    /// Converts the user to it's unique index by the user identities.
    fn to_unique_index_by_identity(&self) -> Vec<(UniqueIndexKey, UUID)> {
        self.identities
            .iter()
            .map(|identity| (UniqueIndexKey::UserIdentity(*identity), self.id))
            .collect()
    }

    /// Extracts all unique indexes for the user.
    pub fn to_unique_indexes(&self) -> Vec<(UniqueIndexKey, UUID)> {
        let mut uniq_indexes = Vec::new();
        uniq_indexes.push(self.to_unique_index_by_name());
        uniq_indexes.extend(self.to_unique_index_by_identity());

        uniq_indexes
    }
}

impl UserGroup {
    /// Converts the user group to it's unique index by name.
    fn to_unique_index_by_name(&self) -> (UniqueIndexKey, UUID) {
        (
            UniqueIndexKey::UserGroupName(format_unique_string(&self.name)),
            self.id,
        )
    }

    /// Extracts all unique indexes for the user group.
    pub fn to_unique_indexes(&self) -> Vec<(UniqueIndexKey, UUID)> {
        vec![self.to_unique_index_by_name()]
    }
}

impl Asset {
    /// Converts the asset to it's unique index by name.
    fn to_unique_index(&self) -> (UniqueIndexKey, UUID) {
        (
            Self::to_unique_index_by_symbol_blockchain(&self.symbol, self.blockchain.to_string()),
            self.id,
        )
    }

    pub fn to_unique_index_by_symbol_blockchain(
        symbol: &str,
        blockchain: String,
    ) -> UniqueIndexKey {
        UniqueIndexKey::AssetSymbolBlockchain(symbol.to_uppercase(), blockchain.to_string())
    }

    /// Extracts all unique indexes for the asset.
    pub fn to_unique_indexes(&self) -> Vec<(UniqueIndexKey, UUID)> {
        vec![self.to_unique_index()]
    }
}

impl ExternalCanister {
    /// Converts the external canister to it's unique index by name.
    fn to_unique_index_by_name(&self) -> (UniqueIndexKey, UUID) {
        (
            UniqueIndexKey::ExternalCanisterName(format_unique_string(&self.name)),
            self.id,
        )
    }

    /// Converts the external canister to it's unique index by canister id.
    fn to_unique_index_by_canister_id(&self) -> (UniqueIndexKey, UUID) {
        (
            UniqueIndexKey::ExternalCanisterId(self.canister_id),
            self.id,
        )
    }

    /// Extracts all unique indexes for the external canister.
    pub fn to_unique_indexes(&self) -> Vec<(UniqueIndexKey, UUID)> {
        vec![
            self.to_unique_index_by_name(),
            self.to_unique_index_by_canister_id(),
        ]
    }
}

impl Account {
    /// Converts the account to it's unique index by name.
    fn to_unique_index_by_name(&self) -> (UniqueIndexKey, UUID) {
        (
            UniqueIndexKey::AccountName(format_unique_string(&self.name)),
            self.id,
        )
    }

    /// Extracts all unique indexes for the account.
    pub fn to_unique_indexes(&self) -> Vec<(UniqueIndexKey, UUID)> {
        vec![self.to_unique_index_by_name()]
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::models::{
        account_test_utils::mock_account, address_book_entry_test_utils::mock_address_book_entry,
        external_canister_test_utils::mock_external_canister,
        user_group_test_utils::mock_user_group, user_test_utils::mock_user, Blockchain,
    };

    #[test]
    fn test_user_unique_indexes() {
        let mut user = mock_user();
        user.name = "Test".to_string();
        user.identities = vec![
            Principal::from_slice(&[1; 29]),
            Principal::from_slice(&[2; 29]),
        ];

        let indexes = user.to_unique_indexes();

        assert_eq!(indexes.len(), 3);
        assert_eq!(
            indexes[0].0,
            UniqueIndexKey::UserName(format_unique_string("Test"))
        );
    }

    #[test]
    fn test_user_group_unique_indexes() {
        let mut user_group = mock_user_group();
        user_group.name = "Test".to_string();

        let indexes = user_group.to_unique_indexes();

        assert_eq!(indexes.len(), 1);
        assert_eq!(
            indexes[0].0,
            UniqueIndexKey::UserGroupName(format_unique_string("Test"))
        );
    }

    #[test]
    fn test_external_canister_unique_indexes() {
        let mut external_canister = mock_external_canister();
        external_canister.name = "Test".to_string();
        external_canister.canister_id = Principal::anonymous();

        let indexes = external_canister.to_unique_indexes();

        assert_eq!(indexes.len(), 2);
        assert_eq!(
            indexes[0].0,
            UniqueIndexKey::ExternalCanisterName(format_unique_string("Test"))
        );
        assert_eq!(
            indexes[1].0,
            UniqueIndexKey::ExternalCanisterId(Principal::anonymous())
        );
    }

    #[test]
    fn test_account_unique_indexes() {
        let mut account = mock_account();
        account.name = "Test".to_string();

        let indexes = account.to_unique_indexes();

        assert_eq!(indexes.len(), 1);
        assert_eq!(
            indexes[0].0,
            UniqueIndexKey::AccountName(format_unique_string("Test"))
        );
    }

    #[test]
    fn test_address_book_entry_unique_indexes() {
        let mut address_book_entry = mock_address_book_entry();
        address_book_entry.blockchain = Blockchain::InternetComputer;
        address_book_entry.address = "Test".to_string();

        let indexes = address_book_entry.to_unique_indexes();

        assert_eq!(indexes.len(), 1);
        assert_eq!(
            indexes[0].0,
            UniqueIndexKey::AddressBookBlockchainAddress(
                Blockchain::InternetComputer.to_string(),
                "Test".to_string()
            )
        );
    }
}
