use super::indexes::user_identity_index::UserIdentityIndexRepository;
use crate::{
    core::{with_memory_manager, Memory, USER_MEMORY_ID},
    models::{indexes::user_identity_index::UserIdentityIndexCriteria, User, UserKey},
};
use candid::Principal;
use ic_canister_core::repository::IndexRepository;
use ic_canister_core::repository::Repository;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use std::cell::RefCell;

thread_local! {
  static DB: RefCell<StableBTreeMap<UserKey, User, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(USER_MEMORY_ID))
    )
  })
}

lazy_static! {
    pub static ref USER_REPOSITORY: UserRepository = UserRepository::default();
}

/// A repository that enables managing users in stable memory.
#[derive(Default, Debug)]
pub struct UserRepository {
    identity_index: UserIdentityIndexRepository,
}

impl Repository<UserKey, User> for UserRepository {
    fn list(&self) -> Vec<User> {
        DB.with(|m| m.borrow().iter().map(|(_, v)| v).collect())
    }

    fn get(&self, key: &UserKey) -> Option<User> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: UserKey, value: User) -> Option<User> {
        DB.with(|m| match m.borrow_mut().insert(key, value.clone()) {
            Some(prev) => {
                let prev_identities = prev.to_index_for_identities();
                let curr_identities = value.to_index_for_identities();
                if prev_identities != curr_identities {
                    prev_identities.iter().for_each(|index| {
                        self.identity_index.remove(index);
                    });
                    curr_identities.iter().for_each(|index| {
                        self.identity_index.insert(index.to_owned());
                    });
                }

                Some(prev)
            }
            None => {
                value.to_index_for_identities().iter().for_each(|index| {
                    self.identity_index.insert(index.to_owned());
                });

                None
            }
        })
    }

    fn remove(&self, key: &UserKey) -> Option<User> {
        DB.with(|m| match m.borrow_mut().remove(key) {
            Some(prev) => {
                prev.to_index_for_identities().iter().for_each(|index| {
                    self.identity_index.remove(index);
                });

                Some(prev)
            }
            None => None,
        })
    }
}

impl UserRepository {
    /// Returns the user associated with the given identity if it exists.
    pub fn find_by_identity(&self, identity: &Principal) -> Option<User> {
        self.identity_index
            .find_by_criteria(UserIdentityIndexCriteria {
                identity_id: identity.to_owned(),
            })
            .iter()
            .find_map(|id| self.get(&User::key(*id)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user_test_utils;

    #[test]
    fn test_crud() {
        let repository = UserRepository::default();
        let user = user_test_utils::mock_user();

        assert!(repository.get(&user.to_key()).is_none());

        repository.insert(user.to_key(), user.clone());

        assert!(repository.get(&user.to_key()).is_some());
        assert!(repository.remove(&user.to_key()).is_some());
        assert!(repository.get(&user.to_key()).is_none());
    }

    #[test]
    fn test_find_by_identity() {
        let repository = UserRepository::default();
        let mut user = user_test_utils::mock_user();
        user.identities = vec![Principal::anonymous()];
        repository.insert(user.to_key(), user.clone());

        let result = repository.find_by_identity(&Principal::anonymous());

        assert!(result.is_some());
    }
}
