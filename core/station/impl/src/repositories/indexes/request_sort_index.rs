use crate::{
    core::{with_memory_manager, Memory, REQUEST_SORT_INDEX_MEMORY_ID},
    models::indexes::request_sort_index::{
        RequestSortIndex, RequestSortIndexCriteria, RequestSortIndexKey, RequestSortIndexValue,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use orbit_essentials::repository::IndexRepository;
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<RequestSortIndexKey, RequestSortIndexValue, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(REQUEST_SORT_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables finding requests based on the voter in stable memory.
#[derive(Default, Debug)]
pub struct RequestSortIndexRepository {}

impl IndexRepository<RequestSortIndex, RequestSortIndexValue> for RequestSortIndexRepository {
    type FindByCriteria = RequestSortIndexCriteria;

    fn exists(&self, index: &RequestSortIndex) -> bool {
        DB.with(|m| m.borrow().contains_key(&index.key))
    }

    fn insert(&self, index: RequestSortIndex) {
        DB.with(|m| m.borrow_mut().insert(index.key, index.value));
    }

    fn remove(&self, index: &RequestSortIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(&index.key).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<RequestSortIndexValue> {
        let value = self.get(&RequestSortIndexKey {
            request_id: criteria.request_id,
        });

        match value {
            Some(value) => {
                let mut set = HashSet::new();
                set.insert(value);
                set
            }
            None => HashSet::new(),
        }
    }
}

impl RequestSortIndexRepository {
    pub fn get(&self, key: &RequestSortIndexKey) -> Option<RequestSortIndexValue> {
        DB.with(|m| m.borrow().get(key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_crud() {
        let repository = RequestSortIndexRepository::default();
        let index = RequestSortIndex {
            key: RequestSortIndexKey {
                request_id: [0; 16],
            },
            value: RequestSortIndexValue {
                creation_timestamp: 1,
                modification_timestamp: 2,
                expiration_timestamp: 3,
            },
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_criteria() {
        let repository = RequestSortIndexRepository::default();
        let index = RequestSortIndex {
            key: RequestSortIndexKey {
                request_id: [0; 16],
            },
            value: RequestSortIndexValue {
                creation_timestamp: 1,
                modification_timestamp: 2,
                expiration_timestamp: 3,
            },
        };

        repository.insert(index.clone());

        let criteria = RequestSortIndexCriteria {
            request_id: [0; 16],
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 1);

        let value = result.into_iter().next().unwrap();

        assert_eq!(value.creation_timestamp, index.value.creation_timestamp);
        assert_eq!(
            value.modification_timestamp,
            index.value.modification_timestamp
        );
        assert_eq!(value.expiration_timestamp, index.value.expiration_timestamp);
    }
}
