use crate::{
    core::{
        utils::format_unique_string, with_memory_manager, Memory, NAME_TO_USER_ID_INDEX_MEMORY_ID,
    },
    models::{
        indexes::name_to_user_id_index::{NameToUserIdIndex, NameToUserIdIndexCriteria},
        UserId,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use orbit_essentials::repository::IndexRepository;
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<NameToUserIdIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(NAME_TO_USER_ID_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables finding accounts based on the id of the user in stable memory.
#[derive(Default, Debug)]
pub struct NameToUserIdIndexRepository {}

impl IndexRepository<NameToUserIdIndex, UserId> for NameToUserIdIndexRepository {
    type FindByCriteria = NameToUserIdIndexCriteria;

    fn exists(&self, index: &NameToUserIdIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: NameToUserIdIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &NameToUserIdIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<UserId> {
        DB.with(|db| {
            let name = format_unique_string(&criteria.name);
            let start_key = NameToUserIdIndex {
                name: name.clone(),
                user_id: [u8::MIN; 16],
            };
            let end_key = NameToUserIdIndex {
                name,
                user_id: [u8::MAX; 16],
            };

            db.borrow()
                .range(start_key..=end_key)
                .map(|(index, _)| index.user_id)
                .collect::<HashSet<_>>()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_crud() {
        let repository = NameToUserIdIndexRepository::default();
        let index = NameToUserIdIndex {
            name: "testuser".to_string(),
            user_id: [2; 16],
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_criteria() {
        let repository = NameToUserIdIndexRepository::default();
        repository.insert(NameToUserIdIndex {
            name: "testuser".to_string(),
            user_id: [1; 16],
        });

        repository.insert(NameToUserIdIndex {
            name: "testuser1".to_string(),
            user_id: [2; 16],
        });

        repository.insert(NameToUserIdIndex {
            name: "testuser2".to_string(),
            user_id: [3; 16],
        });

        let result = repository.find_by_criteria(NameToUserIdIndexCriteria {
            name: "Test User".to_string(),
        });

        assert_eq!(result.len(), 1);
        assert!(result.contains(&[1; 16]));

        let result = repository.find_by_criteria(NameToUserIdIndexCriteria {
            name: "Test User 2".to_string(),
        });

        assert_eq!(result.len(), 1);
        assert!(result.contains(&[3; 16]));
    }
}
