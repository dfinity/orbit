use crate::{
    core::{with_memory_manager, Memory, USER_IDENTITY_INDEX_MEMORY_ID},
    models::{
        indexes::user_identity_index::{UserIdentityIndex, UserIdentityIndexCriteria},
        UserId,
    },
};
use ic_canister_core::repository::IndexRepository;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<UserIdentityIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(USER_IDENTITY_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables managing user by identity index in stable memory.
#[derive(Default, Debug)]
pub struct UserIdentityIndexRepository {}

impl IndexRepository<UserIdentityIndex, UserId> for UserIdentityIndexRepository {
    type FindByCriteria = UserIdentityIndexCriteria;

    fn exists(&self, index: &UserIdentityIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: UserIdentityIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &UserIdentityIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<UserId> {
        DB.with(|db| {
            let start_key = UserIdentityIndex {
                identity_id: criteria.identity_id,
                user_id: [u8::MIN; 16],
            };
            let end_key = UserIdentityIndex {
                identity_id: criteria.identity_id,
                user_id: [u8::MAX; 16],
            };

            db.borrow()
                .range(start_key..=end_key)
                .map(|(index, _)| index.user_id)
                .collect::<HashSet<UserId>>()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use candid::Principal;

    #[test]
    fn test_user_identity_index_repository() {
        let repository = UserIdentityIndexRepository::default();
        let index = UserIdentityIndex {
            identity_id: Principal::from_slice(&[1; 29]),
            user_id: [1; 16],
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_identity() {
        let repository = UserIdentityIndexRepository::default();
        let index = UserIdentityIndex {
            identity_id: Principal::from_slice(&[1; 29]),
            user_id: [1; 16],
        };

        repository.insert(index.clone());

        let result = repository.find_by_criteria(UserIdentityIndexCriteria {
            identity_id: Principal::from_slice(&[1; 29]),
        });

        assert!(!result.is_empty());
        assert!(result.contains(&[1; 16]));
    }
}
