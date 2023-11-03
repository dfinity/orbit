use crate::{
    core::{with_memory_manager, Memory, OPERATION_USER_INDEX_MEMORY_ID},
    models::{
        indexes::operation_user_index::{OperationUserIndex, OperationUserIndexCriteria},
        OperationId,
    },
};
use ic_canister_core::repository::IndexRepository;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<OperationUserIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(OPERATION_USER_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables finding operations based on the user in stable memory.
#[derive(Default, Debug)]
pub struct OperationUserIndexRepository {}

impl IndexRepository<OperationUserIndex, OperationId> for OperationUserIndexRepository {
    type FindByCriteria = OperationUserIndexCriteria;

    fn exists(&self, index: &OperationUserIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: OperationUserIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &OperationUserIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<OperationId> {
        DB.with(|db| {
            let start_key = OperationUserIndex {
                user_id: criteria.user_id.to_owned(),
                created_at: criteria.from_dt.to_owned().unwrap_or(u64::MIN),
                id: [u8::MIN; 16],
            };
            let end_key = OperationUserIndex {
                user_id: criteria.user_id.to_owned(),
                created_at: criteria.to_dt.to_owned().unwrap_or(u64::MAX),
                id: [u8::MAX; 16],
            };

            db.borrow()
                .range(start_key..=end_key)
                .map(|(index, _)| index.id)
                .collect::<HashSet<OperationId>>()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_crud() {
        let repository = OperationUserIndexRepository::default();
        let index = OperationUserIndex {
            id: [0; 16],
            created_at: 10,
            user_id: [1; 16],
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_criteria() {
        let repository = OperationUserIndexRepository::default();
        let index = OperationUserIndex {
            id: [0; 16],
            created_at: 10,
            user_id: [1; 16],
        };

        repository.insert(index.clone());

        let criteria = OperationUserIndexCriteria {
            user_id: [1; 16],
            from_dt: None,
            to_dt: None,
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&index.id));
    }
}
