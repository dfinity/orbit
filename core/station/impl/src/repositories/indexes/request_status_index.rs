use crate::{
    core::{with_memory_manager, Memory, REQUEST_STATUS_INDEX_MEMORY_ID},
    models::indexes::request_status_index::{RequestStatusIndex, RequestStatusIndexCriteria},
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use orbit_essentials::{repository::IndexRepository, types::UUID};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<RequestStatusIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(REQUEST_STATUS_INDEX_MEMORY_ID))
    )
  })
}

#[derive(Default, Debug)]
pub struct RequestStatusIndexRepository {}

impl IndexRepository<RequestStatusIndex, UUID> for RequestStatusIndexRepository {
    type FindByCriteria = RequestStatusIndexCriteria;

    fn exists(&self, index: &RequestStatusIndex) -> bool {
        DB.with(|m| m.borrow().contains_key(index))
    }

    fn insert(&self, index: RequestStatusIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &RequestStatusIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<UUID> {
        DB.with(|db| {
            let start_key = RequestStatusIndex {
                status: criteria.status.to_owned(),
                request_id: [std::u8::MIN; 16],
            };
            let end_key = RequestStatusIndex {
                status: criteria.status.to_owned(),
                request_id: [std::u8::MAX; 16],
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
    use crate::models::RequestStatusCode;

    #[test]
    fn test_repository_crud() {
        let repository = RequestStatusIndexRepository::default();
        let index = RequestStatusIndex {
            status: RequestStatusCode::Created,
            request_id: [1; 16],
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_criteria() {
        let repository = RequestStatusIndexRepository::default();
        let index = RequestStatusIndex {
            status: RequestStatusCode::Approved,
            request_id: [1; 16],
        };

        repository.insert(index.clone());
        repository.insert(RequestStatusIndex {
            status: RequestStatusCode::Created,
            request_id: [2; 16],
        });

        let criteria = RequestStatusIndexCriteria {
            status: RequestStatusCode::Created,
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&[2; 16]));
    }
}
