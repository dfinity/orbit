use super::indexes::{
    user_identity_index::UserIdentityIndexRepository,
    user_status_group_index::UserStatusGroupIndexRepository,
};
use crate::{
    core::{with_memory_manager, Memory, USER_MEMORY_ID},
    models::{
        indexes::{
            user_identity_index::UserIdentityIndexCriteria,
            user_status_group_index::UserStatusGroupIndexCriteria,
        },
        User, UserKey, UserStatus,
    },
};
use candid::Principal;
use ic_canister_core::repository::{IndexRepository, RefreshIndexMode};
use ic_canister_core::{repository::Repository, types::UUID};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use std::{cell::RefCell, sync::Arc};

thread_local! {
  static DB: RefCell<StableBTreeMap<UserKey, User, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(USER_MEMORY_ID))
    )
  })
}

lazy_static! {
    pub static ref USER_REPOSITORY: Arc<UserRepository> = Arc::new(UserRepository::default());
}

/// A repository that enables managing users in stable memory.
#[derive(Default, Debug)]
pub struct UserRepository {
    identity_index: UserIdentityIndexRepository,
    group_status_index: UserStatusGroupIndexRepository,
}

impl Repository<UserKey, User> for UserRepository {
    fn list(&self) -> Vec<User> {
        DB.with(|m| m.borrow().iter().map(|(_, v)| v).collect())
    }

    fn get(&self, key: &UserKey) -> Option<User> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: UserKey, value: User) -> Option<User> {
        DB.with(|m| {
            let prev = m.borrow_mut().insert(key, value.clone());

            self.identity_index
                .refresh_index_on_modification(RefreshIndexMode::List {
                    previous: prev
                        .clone()
                        .map_or(Vec::new(), |prev| prev.to_index_for_identities()),
                    current: value.to_index_for_identities(),
                });
            self.group_status_index
                .refresh_index_on_modification(RefreshIndexMode::List {
                    previous: prev
                        .clone()
                        .map_or(Vec::new(), |prev| prev.to_index_for_groups()),
                    current: value.to_index_for_groups(),
                });

            prev
        })
    }

    fn remove(&self, key: &UserKey) -> Option<User> {
        DB.with(|m| {
            let prev = m.borrow_mut().remove(key);
            self.identity_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupList {
                    current: prev
                        .clone()
                        .map_or(Vec::new(), |prev| prev.to_index_for_identities()),
                });
            self.group_status_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupList {
                    current: prev
                        .clone()
                        .map_or(Vec::new(), |prev| prev.to_index_for_groups()),
                });

            prev
        })
    }

    fn len(&self) -> usize {
        DB.with(|m| m.borrow().len()) as usize
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

    /// Returns the users associated with the given group and their user status if they exist.
    pub fn find_by_group_and_status(&self, group_id: &UUID, status: &UserStatus) -> Vec<User> {
        self.group_status_index
            .find_by_criteria(UserStatusGroupIndexCriteria {
                group_id: group_id.to_owned(),
                user_status: status.to_owned(),
            })
            .iter()
            .filter_map(|user_id| self.get(&User::key(*user_id)))
            .collect()
    }

    /// Returns the users that match the given filters.
    pub fn find_where(&self, filters: UserWhereClause) -> Vec<User> {
        let mut users = self.list();

        if let Some(search_term) = filters.search_term {
            users.retain(|user| {
                user.name.as_ref().map_or(false, |name| {
                    name.to_lowercase().starts_with(&search_term.to_lowercase())
                })
            });
        }

        if let Some(statuses) = filters.statuses {
            users.retain(|user| statuses.contains(&user.status));
        }

        users.sort();

        users
    }
}

#[derive(Debug, Clone)]
pub struct UserWhereClause {
    pub search_term: Option<String>,
    pub statuses: Option<Vec<UserStatus>>,
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

    #[test]
    fn test_find_by_group_and_user_status() {
        let repository = UserRepository::default();
        let mut user = user_test_utils::mock_user();
        user.groups = vec![[0; 16]];
        user.status = UserStatus::Inactive;
        repository.insert(user.to_key(), user.clone());

        let result = repository.find_by_group_and_status(&[0; 16], &UserStatus::Inactive);

        assert!(!result.is_empty());
    }
}
