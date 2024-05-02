use crate::{
    core::{with_memory_manager, Memory, REQUEST_EXPIRATION_TIME_INDEX_MEMORY_ID},
    models::indexes::request_expiration_time_index::{
        RequestExpirationTimeIndex, RequestExpirationTimeIndexCriteria,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use orbit_essentials::{repository::IndexRepository, types::UUID};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<RequestExpirationTimeIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(REQUEST_EXPIRATION_TIME_INDEX_MEMORY_ID))
    )
  })
}

#[derive(Default, Debug)]
pub struct RequestExpirationTimeIndexRepository {}

impl IndexRepository<RequestExpirationTimeIndex, UUID> for RequestExpirationTimeIndexRepository {
    type FindByCriteria = RequestExpirationTimeIndexCriteria;

    fn exists(&self, index: &RequestExpirationTimeIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: RequestExpirationTimeIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &RequestExpirationTimeIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<UUID> {
        DB.with(|db| {
            let start_key = RequestExpirationTimeIndex {
                expiration_dt: criteria.from_dt.to_owned().unwrap_or(u64::MIN),
                request_id: [std::u8::MIN; 16],
            };
            let end_key = RequestExpirationTimeIndex {
                expiration_dt: criteria.to_dt.to_owned().unwrap_or(u64::MAX),
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

    #[test]
    fn test_repository_crud() {
        let repository = RequestExpirationTimeIndexRepository::default();
        let index = RequestExpirationTimeIndex {
            expiration_dt: 10,
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
        let repository = RequestExpirationTimeIndexRepository::default();
        let index = RequestExpirationTimeIndex {
            expiration_dt: 10,
            request_id: [1; 16],
        };

        repository.insert(index.clone());
        repository.insert(RequestExpirationTimeIndex {
            expiration_dt: 11,
            request_id: [2; 16],
        });

        let criteria = RequestExpirationTimeIndexCriteria {
            from_dt: None,
            to_dt: Some(10),
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&index.request_id));
    }
}
