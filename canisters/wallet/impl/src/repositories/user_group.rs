use crate::{
    core::{with_memory_manager, Memory, USER_GROUP_MEMORY_ID},
    models::{UserGroup, UserGroupKey},
};
use ic_canister_core::repository::Repository;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::cell::RefCell;

thread_local! {
  static DB: RefCell<StableBTreeMap<UserGroupKey, UserGroup, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(USER_GROUP_MEMORY_ID))
    )
  })
}

/// A repository that enables managing users in stable memory.
#[derive(Default, Debug)]
pub struct UserGroupRepository {}

impl Repository<UserGroupKey, UserGroup> for UserGroupRepository {
    fn get(&self, key: &UserGroupKey) -> Option<UserGroup> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: UserGroupKey, value: UserGroup) -> Option<UserGroup> {
        DB.with(|m| {
            let prev = m.borrow_mut().insert(key, value.clone());

            prev
        })
    }

    fn remove(&self, key: &UserGroupKey) -> Option<UserGroup> {
        DB.with(|m| {
            let prev = m.borrow_mut().remove(key);

            prev
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user_group_test_utils;

    #[test]
    fn test_crud() {
        let repository = UserGroupRepository::default();
        let user_group = user_group_test_utils::mock_user_group();

        assert!(repository.get(&user_group.to_key()).is_none());

        repository.insert(user_group.to_key(), user_group.clone());

        assert!(repository.get(&user_group.to_key()).is_some());
        assert!(repository.remove(&user_group.to_key()).is_some());
        assert!(repository.get(&user_group.to_key()).is_none());
    }
}
