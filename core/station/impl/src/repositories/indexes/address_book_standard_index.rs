use crate::{
    core::{with_memory_manager, Memory, ADDRESS_BOOK_INDEX_MEMORY_ID},
    models::{
        indexes::address_book_standard_index::{
            AddressBookStandardIndex, AddressBookStandardIndexCriteria,
        },
        AddressBookEntryId,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use orbit_essentials::repository::IndexRepository;
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<AddressBookStandardIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(ADDRESS_BOOK_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables finding address book entries based on the address in stable memory.
#[derive(Default, Debug)]
pub struct AddressBookStandardIndexRepository {}

impl IndexRepository<AddressBookStandardIndex, AddressBookEntryId>
    for AddressBookStandardIndexRepository
{
    type FindByCriteria = AddressBookStandardIndexCriteria;

    fn exists(&self, index: &AddressBookStandardIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: AddressBookStandardIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &AddressBookStandardIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<AddressBookEntryId> {
        DB.with(|db| {
            let start_key = AddressBookStandardIndex {
                blockchain: criteria.blockchain.clone(),
                standard: criteria.standard.clone(),
                address_book_entry_id: [u8::MIN; 16],
            };
            let end_key = AddressBookStandardIndex {
                blockchain: criteria.blockchain,
                standard: criteria.standard,
                address_book_entry_id: [u8::MAX; 16],
            };

            db.borrow()
                .range(start_key..=end_key)
                .map(|(index, _)| index.address_book_entry_id)
                .collect::<HashSet<_>>()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{Blockchain, BlockchainStandard};

    #[test]
    fn test_repository_crud() {
        let repository = AddressBookStandardIndexRepository::default();
        let index = AddressBookStandardIndex {
            blockchain: Blockchain::InternetComputer,
            standard: BlockchainStandard::Native,
            address_book_entry_id: [2; 16],
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_criteria() {
        let repository = AddressBookStandardIndexRepository::default();
        let index = AddressBookStandardIndex {
            blockchain: Blockchain::InternetComputer,
            standard: BlockchainStandard::Native,
            address_book_entry_id: [2; 16],
        };

        repository.insert(index.clone());

        let criteria = AddressBookStandardIndexCriteria {
            blockchain: Blockchain::InternetComputer,
            standard: BlockchainStandard::Native,
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&index.address_book_entry_id));
    }

    #[test]
    fn test_find_by_blockchain_standard() {
        let repository = AddressBookStandardIndexRepository::default();

        let index_0 = AddressBookStandardIndex {
            blockchain: Blockchain::InternetComputer,
            standard: BlockchainStandard::Native,
            address_book_entry_id: [0; 16],
        };
        repository.insert(index_0.clone());

        let index_1 = AddressBookStandardIndex {
            blockchain: Blockchain::InternetComputer,
            standard: BlockchainStandard::ICRC1,
            address_book_entry_id: [1; 16],
        };
        repository.insert(index_1.clone());

        let index_2 = AddressBookStandardIndex {
            blockchain: Blockchain::InternetComputer,
            standard: BlockchainStandard::Native,
            address_book_entry_id: [2; 16],
        };
        repository.insert(index_2.clone());

        let criteria = AddressBookStandardIndexCriteria {
            blockchain: Blockchain::InternetComputer,
            standard: BlockchainStandard::Native,
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 2);
        assert!(result.contains(&index_0.address_book_entry_id));
        assert!(!result.contains(&index_1.address_book_entry_id));
        assert!(result.contains(&index_2.address_book_entry_id));
    }
}
