use crate::{
    core::{with_memory_manager, Memory, UNIQUE_INDEX_MEMORY_ID},
    models::indexes::unique_index::UniqueIndexKey,
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use orbit_essentials::{
    repository::{Repository, StableDb},
    types::UUID,
};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<UniqueIndexKey, UUID, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(UNIQUE_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that holds unique indexes for the models.
#[derive(Default, Debug)]
pub struct UniqueIndexRepository {}

impl StableDb<UniqueIndexKey, UUID, VirtualMemory<Memory>> for UniqueIndexRepository {
    fn with_db<F, R>(f: F) -> R
    where
        F: FnOnce(&mut StableBTreeMap<UniqueIndexKey, UUID, VirtualMemory<Memory>>) -> R,
    {
        DB.with(|m| f(&mut m.borrow_mut()))
    }
}

impl Repository<UniqueIndexKey, UUID, VirtualMemory<Memory>> for UniqueIndexRepository {}

impl UniqueIndexRepository {
    /// Returns all the entries that are between the given keys.
    pub fn find_by_criteria(
        &self,
        start_key: Option<UniqueIndexKey>,
        end_key: Option<UniqueIndexKey>,
        take_limit: Option<usize>,
    ) -> HashSet<UUID> {
        DB.with(|m| match (start_key, end_key) {
            (Some(start), Some(end)) => m
                .borrow()
                .range(start..=end)
                .map(|(_, v)| v)
                .take(take_limit.unwrap_or(usize::MAX))
                .collect(),
            (Some(start), None) => m
                .borrow()
                .range(start..)
                .map(|(_, v)| v)
                .take(take_limit.unwrap_or(usize::MAX))
                .collect(),
            (None, Some(end)) => m
                .borrow()
                .range(..=end)
                .map(|(_, v)| v)
                .take(take_limit.unwrap_or(usize::MAX))
                .collect(),
            (None, None) => m
                .borrow()
                .iter()
                .map(|(_, v)| v)
                .take(take_limit.unwrap_or(usize::MAX))
                .collect(),
        })
    }

    /// Checks if the given key exists in the repository.
    pub fn exists(&self, key: &UniqueIndexKey) -> bool {
        DB.with(|m| m.borrow().get(key).is_some())
    }

    /// Clears all the entries in the repository that match the given criteria.
    pub fn clear_when<F>(&self, should_remove: F)
    where
        F: Fn(&UniqueIndexKey) -> bool,
    {
        DB.with(|db| {
            let mut keys_to_remove = Vec::new();

            for (key, _) in db.borrow().iter() {
                if should_remove(&key) {
                    keys_to_remove.push(key);
                }
            }

            keys_to_remove.into_iter().for_each(|key| {
                db.borrow_mut().remove(&key);
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::indexes::unique_index::UniqueIndexKey;

    #[test]
    fn test_unique_index_repository() {
        let repository = UniqueIndexRepository::default();

        let key = UniqueIndexKey::AccountName("test".to_string());
        let id = [1; 16];

        assert!(!repository.exists(&key));

        repository.insert(key.clone(), id);

        let another_key = UniqueIndexKey::AccountName("test2".to_string());
        let another_id = [2; 16];

        repository.insert(another_key.clone(), another_id);

        assert!(repository.exists(&key));
        assert_eq!(repository.get(&key), Some(id));

        repository.remove(&key);

        assert!(!repository.exists(&key));
    }
}
