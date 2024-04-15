use crate::{
    core::{with_memory_manager, Memory, USER_STATUS_INDEX_MEMORY_ID},
    models::{
        indexes::user_status_index::{UserStatusIndex, UserStatusIndexCriteria},
        UserId,
    },
};
use ic_canister_core::repository::IndexRepository;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<UserStatusIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(USER_STATUS_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables managing user by user subscription status index in stable memory.
#[derive(Default, Debug)]
pub struct UserStatusIndexRepository {}

impl IndexRepository<UserStatusIndex, UserId> for UserStatusIndexRepository {
    type FindByCriteria = UserStatusIndexCriteria;

    fn exists(&self, index: &UserStatusIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: UserStatusIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &UserStatusIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<UserId> {
        DB.with(|db| {
            let start_key = UserStatusIndex {
                status: criteria.status.clone(),
                user_id: [u8::MIN; 16],
            };
            let end_key = UserStatusIndex {
                status: criteria.status,
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
    use crate::models::indexes::user_status_index::UserIndexSubscriptionStatus;

    #[test]
    fn test_user_status_index_repository() {
        let repository = UserStatusIndexRepository::default();
        let index = UserStatusIndex {
            status: UserIndexSubscriptionStatus::Pending,
            user_id: [1; 16],
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_status() {
        let repository = UserStatusIndexRepository::default();

        let index = UserStatusIndex {
            status: UserIndexSubscriptionStatus::Pending,
            user_id: [1; 16],
        };
        repository.insert(index.clone());
        let index = UserStatusIndex {
            status: UserIndexSubscriptionStatus::Unsubscribed,
            user_id: [2; 16],
        };
        repository.insert(index.clone());
        let index = UserStatusIndex {
            status: UserIndexSubscriptionStatus::Pending,
            user_id: [3; 16],
        };
        repository.insert(index.clone());

        let result = repository.find_by_criteria(UserStatusIndexCriteria {
            status: UserIndexSubscriptionStatus::Pending,
        });

        assert_eq!(result.len(), 2);
        assert!(result.contains(&[1; 16]));
        assert!(result.contains(&[3; 16]));
    }
}
