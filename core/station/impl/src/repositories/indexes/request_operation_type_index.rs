use crate::{
    core::{with_memory_manager, Memory, OPERATION_TYPE_TO_REQUEST_ID_INDEX_MEMORY_ID},
    models::{
        indexes::request_operation_type_index::{
            RequestOperationTypeIndex, RequestOperationTypeIndexCriteria,
        },
        RequestId,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use orbit_essentials::repository::IndexRepository;
use std::{cell::RefCell, collections::HashSet};

thread_local! {
static DB: RefCell<StableBTreeMap<RequestOperationTypeIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
  RefCell::new(
    StableBTreeMap::init(memory_manager.get(OPERATION_TYPE_TO_REQUEST_ID_INDEX_MEMORY_ID))
  )
})
}

/// A repository that enables finding requests based on the operation type.
#[derive(Default, Debug)]
pub struct RequestOperationTypeIndexRepository {}

impl IndexRepository<RequestOperationTypeIndex, RequestId> for RequestOperationTypeIndexRepository {
    type FindByCriteria = RequestOperationTypeIndexCriteria;

    fn exists(&self, index: &RequestOperationTypeIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: RequestOperationTypeIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &RequestOperationTypeIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<RequestId> {
        DB.with(|db| {
            let start_key = RequestOperationTypeIndex {
                operation_type: criteria.operation_type.clone(),
                request_id: [u8::MIN; 16],
            };
            let end_key = RequestOperationTypeIndex {
                operation_type: criteria.operation_type,
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
    use crate::models::request_operation_filter_type::RequestOperationFilterType;

    #[test]
    fn test_repository_crud() {
        let repository = RequestOperationTypeIndexRepository::default();
        let index = RequestOperationTypeIndex {
            operation_type: RequestOperationFilterType::AddAccount,
            request_id: [0; 16],
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_criteria() {
        let repository = RequestOperationTypeIndexRepository::default();
        let index = RequestOperationTypeIndex {
            request_id: [0; 16],
            operation_type: RequestOperationFilterType::Transfer(None),
        };

        repository.insert(index.clone());

        let index = RequestOperationTypeIndex {
            request_id: [1; 16],
            operation_type: RequestOperationFilterType::Transfer(Some([0; 16])),
        };

        repository.insert(index.clone());

        let result = repository.find_by_criteria(RequestOperationTypeIndexCriteria {
            operation_type: RequestOperationFilterType::AddAccount,
        });

        assert!(result.is_empty());

        let result = repository.find_by_criteria(RequestOperationTypeIndexCriteria {
            operation_type: RequestOperationFilterType::Transfer(None),
        });

        assert_eq!(result.len(), 1);
        assert!(result.contains(&[0; 16]));

        let result = repository.find_by_criteria(RequestOperationTypeIndexCriteria {
            operation_type: RequestOperationFilterType::Transfer(Some([0; 16])),
        });

        assert_eq!(result.len(), 1);
    }
}
