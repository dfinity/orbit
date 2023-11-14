use crate::{
    core::{with_memory_manager, Memory, ACCOUNT_USER_INDEX_MEMORY_ID},
    models::{
        indexes::account_user_index::{AccountUserIndex, AccountUserIndexCriteria},
        AccountId,
    },
};
use ic_canister_core::repository::IndexRepository;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<AccountUserIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(ACCOUNT_USER_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables finding accounts based on the id of the user in stable memory.
#[derive(Default, Debug)]
pub struct AccountUserIndexRepository {}

impl IndexRepository<AccountUserIndex, AccountId> for AccountUserIndexRepository {
    type FindByCriteria = AccountUserIndexCriteria;

    fn exists(&self, index: &AccountUserIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: AccountUserIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &AccountUserIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<AccountId> {
        DB.with(|db| {
            let start_key = AccountUserIndex {
                user_id: criteria.user_id,
                account_id: [u8::MIN; 16],
            };
            let end_key = AccountUserIndex {
                user_id: criteria.user_id,
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
        let repository = AccountUserIndexRepository::default();
        let index = AccountUserIndex {
            user_id: [1; 16],
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
        let repository = AccountUserIndexRepository::default();
        let index = AccountUserIndex {
            user_id: [1; 16],
            account_id: [2; 16],
        };

        repository.insert(index.clone());

        let criteria = AccountUserIndexCriteria { user_id: [1; 16] };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&index.account_id));
    }
}
