use crate::{
    core::{
        utils::format_unique_string, with_memory_manager, Memory,
        NAME_TO_ACCOUNT_ID_INDEX_MEMORY_ID,
    },
    models::{
        indexes::name_to_account_id_index::{NameToAccountIdIndex, NameToAccountIdIndexCriteria},
        AccountId,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use orbit_essentials::repository::IndexRepository;
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<NameToAccountIdIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(NAME_TO_ACCOUNT_ID_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables finding accounts based on the id of the user in stable memory.
#[derive(Default, Debug)]
pub struct NameToAccountIdIndexRepository {}

impl IndexRepository<NameToAccountIdIndex, AccountId> for NameToAccountIdIndexRepository {
    type FindByCriteria = NameToAccountIdIndexCriteria;

    fn exists(&self, index: &NameToAccountIdIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: NameToAccountIdIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &NameToAccountIdIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<AccountId> {
        DB.with(|db| {
            let name = format_unique_string(&criteria.name);
            let start_key = NameToAccountIdIndex {
                name: name.clone(),
                account_id: [u8::MIN; 16],
            };
            let end_key = NameToAccountIdIndex {
                name,
                account_id: [u8::MAX; 16],
            };

            db.borrow()
                .range(start_key..=end_key)
                .map(|(index, _)| index.account_id)
                .collect::<HashSet<_>>()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_crud() {
        let repository = NameToAccountIdIndexRepository::default();
        let index = NameToAccountIdIndex {
            name: "testaccount".to_string(),
            account_id: [2; 16],
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_criteria() {
        let repository = NameToAccountIdIndexRepository::default();
        repository.insert(NameToAccountIdIndex {
            name: "testaccou".to_string(),
            account_id: [1; 16],
        });

        repository.insert(NameToAccountIdIndex {
            name: "testaccoun".to_string(),
            account_id: [2; 16],
        });

        repository.insert(NameToAccountIdIndex {
            name: "testaccount".to_string(),
            account_id: [3; 16],
        });

        let result = repository.find_by_criteria(NameToAccountIdIndexCriteria {
            name: "testaccoun".to_string(),
        });

        assert_eq!(result.len(), 1);
        assert!(result.contains(&[2; 16]));

        let result = repository.find_by_criteria(NameToAccountIdIndexCriteria {
            name: "testaccount".to_string(),
        });

        assert_eq!(result.len(), 1);
        assert!(result.contains(&[3; 16]));
    }
}
