use super::indexes::address_book_index::AddressBookIndexRepository;
use super::indexes::address_book_standard_index::AddressBookStandardIndexRepository;
use crate::{
    core::{metrics::ADDRESS_BOOK_METRICS, with_memory_manager, Memory, ADDRESS_BOOK_MEMORY_ID},
    models::{
        indexes::{
            address_book_index::AddressBookIndexCriteria,
            address_book_standard_index::AddressBookStandardIndexCriteria,
        },
        AddressBookEntry, AddressBookEntryId, AddressBookEntryKey, AddressChain, Blockchain,
        BlockchainStandard,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use orbit_essentials::{
    repository::{IndexRepository, RefreshIndexMode, Repository},
    types::UUID,
};
use std::{cell::RefCell, sync::Arc};

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
    index: AddressBookIndexRepository,
    standard_index: AddressBookStandardIndexRepository,
}

impl Repository<AddressBookEntryKey, AddressBookEntry> for AddressBookRepository {
    fn list(&self) -> Vec<AddressBookEntry> {
        DB.with(|m| m.borrow().iter().map(|(_, v)| v).collect())
    }

    fn get(&self, key: &AddressBookEntryKey) -> Option<AddressBookEntry> {
        DB.with(|m| m.borrow().get(key))
    }

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

            self.index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev.clone().map(|prev| prev.to_index()),
                    current: Some(value.to_index()),
                });

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

            self.index
                .refresh_index_on_modification(RefreshIndexMode::CleanupValue {
                    current: prev.clone().map(|prev| prev.to_index()),
                });

            prev
        })
    }

    fn len(&self) -> usize {
        DB.with(|m| m.borrow().len()) as usize
    }
}

impl AddressBookRepository {
    pub fn find_by_address(
        &self,
        blockchain: Blockchain,
        standard: BlockchainStandard,
        address: String,
    ) -> Option<AddressBookEntry> {
        let ids = self.index.find_by_criteria(AddressBookIndexCriteria {
            blockchain,
            standard,
            address,
        });

        ids.iter()
            .find_map(|id| self.get(&AddressBookEntry::key(*id)))
    }

    pub fn exists(
        &self,
        blockchain: Blockchain,
        standard: BlockchainStandard,
        address: String,
    ) -> bool {
        !self
            .index
            .find_by_criteria(AddressBookIndexCriteria {
                blockchain,
                standard,
                address,
            })
            .is_empty()
    }

    pub fn find_by_blockchain_standard(
        &self,
        blockchain: Blockchain,
        standard: BlockchainStandard,
    ) -> Vec<AddressBookEntry> {
        let ids = self
            .standard_index
            .find_by_criteria(AddressBookStandardIndexCriteria {
                blockchain,
                standard,
            });

        ids.iter()
            .filter_map(|id| self.get(&AddressBookEntry::key(*id)))
            .collect::<Vec<_>>()
    }

    pub fn find_by_ids(&self, ids: Vec<AddressBookEntryId>) -> Vec<AddressBookEntry> {
        ids.iter()
            .filter_map(|id| self.get(&AddressBookEntry::key(*id)))
            .collect::<Vec<_>>()
    }

    pub fn find_where(&self, where_clause: AddressBookWhereClause) -> Vec<AddressBookEntry> {
        let mut entries = match where_clause.address_chain {
            Some(chain) => self.find_by_blockchain_standard(chain.blockchain, chain.standard),
            _ => self.list(),
        };

        if let Some(ids) = where_clause.ids {
            entries.retain(|entry| ids.contains(&entry.id));
        }

        if let Some(addresses) = where_clause.addresses {
            entries.retain(|entry| addresses.contains(&entry.address));
        }

        entries.sort();

        entries
    }
}

#[derive(Debug, Clone)]
pub struct AddressBookWhereClause {
    pub address_chain: Option<AddressChain>,
    pub addresses: Option<Vec<String>>,
    pub ids: Option<Vec<UUID>>,
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
    fn test_find_by_blockchain_standard() {
        let repository = AddressBookRepository::default();

        let address_book_entry_0 = address_book_entry_test_utils::mock_address_book_entry();
        repository.insert(address_book_entry_0.to_key(), address_book_entry_0.clone());

        let mut address_book_entry_1 = address_book_entry_test_utils::mock_address_book_entry();
        address_book_entry_1.address = "0x5678".to_string();
        address_book_entry_1.id = [42; 16];
        repository.insert(address_book_entry_1.to_key(), address_book_entry_1.clone());

        let mut address_book_entry_2 = address_book_entry_test_utils::mock_address_book_entry();
        address_book_entry_2.standard = BlockchainStandard::ICRC1;
        address_book_entry_2.id = [66; 16];
        repository.insert(address_book_entry_2.to_key(), address_book_entry_2.clone());

        let result = repository
            .find_by_blockchain_standard(Blockchain::InternetComputer, BlockchainStandard::Native);
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
}
