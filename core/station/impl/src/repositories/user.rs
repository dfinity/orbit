use super::indexes::{
    unique_index::UniqueIndexRepository, user_status_group_index::UserStatusGroupIndexRepository,
};
use crate::core::ic_cdk::api::print;
use crate::{
    core::{
        cache::Cache, metrics::USER_METRICS, observer::Observer, utils::format_unique_string,
        with_memory_manager, Memory, USER_MEMORY_ID,
    },
    models::{
        indexes::{
            unique_index::UniqueIndexKey, user_status_group_index::UserStatusGroupIndexCriteria,
        },
        User, UserGroupId, UserId, UserKey, UserStatus,
    },
    services::{disaster_recovery_observes_insert_user, disaster_recovery_observes_remove_user},
};
use candid::Principal;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use orbit_essentials::repository::{IndexRepository, IndexedRepository, StableDb};
use orbit_essentials::{repository::Repository, types::UUID};
use std::{cell::RefCell, sync::Arc};

thread_local! {
  static DB: RefCell<StableBTreeMap<UserKey, User, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(USER_MEMORY_ID))
    )
  });

  static CACHE: RefCell<Cache<UserId, User>> = RefCell::new(Cache::new(UserRepository::MAX_CACHE_SIZE));
}

lazy_static! {
    pub static ref USER_REPOSITORY: Arc<UserRepository> = Arc::new(UserRepository::default());
}

/// A repository that enables managing users in stable memory.
#[derive(Debug)]
pub struct UserRepository {
    unique_index: UniqueIndexRepository,
    group_status_index: UserStatusGroupIndexRepository,
    change_observer: Observer<(User, Option<User>)>,
    remove_observer: Observer<User>,
}

impl Default for UserRepository {
    fn default() -> Self {
        let mut change_observer = Observer::default();
        disaster_recovery_observes_insert_user(&mut change_observer);

        let mut remove_observer = Observer::default();
        disaster_recovery_observes_remove_user(&mut remove_observer);

        Self {
            change_observer,
            remove_observer,
            unique_index: UniqueIndexRepository::default(),
            group_status_index: UserStatusGroupIndexRepository::default(),
        }
    }
}

impl StableDb<UserKey, User, VirtualMemory<Memory>> for UserRepository {
    fn with_db<F, R>(f: F) -> R
    where
        F: FnOnce(&mut StableBTreeMap<UserKey, User, VirtualMemory<Memory>>) -> R,
    {
        DB.with(|m| f(&mut m.borrow_mut()))
    }
}

impl IndexedRepository<UserKey, User, VirtualMemory<Memory>> for UserRepository {
    fn remove_entry_indexes(&self, entry: &User) {
        self.unique_index.refresh(&[], &entry.to_unique_indexes());

        entry.to_index_for_groups().iter().for_each(|index| {
            self.group_status_index.remove(index);
        });
    }

    fn add_entry_indexes(&self, entry: &User) {
        self.unique_index.refresh(&entry.to_unique_indexes(), &[]);

        entry.to_index_for_groups().iter().for_each(|index| {
            self.group_status_index.insert(index.clone());
        });
    }
}

impl Repository<UserKey, User, VirtualMemory<Memory>> for UserRepository {
    fn list(&self) -> Vec<User> {
        let mut users = Vec::with_capacity(self.len());

        if self.use_only_cache() {
            CACHE.with(|cache| {
                cache
                    .borrow()
                    .iter()
                    .for_each(|(_, user)| users.push(user.clone()))
            });
        } else {
            Self::with_db(|db| {
                db.iter().for_each(|(_, user)| users.push(user));
            });
        }

        users
    }

    fn get(&self, key: &UserKey) -> Option<User> {
        DB.with(|m| {
            let maybe_cache_hit = CACHE.with(|cache| cache.borrow().get(&key.id).cloned());

            match self.use_only_cache() {
                true => maybe_cache_hit,
                false => maybe_cache_hit.or_else(|| m.borrow().get(key)),
            }
        })
    }

    fn insert(&self, key: UserKey, value: User) -> Option<User> {
        DB.with(|m| {
            CACHE.with(|cache| cache.borrow_mut().insert(key.id, value.clone()));

            let prev = m.borrow_mut().insert(key, value.clone());

            // Update metrics when a user is upserted.
            USER_METRICS.with(|metrics| {
                metrics
                    .iter()
                    .for_each(|metric| metric.borrow_mut().sum(&value, prev.as_ref()))
            });

            self.save_entry_indexes(&value, prev.as_ref());

            let args = (value, prev);
            self.change_observer.notify(&args);

            args.1
        })
    }

