use crate::{
    core::{with_memory_manager, Memory, REQUEST_REQUESTER_INDEX_MEMORY_ID},
    models::indexes::request_requester_index::{
        RequestRequesterIndex, RequestRequesterIndexCriteria,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use orbit_essentials::{repository::IndexRepository, types::UUID};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<RequestRequesterIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(REQUEST_REQUESTER_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables finding requests based on the requester in stable memory.
#[derive(Default, Debug)]
pub struct RequestRequesterIndexRepository {}

impl IndexRepository<RequestRequesterIndex, UUID> for RequestRequesterIndexRepository {
    type FindByCriteria = RequestRequesterIndexCriteria;

    fn exists(&self, index: &RequestRequesterIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: RequestRequesterIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &RequestRequesterIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<UUID> {
        DB.with(|db| {
            let start_key = RequestRequesterIndex {
                requester_id: criteria.requester_id.to_owned(),
                request_id: [u8::MIN; 16],
            };
            let end_key = RequestRequesterIndex {
                requester_id: criteria.requester_id.to_owned(),
                request_id: [u8::MAX; 16],
            };

            db.borrow()
                .range(start_key..=end_key)
                .map(|(index, _)| index.request_id)
                .collect::<HashSet<UUID>>()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_crud() {
        let repository = RequestRequesterIndexRepository::default();
        let index = RequestRequesterIndex {
            request_id: [0; 16],
            requester_id: [1; 16],
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_criteria() {
        let repository = RequestRequesterIndexRepository::default();
        let index = RequestRequesterIndex {
            request_id: [0; 16],
            requester_id: [1; 16],
        };

        repository.insert(index.clone());

        let criteria = RequestRequesterIndexCriteria {
            requester_id: [1; 16],
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&index.request_id));
    }
}
