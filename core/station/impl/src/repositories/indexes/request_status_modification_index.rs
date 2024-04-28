use crate::{
    core::{with_memory_manager, Memory, REQUEST_STATUS_MODIFICATION_INDEX_MEMORY_ID},
    models::indexes::request_status_modification_index::{
        RequestStatusModificationIndex, RequestStatusModificationIndexCriteria,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use orbit_essentials::{repository::IndexRepository, types::UUID};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<RequestStatusModificationIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(REQUEST_STATUS_MODIFICATION_INDEX_MEMORY_ID))
    )
  })
}

#[derive(Default, Debug)]
pub struct RequestStatusModificationIndexRepository;

impl IndexRepository<RequestStatusModificationIndex, UUID>
    for RequestStatusModificationIndexRepository
{
    type FindByCriteria = RequestStatusModificationIndexCriteria;

    fn exists(&self, index: &RequestStatusModificationIndex) -> bool {
        DB.with(|m| m.borrow().contains_key(index))
    }

    fn insert(&self, index: RequestStatusModificationIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &RequestStatusModificationIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<UUID> {
        DB.with(|db| {
            let start_key = RequestStatusModificationIndex {
                status: criteria.status.to_owned(),
                modification_timestamp: criteria.from_dt.unwrap_or(u64::MIN),
                request_id: [std::u8::MIN; 16],
            };
            let end_key = RequestStatusModificationIndex {
                status: criteria.status.to_owned(),
                modification_timestamp: criteria.to_dt.unwrap_or(u64::MAX),
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
        let repository = RequestStatusModificationIndexRepository;
        let index = RequestStatusModificationIndex {
            status: RequestStatusCode::Created,
            modification_timestamp: 1,
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
        let repository = RequestStatusModificationIndexRepository;
        let index = RequestStatusModificationIndex {
            status: RequestStatusCode::Created,
            modification_timestamp: 1,
            request_id: [1; 16],
        };

        repository.insert(index.clone());
        repository.insert(RequestStatusModificationIndex {
            status: RequestStatusCode::Created,
            modification_timestamp: 2,
            request_id: [2; 16],
        });

        let criteria = RequestStatusModificationIndexCriteria {
            status: RequestStatusCode::Created,
            from_dt: Some(0),
            to_dt: Some(1),
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&index.request_id));
    }
}
