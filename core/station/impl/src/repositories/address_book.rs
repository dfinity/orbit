use super::indexes::unique_index::UniqueIndexRepository;
use crate::{
    core::{
        metrics::ADDRESS_BOOK_METRICS, utils::max_string_of_size, with_memory_manager, Memory,
        ADDRESS_BOOK_MEMORY_ID,
    },
    models::{
        indexes::unique_index::UniqueIndexKey, AddressBookEntry, AddressBookEntryId,
        AddressBookEntryKey, AddressFormat, Blockchain,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use orbit_essentials::{
    repository::{IndexedRepository, Repository, StableDb},
    types::UUID,
};
use std::{cell::RefCell, collections::HashSet, sync::Arc};

thread_local! {
  /// The memory reference to the AddressBookEntry repository.
  static DB: RefCell<StableBTreeMap<AddressBookEntryKey, AddressBookEntry, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(ADDRESS_BOOK_MEMORY_ID))
    )
  })
}

lazy_static! {
    pub static ref ADDRESS_BOOK_REPOSITORY: Arc<AddressBookRepository> =
        Arc::new(AddressBookRepository::default());
}

/// A repository that enables managing address book entries in stable memory.
#[derive(Default, Debug)]
pub struct AddressBookRepository {
    unique_index: UniqueIndexRepository,
}

impl StableDb<AddressBookEntryKey, AddressBookEntry, VirtualMemory<Memory>>
    for AddressBookRepository
{
    fn with_db<F, R>(f: F) -> R
    where
        F: FnOnce(
            &mut StableBTreeMap<AddressBookEntryKey, AddressBookEntry, VirtualMemory<Memory>>,
        ) -> R,
    {
        DB.with(|m| f(&mut m.borrow_mut()))
    }
}

impl IndexedRepository<AddressBookEntryKey, AddressBookEntry, VirtualMemory<Memory>>
    for AddressBookRepository
{
    fn remove_entry_indexes(&self, value: &AddressBookEntry) {
        value.to_unique_indexes().iter().for_each(|(index, _)| {
            self.unique_index.remove(index);
        });
    }

    fn add_entry_indexes(&self, value: &AddressBookEntry) {
        value
            .to_unique_indexes()
            .into_iter()
            .for_each(|(index, entry_id)| {
                self.unique_index.insert(index, entry_id);
            });
    }

    /// Clears all the indexes for the repository.
    fn clear_indexes(&self) {
        self.unique_index
            .clear_when(|key| matches!(key, UniqueIndexKey::AddressBookBlockchainAddress(_, _)));
    }
}

impl Repository<AddressBookEntryKey, AddressBookEntry, VirtualMemory<Memory>>
    for AddressBookRepository
{
    fn insert(
        &self,
        key: AddressBookEntryKey,
        value: AddressBookEntry,
    ) -> Option<AddressBookEntry> {
        DB.with(|m| {
            let prev = m.borrow_mut().insert(key, value.clone());

            // Update metrics when an entry is upserted.
            ADDRESS_BOOK_METRICS.with(|metrics| {
                metrics
                    .iter()
                    .for_each(|metric| metric.borrow_mut().sum(&value, prev.as_ref()))
            });

            self.save_entry_indexes(&value, prev.as_ref());

            prev
        })
    }

    fn remove(&self, key: &AddressBookEntryKey) -> Option<AddressBookEntry> {
        DB.with(|m| {
            let prev = m.borrow_mut().remove(key);

            // Update metrics when an entry is removed.
            if let Some(prev) = &prev {
                ADDRESS_BOOK_METRICS.with(|metrics| {
                    metrics
                        .iter()
                        .for_each(|metric| metric.borrow_mut().sub(prev))
                });
            }

            if let Some(prev) = &prev {
                self.remove_entry_indexes(prev);
            }

            prev
        })
    }
}

impl AddressBookRepository {
    /// Get the address book entry by it's unique key.
    pub fn find_by_address(
        &self,
        blockchain: Blockchain,
        address: String,
    ) -> Option<AddressBookEntry> {
        self.unique_index
            .get(&UniqueIndexKey::AddressBookBlockchainAddress(
                blockchain.to_string().to_lowercase(),
                address,
            ))
            .and_then(|id| self.get(&AddressBookEntry::key(id)))
    }

    /// Checks if an address book entry exists for the given blockchain and address.
    pub fn exists(&self, blockchain: Blockchain, address: String) -> bool {
        self.unique_index
            .get(&UniqueIndexKey::AddressBookBlockchainAddress(
                blockchain.to_string().to_lowercase(),
                address,
            ))
            .is_some()
    }

    /// List all address book entries of a given blockchain.
    pub fn find_by_blockchain(&self, blockchain: Blockchain) -> Vec<AddressBookEntry> {
        self.unique_index
            .find_by_criteria(
                Some(UniqueIndexKey::AddressBookBlockchainAddress(
                    blockchain.to_string().to_lowercase(),
                    String::new(),
                )),
                Some(UniqueIndexKey::AddressBookBlockchainAddress(
                    blockchain.to_string().to_lowercase(),
                    max_string_of_size(&AddressBookEntry::ADDRESS_RANGE.1),
                )),
                None,
            )
            .iter()
            .filter_map(|id| self.get(&AddressBookEntry::key(*id)))
            .collect::<Vec<_>>()
    }

    pub fn find_by_ids(&self, ids: Vec<AddressBookEntryId>) -> Vec<AddressBookEntry> {
        ids.iter()
            .filter_map(|id| self.get(&AddressBookEntry::key(*id)))
            .collect::<Vec<_>>()
    }

