use super::indexes::unique_index::UniqueIndexRepository;
use crate::{
    core::{
        cache::Cache, ic_cdk::api::print, metrics::USER_GROUP_METRICS, utils::format_unique_string,
        with_memory_manager, Memory, USER_GROUP_MEMORY_ID,
    },
    models::{indexes::unique_index::UniqueIndexKey, UserGroup, UserGroupId},
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use orbit_essentials::{
    repository::{IndexedRepository, Repository, StableDb},
    types::UUID,
};
use std::{cell::RefCell, sync::Arc};

thread_local! {
  static DB: RefCell<StableBTreeMap<UUID, UserGroup, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(USER_GROUP_MEMORY_ID))
    )
  });

  static CACHE: RefCell<Cache<UserGroupId, UserGroup>> = RefCell::new(Cache::new(UserGroupRepository::MAX_CACHE_SIZE));
}

lazy_static! {
    pub static ref USER_GROUP_REPOSITORY: Arc<UserGroupRepository> =
        Arc::new(UserGroupRepository::default());
}

/// A repository that enables managing users in stable memory.
#[derive(Default, Debug)]
pub struct UserGroupRepository {
    unique_index: UniqueIndexRepository,
}

impl StableDb<UUID, UserGroup, VirtualMemory<Memory>> for UserGroupRepository {
    fn with_db<F, R>(f: F) -> R
    where
        F: FnOnce(&mut StableBTreeMap<UUID, UserGroup, VirtualMemory<Memory>>) -> R,
    {
        DB.with(|m| f(&mut m.borrow_mut()))
    }
}

impl IndexedRepository<UUID, UserGroup, VirtualMemory<Memory>> for UserGroupRepository {
    fn remove_entry_indexes(&self, entry: &UserGroup) {
        entry
            .to_unique_indexes()
            .into_iter()
            .for_each(|(index, _)| {
                self.unique_index.remove(&index);
            });
    }

    fn add_entry_indexes(&self, entry: &UserGroup) {
        entry
            .to_unique_indexes()
            .into_iter()
            .for_each(|(index, id)| {
                self.unique_index.insert(index, id);
            });
    }

    /// Clears all the indexes for the user group.
    fn clear_indexes(&self) {
        CACHE.with(|cache| cache.borrow_mut().clear());

        self.unique_index
            .clear_when(|key| matches!(key, UniqueIndexKey::UserGroupName(_)));
    }
}

impl Repository<UUID, UserGroup, VirtualMemory<Memory>> for UserGroupRepository {
    fn list(&self) -> Vec<UserGroup> {
        let mut user_groups = Vec::with_capacity(self.len());

        if self.use_only_cache() {
            CACHE.with(|cache| {
                cache.borrow().iter().for_each(|(_, user_group)| {
                    user_groups.push(user_group.clone());
                });
            });
        } else {
            Self::with_db(|db| {
                db.iter().for_each(|(_, user_group)| {
                    user_groups.push(user_group);
                });
            });
        }

        user_groups
    }

    fn get(&self, key: &UserGroupId) -> Option<UserGroup> {
        let maybe_cache_hit = CACHE.with(|cache| cache.borrow().get(key).cloned());

        match self.use_only_cache() {
            true => maybe_cache_hit,
            false => maybe_cache_hit.or_else(|| Self::with_db(|db| db.get(key))),
        }
    }

    fn insert(&self, key: UserGroupId, value: UserGroup) -> Option<UserGroup> {
        DB.with(|m| {
            CACHE.with(|cache| cache.borrow_mut().insert(key, value.clone()));

            let prev = m.borrow_mut().insert(key, value.clone());

            // Update metrics when a user group is upserted.
            USER_GROUP_METRICS.with(|metrics| {
                metrics
                    .iter()
                    .for_each(|metric| metric.borrow_mut().sum(&value, prev.as_ref()))
            });

            self.save_entry_indexes(&value, prev.as_ref());

            prev
        })
    }

    fn remove(&self, key: &UserGroupId) -> Option<UserGroup> {
        DB.with(|m| {
            CACHE.with(|cache| cache.borrow_mut().remove(key));

            let prev = m.borrow_mut().remove(key);

            // Update metrics when a user group is removed.
            if let Some(prev) = &prev {
                USER_GROUP_METRICS.with(|metrics| {
                    metrics
                        .iter()
                        .for_each(|metric| metric.borrow_mut().sub(prev))
                });

                self.remove_entry_indexes(prev);
            }

            prev
        })
    }
}

impl UserGroupRepository {
    /// Currently the cache uses around 100 bytes per entry (UUID, UserGroup),
    /// so the max cache storage size is around 10MiB.
    pub const MAX_CACHE_SIZE: usize = 100_000;

    /// Checks if every user group in the repository is in the cache.
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
                "Only the first {} user groups will be added to the cache, the reposity has {} user groups.",
                Self::MAX_CACHE_SIZE,
                self.len(),
            ));
        }

        CACHE.with(|cache| {
            cache.borrow_mut().clear();

            DB.with(|db| {
                for (_, user_group) in db.borrow().iter().take(Self::MAX_CACHE_SIZE) {
                    cache.borrow_mut().insert(user_group.id, user_group);
                }
            });
        });
    }

    pub fn find_by_name(&self, name: &str) -> Option<UserGroup> {
        self.unique_index
            .get(&UniqueIndexKey::UserGroupName(format_unique_string(name)))
            .and_then(|id| self.get(&id))
    }

    pub fn find_where(&self, where_clause: UseGroupWhereClause) -> Vec<UserGroup> {
        let mut user_groups = self.list();

        if let Some(search_term) = where_clause.search_term {
            user_groups.retain(|user_group| {
                user_group
                    .name
                    .to_lowercase()
                    .contains(&search_term.to_lowercase())
            });
        }

        user_groups.sort();

        user_groups
    }
}

#[derive(Debug, Clone)]
pub struct UseGroupWhereClause {
    pub search_term: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::user_group_test_utils;

    #[test]
    fn test_crud() {
        let repository = UserGroupRepository::default();
        let user_group = user_group_test_utils::mock_user_group();

        assert!(repository.get(&user_group.id).is_none());

        repository.insert(user_group.id.to_owned(), user_group.clone());

        assert!(repository.get(&user_group.id).is_some());
        assert!(repository.remove(&user_group.id).is_some());
        assert!(repository.get(&user_group.id).is_none());
    }

    #[test]
    fn test_find_by_name() {
        let repository = UserGroupRepository::default();
        let user_group = user_group_test_utils::mock_user_group();

        assert!(repository.find_by_name(&user_group.name).is_none());

        repository.insert(user_group.id, user_group.clone());

        assert!(repository.find_by_name(&user_group.name).is_some());
        assert!(repository.remove(&user_group.id).is_some());
        assert!(repository.find_by_name(&user_group.name).is_none());
    }
}
