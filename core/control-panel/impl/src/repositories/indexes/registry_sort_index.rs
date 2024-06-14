use crate::{
    core::{with_memory_manager, Memory, REGISTRY_SORT_INDEX_MEMORY_ID},
    models::{indexes::registry_sort_index::RegistrySortIndex, RegistryEntryId},
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use orbit_essentials::repository::Repository;
use std::cell::RefCell;

thread_local! {
  static DB: RefCell<StableBTreeMap<RegistryEntryId, RegistrySortIndex, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(REGISTRY_SORT_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables sorting requests in a more efficient way.
#[derive(Default, Debug)]
pub struct RegistrySortIndexRepository {}

impl Repository<RegistryEntryId, RegistrySortIndex> for RegistrySortIndexRepository {
    fn list(&self) -> Vec<RegistrySortIndex> {
        DB.with(|m| m.borrow().iter().map(|(_, v)| v).collect())
    }

    fn get(&self, key: &RegistryEntryId) -> Option<RegistrySortIndex> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: RegistryEntryId, value: RegistrySortIndex) -> Option<RegistrySortIndex> {
        DB.with(|m| m.borrow_mut().insert(key, value.clone()))
    }

    fn remove(&self, key: &RegistryEntryId) -> Option<RegistrySortIndex> {
        DB.with(|m| m.borrow_mut().remove(key))
    }

    fn len(&self) -> usize {
        DB.with(|m| m.borrow().len()) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::registry_entry_test_utils::create_registry_entry;

    #[test]
    fn check_crud_operations() {
        let repository = RegistrySortIndexRepository::default();
        let entry = create_registry_entry();

        assert!(repository.get(&entry.id).is_none());

        repository.insert(entry.id, entry.to_sort_index());
        assert_eq!(repository.len(), 1);
        assert_eq!(repository.get(&entry.id), Some(entry.to_sort_index()));

        repository.remove(&entry.id);

        assert!(repository.get(&entry.id).is_none());

        assert_eq!(repository.len(), 0);
    }
}
