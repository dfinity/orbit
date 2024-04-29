use crate::{
    core::{with_memory_manager, Memory, REQUEST_KEY_CREATION_TIME_INDEX_MEMORY_ID},
    models::indexes::request_key_creation_time_index::{
        RequestKeyCreationTimeIndex, RequestKeyCreationTimeIndexCriteria,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use orbit_essentials::{repository::IndexRepository, types::UUID};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<RequestKeyCreationTimeIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(REQUEST_KEY_CREATION_TIME_INDEX_MEMORY_ID))
    )
  })
}

#[derive(Default, Debug)]
pub struct RequestKeyCreationTimeIndexRepository {}

impl IndexRepository<RequestKeyCreationTimeIndex, UUID> for RequestKeyCreationTimeIndexRepository {
    type FindByCriteria = RequestKeyCreationTimeIndexCriteria;

    fn exists(&self, index: &RequestKeyCreationTimeIndex) -> bool {
        DB.with(|m| m.borrow().contains_key(index))
    }

    fn insert(&self, index: RequestKeyCreationTimeIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &RequestKeyCreationTimeIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<UUID> {
        DB.with(|db| {
            let start_key = RequestKeyCreationTimeIndex {
                request_id: criteria.request_id.to_owned(),
                created_at: criteria.from_dt.to_owned().unwrap_or(u64::MIN),
            };
            let end_key = RequestKeyCreationTimeIndex {
                request_id: criteria.request_id,
                created_at: criteria.to_dt.to_owned().unwrap_or(u64::MAX),
            };

            db.borrow()
                .range(start_key..=end_key)
                .map(|(index, _)| index.request_id)
                .collect::<HashSet<UUID>>()
        })
    }
}

impl RequestKeyCreationTimeIndexRepository {
    pub fn exists_by_criteria(&self, criteria: RequestKeyCreationTimeIndexCriteria) -> bool {
        let start_key = RequestKeyCreationTimeIndex {
            request_id: criteria.request_id.to_owned(),
            created_at: criteria.from_dt.to_owned().unwrap_or(u64::MIN),
        };
        let end_key = RequestKeyCreationTimeIndex {
            request_id: criteria.request_id,
            created_at: criteria.to_dt.to_owned().unwrap_or(u64::MAX),
        };

        DB.with(|db| db.borrow().range(start_key..=end_key).next().is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_crud() {
        let repository = RequestKeyCreationTimeIndexRepository::default();
        let index = RequestKeyCreationTimeIndex {
            created_at: 10,
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
        let repository = RequestKeyCreationTimeIndexRepository::default();
        let index = RequestKeyCreationTimeIndex {
            created_at: 10,
            request_id: [1; 16],
        };

        repository.insert(index.clone());
        repository.insert(RequestKeyCreationTimeIndex {
            created_at: 11,
            request_id: [2; 16],
        });

        let criteria = RequestKeyCreationTimeIndexCriteria {
            request_id: [1; 16],
            from_dt: None,
            to_dt: Some(10),
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&index.request_id));
    }
}
