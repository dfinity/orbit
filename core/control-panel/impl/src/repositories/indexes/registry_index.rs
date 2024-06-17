use crate::{
    core::{with_memory_manager, Memory, REGISTRY_INDEX_MEMORY_ID},
    models::{
        indexes::registry_index::{RegistryIndex, RegistryIndexCriteria},
        RegistryEntryId,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use orbit_essentials::repository::IndexRepository;
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<RegistryIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(REGISTRY_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that holds the registry indexes, which is used to find registry entries efficiently.
#[derive(Default, Debug)]
pub struct RegistryIndexRepository {}

impl IndexRepository<RegistryIndex, RegistryEntryId> for RegistryIndexRepository {
    type FindByCriteria = RegistryIndexCriteria;

    fn exists(&self, index: &RegistryIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: RegistryIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &RegistryIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<RegistryEntryId> {
        DB.with(|db| {
            let start_key = RegistryIndex {
                index: criteria.from,
                registry_entry_id: [u8::MIN; 16],
            };
            let end_key = RegistryIndex {
                index: criteria.to,
                registry_entry_id: [u8::MAX; 16],
            };

            db.borrow()
                .range(start_key..=end_key)
                .map(|(index, _)| index.registry_entry_id)
                .collect::<HashSet<RegistryEntryId>>()
        })
    }
}

impl RegistryIndexRepository {
    pub fn len(&self) -> u64 {
        DB.with(|m| m.borrow().len())
    }

    pub fn is_empty(&self) -> bool {
        DB.with(|m| m.borrow().is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::indexes::registry_index::RegistryIndexKind;
    use orbit_essentials::repository::IndexRepository;

    #[test]
    fn test_index_repository() {
        let repository = RegistryIndexRepository::default();
        let index = RegistryIndex {
            index: RegistryIndexKind::Fullname("test".to_string()),
            registry_entry_id: [1; 16],
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_namespace() {
        let repository = RegistryIndexRepository::default();
        for i in 0..10 {
            repository.insert(RegistryIndex {
                index: RegistryIndexKind::Namespace(format!("ns-{}", i)),
                registry_entry_id: [i; 16],
            });
        }

        let result = repository.find_by_criteria(RegistryIndexCriteria {
            from: RegistryIndexKind::Namespace("ns-0".to_string()),
            to: RegistryIndexKind::Namespace("ns-2".to_string()),
        });

        assert!(!result.is_empty());
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_find_by_categories() {
        let repository = RegistryIndexRepository::default();
        for i in 0..10 {
            repository.insert(RegistryIndex {
                index: RegistryIndexKind::Category(i.to_string()),
                registry_entry_id: [i; 16],
            });
        }

        let result = repository.find_by_criteria(RegistryIndexCriteria {
            from: RegistryIndexKind::Category("3".to_string()),
            to: RegistryIndexKind::Category("4".to_string()),
        });

        assert!(!result.is_empty());
        assert_eq!(result.len(), 2);
    }
}
