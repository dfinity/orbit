use crate::{
    core::{with_memory_manager, Memory, USER_GROUP_NAME_INDEX_MEMORY_ID},
    models::indexes::user_group_name_index::{UserGroupNameIndex, UserGroupNameIndexCriteria},
};
use ic_canister_core::{repository::IndexRepository, types::UUID};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<UserGroupNameIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(USER_GROUP_NAME_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables efficient searching of user groups by name.
#[derive(Default, Debug)]
pub struct UserGroupNameIndexRepository {}

impl IndexRepository<UserGroupNameIndex, UUID> for UserGroupNameIndexRepository {
    type FindByCriteria = UserGroupNameIndexCriteria;

    fn exists(&self, index: &UserGroupNameIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: UserGroupNameIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &UserGroupNameIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<UUID> {
        DB.with(|db| {
            let start_key = UserGroupNameIndex {
                name: criteria.name.clone(),
                user_group_id: [u8::MIN; 16],
            };
            let end_key = UserGroupNameIndex {
                name: criteria.name,
                user_group_id: [u8::MAX; 16],
            };

            db.borrow()
                .range(start_key..=end_key)
                .map(|(index, _)| index.user_group_id)
                .collect::<HashSet<UUID>>()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_identity_index_repository() {
        let repository = UserGroupNameIndexRepository::default();
        let index = UserGroupNameIndex {
            name: "Finance".to_string(),
            user_group_id: [1; 16],
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_name() {
        let repository = UserGroupNameIndexRepository::default();
        let index = UserGroupNameIndex {
            name: "Finance".to_string(),
            user_group_id: [1; 16],
        };

        repository.insert(index.clone());

        let result = repository.find_by_criteria(UserGroupNameIndexCriteria {
            name: "Finance".to_string(),
        });

        assert!(!result.is_empty());
        assert!(result.contains(&[1; 16]));
    }
}
