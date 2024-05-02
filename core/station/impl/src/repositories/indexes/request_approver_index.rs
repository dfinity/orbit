use crate::{
    core::{with_memory_manager, Memory, REQUEST_APPROVER_INDEX_MEMORY_ID},
    models::{
        indexes::request_approver_index::{RequestApproverIndex, RequestApproverIndexCriteria},
        RequestId,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use orbit_essentials::repository::IndexRepository;
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<RequestApproverIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(REQUEST_APPROVER_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables finding requests based on the approver in stable memory.
#[derive(Default, Debug)]
pub struct RequestApproverIndexRepository {}

impl IndexRepository<RequestApproverIndex, RequestId> for RequestApproverIndexRepository {
    type FindByCriteria = RequestApproverIndexCriteria;

    fn exists(&self, index: &RequestApproverIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: RequestApproverIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &RequestApproverIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<RequestId> {
        DB.with(|db| {
            let start_key = RequestApproverIndex {
                approver_id: criteria.approver_id.to_owned(),
                request_id: [u8::MIN; 16],
            };
            let end_key = RequestApproverIndex {
                approver_id: criteria.approver_id.to_owned(),
                request_id: [u8::MAX; 16],
            };

            db.borrow()
                .range(start_key..=end_key)
                .map(|(index, _)| index.request_id)
                .collect::<HashSet<RequestId>>()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_crud() {
        let repository = RequestApproverIndexRepository::default();
        let index = RequestApproverIndex {
            request_id: [0; 16],
            approver_id: [1; 16],
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_criteria() {
        let repository = RequestApproverIndexRepository::default();
        let index = RequestApproverIndex {
            request_id: [0; 16],
            approver_id: [1; 16],
        };

        repository.insert(index.clone());

        let criteria = RequestApproverIndexCriteria {
            approver_id: [1; 16],
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&index.request_id));
    }
}