    pub fn find_where(&self, where_clause: AddressBookWhereClause) -> Vec<AddressBookEntry> {
        let mut entries = match where_clause.blockchain {
            Some(blockchain) => self.find_by_blockchain(blockchain),
            _ => self.list(),
        };

        let where_labels = where_clause
            .labels
            .map(|labels| {
                labels
                    .into_iter()
                    .map(|label| label.to_lowercase())
                    .collect::<HashSet<_>>()
            })
            .unwrap_or_default();

        if !where_labels.is_empty() {
            entries.retain(|entry| {
                let entry_labels = entry
                    .labels
                    .iter()
                    .map(|label| label.to_lowercase())
                    .collect::<HashSet<_>>();

                entry_labels.intersection(&where_labels).count() == where_labels.len()
            });
        }

        if let Some(ids) = where_clause.ids {
            entries.retain(|entry| ids.contains(&entry.id));
        }

        if let Some(addresses) = where_clause.addresses {
            entries.retain(|entry| addresses.contains(&entry.address));
        }

        if let Some(address_formats) = where_clause.address_formats {
            entries.retain(|entry| address_formats.contains(&entry.address_format));
        }

        entries.sort();

        entries
    }
}

#[derive(Debug, Clone)]
pub struct AddressBookWhereClause {
    pub blockchain: Option<Blockchain>,
    pub labels: Option<Vec<String>>,
    pub addresses: Option<Vec<String>>,
    pub ids: Option<Vec<UUID>>,
    pub address_formats: Option<Vec<AddressFormat>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::address_book_entry_test_utils;

    #[test]
    fn test_crud() {
        let repository = AddressBookRepository::default();
        let address_book_entry = address_book_entry_test_utils::mock_address_book_entry();

        assert!(repository.get(&address_book_entry.to_key()).is_none());

        repository.insert(address_book_entry.to_key(), address_book_entry.clone());

        assert!(repository.get(&address_book_entry.to_key()).is_some());
        assert!(repository.remove(&address_book_entry.to_key()).is_some());
        assert!(repository.get(&address_book_entry.to_key()).is_none());
    }

    #[test]
    fn test_find_by_blockchain() {
        let repository = AddressBookRepository::default();

        let mut address_book_entry_0 = address_book_entry_test_utils::mock_address_book_entry();
        address_book_entry_0.blockchain = Blockchain::InternetComputer;
        repository.insert(address_book_entry_0.to_key(), address_book_entry_0.clone());

        let mut address_book_entry_1 = address_book_entry_test_utils::mock_address_book_entry();
        address_book_entry_1.blockchain = Blockchain::InternetComputer;
        address_book_entry_1.address = "0x5678".to_string();
        address_book_entry_1.id = [42; 16];
        repository.insert(address_book_entry_1.to_key(), address_book_entry_1.clone());

        let mut address_book_entry_2 = address_book_entry_test_utils::mock_address_book_entry();
        address_book_entry_2.blockchain = Blockchain::Ethereum;
        address_book_entry_2.id = [66; 16];
        repository.insert(address_book_entry_2.to_key(), address_book_entry_2.clone());

        let result = repository.find_by_blockchain(Blockchain::InternetComputer);
        assert!(result.contains(&address_book_entry_0));
        assert!(result.contains(&address_book_entry_1));
        assert!(!result.contains(&address_book_entry_2));
    }

    #[test]
    fn test_find_by_ids() {
        let repository = AddressBookRepository::default();
        let mut address_book_entry_0 = address_book_entry_test_utils::mock_address_book_entry();
        let mut address_book_entry_1 = address_book_entry_test_utils::mock_address_book_entry();
        address_book_entry_0.id = [1; 16];
        address_book_entry_1.id = [2; 16];

        repository.insert(address_book_entry_0.to_key(), address_book_entry_0.clone());
        repository.insert(address_book_entry_1.to_key(), address_book_entry_1.clone());

        let result = repository.find_by_ids(vec![address_book_entry_0.id, address_book_entry_1.id]);
        assert!(result.contains(&address_book_entry_0));
        assert!(result.contains(&address_book_entry_1));
    }

    #[test]
    fn test_find_by_address_formats() {
        let repository = AddressBookRepository::default();
        let mut address_book_entry_0 = address_book_entry_test_utils::mock_address_book_entry();
        let mut address_book_entry_1 = address_book_entry_test_utils::mock_address_book_entry();
        address_book_entry_0.id = [1; 16];
        address_book_entry_1.id = [2; 16];

        address_book_entry_0.address_format = AddressFormat::ICPAccountIdentifier;
        address_book_entry_1.address_format = AddressFormat::ICRC1Account;

        repository.insert(address_book_entry_0.to_key(), address_book_entry_0.clone());
        repository.insert(address_book_entry_1.to_key(), address_book_entry_1.clone());

        let result = repository.find_where(AddressBookWhereClause {
            blockchain: None,
            labels: None,
            addresses: None,
            ids: None,
            address_formats: Some(vec![AddressFormat::ICPAccountIdentifier]),
        });
        assert!(result.contains(&address_book_entry_0));
        assert_eq!(result.len(), 1);

        let result = repository.find_where(AddressBookWhereClause {
            blockchain: None,
            labels: None,
            addresses: None,
            ids: None,
            address_formats: Some(vec![AddressFormat::ICRC1Account]),
        });
        assert!(result.contains(&address_book_entry_1));
        assert_eq!(result.len(), 1);
    }
}
