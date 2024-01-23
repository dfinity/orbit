use crate::{
    core::{with_memory_manager, Memory, ADDRESS_BOOK_INDEX_MEMORY_ID},
    models::{
        indexes::address_book_index::{AddressBookIndex, AddressBookIndexCriteria},
        AddressBookEntryId,
    },
};
use ic_canister_core::repository::IndexRepository;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<AddressBookIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(ADDRESS_BOOK_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables finding address book entries based on the address in stable memory.
#[derive(Default, Debug)]
pub struct AddressBookIndexRepository {}

impl IndexRepository<AddressBookIndex, AddressBookEntryId> for AddressBookIndexRepository {
    type FindByCriteria = AddressBookIndexCriteria;

    fn exists(&self, index: &AddressBookIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: AddressBookIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &AddressBookIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<AddressBookEntryId> {
        DB.with(|db| {
            let start_key = AddressBookIndex {
                blockchain: criteria.blockchain.clone(),
                standard: criteria.standard.clone(),
                address_book_entry_id: [u8::MIN; 16],
                address: criteria.address.clone().unwrap_or_default(),
            };
            let end_key = AddressBookIndex {
                blockchain: criteria.blockchain,
                standard: criteria.standard,
                address_book_entry_id: [u8::MAX; 16],
                address: criteria.address.clone().unwrap_or_default(),
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
        let repository = AddressBookIndexRepository::default();
        let index = AddressBookIndex {
            blockchain: Blockchain::InternetComputer,
            standard: BlockchainStandard::Native,
            address_book_entry_id: [2; 16],
            address: "0x1234".to_string(),
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_criteria() {
        let repository = AddressBookIndexRepository::default();
        let index = AddressBookIndex {
            blockchain: Blockchain::InternetComputer,
            standard: BlockchainStandard::Native,
            address_book_entry_id: [2; 16],
            address: "0x1234".to_string(),
        };

        repository.insert(index.clone());

        let criteria = AddressBookIndexCriteria {
            blockchain: Blockchain::InternetComputer,
            standard: BlockchainStandard::Native,
            address: Some("0x1234".to_string()),
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&index.address_book_entry_id));
    }

    #[test]
    fn test_find_by_blockchain_standard() {
        let repository = AddressBookIndexRepository::default();

        let index_0 = AddressBookIndex {
            blockchain: Blockchain::InternetComputer,
            standard: BlockchainStandard::Native,
            address_book_entry_id: [0; 16],
            address: "0".to_string(),
        };
        repository.insert(index_0.clone());

        let index_1 = AddressBookIndex {
            blockchain: Blockchain::InternetComputer,
            standard: BlockchainStandard::ICRC1,
            address_book_entry_id: [1; 16],
            address: "1".to_string(),
        };
        repository.insert(index_1.clone());

        let index_2 = AddressBookIndex {
            blockchain: Blockchain::InternetComputer,
            standard: BlockchainStandard::Native,
            address_book_entry_id: [2; 16],
            address: "2".to_string(),
        };
        repository.insert(index_2.clone());

        let criteria = AddressBookIndexCriteria {
            blockchain: Blockchain::InternetComputer,
            standard: BlockchainStandard::Native,
            address: None,
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 2);
        assert!(result.contains(&index_0.address_book_entry_id));
        assert!(!result.contains(&index_1.address_book_entry_id));
        assert!(result.contains(&index_2.address_book_entry_id));
    }
}
