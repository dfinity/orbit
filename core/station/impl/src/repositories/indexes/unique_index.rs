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

    /// Refreshes the unique index repository with the given current and previous indexes.
    pub fn refresh(&self, current: &[(UniqueIndexKey, UUID)], previous: &[(UniqueIndexKey, UUID)]) {
        DB.with(|m| {
            let set_current: HashSet<UniqueIndexKey> =
                current.iter().map(|(k, _)| k.clone()).collect();
            let mut already_inserted = HashSet::new();

            for (key, value) in previous {
                if !set_current.contains(key) {
                    m.borrow_mut().remove(key);
                } else {
                    already_inserted.insert((key, value));
                }
            }

            for (key, value) in current {
                if !already_inserted.contains(&(key, value)) {
                    m.borrow_mut().insert(key.clone(), *value);
                }
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::indexes::unique_index::UniqueIndexKey;

    #[test]
    fn refresh_removes_unused_keys() {
        let repository = UniqueIndexRepository::default();
        let current = vec![(UniqueIndexKey::UserName("name".to_owned()), [1u8; 16])];
        let previous = vec![(UniqueIndexKey::UserName("old_name".to_owned()), [2u8; 16])];

        for (index, id) in &previous {
            repository.insert(index.clone(), *id);
        }

        assert_eq!(repository.len(), 1);
        assert_eq!(
            repository.get(&UniqueIndexKey::UserName("old_name".to_owned())),
            Some([2u8; 16])
        );

        repository.refresh(&current, &previous);

        assert_eq!(repository.len(), 1);
        assert_eq!(
            repository.get(&UniqueIndexKey::UserName("old_name".to_owned())),
            None
        );
        assert_eq!(
            repository.get(&UniqueIndexKey::UserName("name".to_owned())),
            Some([1u8; 16])
        );
    }
}