    fn remove(&self, key: &UserKey) -> Option<User> {
        DB.with(|m| {
            CACHE.with(|cache| cache.borrow_mut().remove(&key.id));

            let prev = m.borrow_mut().remove(key);

            // Update metrics when a user is removed.
            if let Some(prev) = &prev {
                USER_METRICS.with(|metrics| {
                    metrics
                        .iter()
                        .for_each(|metric| metric.borrow_mut().sub(prev))
                });
            }

            if let Some(prev) = &prev {
                self.remove_entry_indexes(prev);
            }

            if let Some(prev) = &prev {
                self.remove_observer.notify(prev);
            }

            prev
        })
    }
}

impl UserRepository {
    /// Currently the cache uses around 250 bytes per entry (UUID, User),
    /// so the max cache size is around 12.5 MiB.
    ///
    /// Moreover, it takes approximately 36_800 instructions to load each entry, which means that
    /// rebuilding the cache from the repository would take around 1.84B instructions.
    pub const MAX_CACHE_SIZE: usize = 50_000;

    /// Checks if every user in the repository is in the cache.
    fn use_only_cache(&self) -> bool {
        self.len() <= Self::MAX_CACHE_SIZE
    }

    /// Builds the cache from the stable memory repository.
    ///
    /// This method should only be called during init or upgrade hooks to ensure that the cache is
    /// up-to-date with the repository and that we have enough instructions to rebuild the cache.
    pub fn build_cache(&self) {
        if self.len() > Self::MAX_CACHE_SIZE {
            print(format!(
                "Only the first {} users will be added to the cache, the reposity has {} users.",
                Self::MAX_CACHE_SIZE,
                USER_REPOSITORY.len(),
            ));
        }

        CACHE.with(|cache| {
            cache.borrow_mut().clear();

            DB.with(|db| {
                for (_, user) in db.borrow().iter().take(Self::MAX_CACHE_SIZE) {
                    cache.borrow_mut().insert(user.id, user);
                }
            });
        });
    }

    /// Returns the user associated with the given identity if it exists.
    pub fn find_by_identity(&self, identity: &Principal) -> Option<User> {
        self.unique_index
            .get(&UniqueIndexKey::UserIdentity(*identity))
            .and_then(|id| self.get(&User::key(id)))
    }

    // Returns the user associated with the given name if it exists.
    pub fn find_by_name(&self, name: &str) -> Option<UserId> {
        self.unique_index
            .get(&UniqueIndexKey::UserName(format_unique_string(name)))
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
                user.name
                    .to_lowercase()
                    .starts_with(&search_term.to_lowercase())
            });
        }

        if let Some(statuses) = filters.statuses {
            users.retain(|user| statuses.contains(&user.status));
        }

        if let Some(groups) = filters.groups {
            users.retain(|user| user.groups.iter().any(|group| groups.contains(group)));
        }

        users.sort();

        users
    }

    pub fn with_empty_observers() -> Self {
        Self {
            change_observer: Observer::default(),
            remove_observer: Observer::default(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct UserWhereClause {
    pub search_term: Option<String>,
    pub statuses: Option<Vec<UserStatus>>,
    pub groups: Option<Vec<UserGroupId>>,
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
        user.identities = vec![Principal::from_slice(&[1; 29])];
        repository.insert(user.to_key(), user.clone());

        let result = repository.find_by_identity(&Principal::from_slice(&[1; 29]));

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

#[cfg(feature = "canbench")]
mod benchs {
    use super::*;
    use crate::models::user_test_utils;
    use canbench_rs::{bench, BenchResult};
    use orbit_essentials::model::ModelKey;

    #[bench(raw)]
    fn find_100_users_from_50k_user_dataset() -> BenchResult {
        for i in 0..50_000 {
            let mut user = user_test_utils::mock_user();
            if i < 100 {
                user.name = format!("lookup_user_{}", i);
            }

            USER_REPOSITORY.insert(user.key(), user);
        }

        canbench_rs::bench_fn(|| {
            let users = USER_REPOSITORY.find_where(UserWhereClause {
                groups: None,
                statuses: Some(vec![UserStatus::Active]),
                search_term: Some("lookup_user_".to_string()),
            });

            if users.len() != 100 {
                panic!("Expected 100 users, got {}", users.len());
            }
        })
    }
}
