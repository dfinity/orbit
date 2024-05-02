use crate::{
    core::{with_memory_manager, Memory, REQUEST_KEY_EXPIRATION_TIME_INDEX_MEMORY_ID},
    models::indexes::request_key_expiration_time_index::{
        RequestKeyExpirationTimeIndex, RequestKeyExpirationTimeIndexCriteria,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use orbit_essentials::{repository::IndexRepository, types::UUID};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<RequestKeyExpirationTimeIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(REQUEST_KEY_EXPIRATION_TIME_INDEX_MEMORY_ID))
    )
  })
}

#[derive(Default, Debug)]
pub struct RequestKeyExpirationTimeIndexRepository {}

impl IndexRepository<RequestKeyExpirationTimeIndex, UUID>
    for RequestKeyExpirationTimeIndexRepository
{
    type FindByCriteria = RequestKeyExpirationTimeIndexCriteria;

    fn exists(&self, index: &RequestKeyExpirationTimeIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: RequestKeyExpirationTimeIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &RequestKeyExpirationTimeIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<UUID> {
        DB.with(|db| {
            let start_key = RequestKeyExpirationTimeIndex {
                request_id: criteria.request_id.to_owned(),
                expiration_dt: criteria.from_dt.to_owned().unwrap_or(u64::MIN),
            };
            let end_key = RequestKeyExpirationTimeIndex {
                request_id: criteria.request_id,
                expiration_dt: criteria.to_dt.to_owned().unwrap_or(u64::MAX),
            };

            db.borrow()
                .range(start_key..=end_key)
                .map(|(index, _)| index.request_id)
                .collect::<HashSet<UUID>>()
        })
    }
}

impl RequestKeyExpirationTimeIndexRepository {
    pub fn exists_by_criteria(&self, criteria: RequestKeyExpirationTimeIndexCriteria) -> bool {
        let start_key = RequestKeyExpirationTimeIndex {
            request_id: criteria.request_id.to_owned(),
            expiration_dt: criteria.from_dt.to_owned().unwrap_or(u64::MIN),
        };
        let end_key = RequestKeyExpirationTimeIndex {
            request_id: criteria.request_id,
            expiration_dt: criteria.to_dt.to_owned().unwrap_or(u64::MAX),
        };

        DB.with(|db| db.borrow().range(start_key..=end_key).next().is_some())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_crud() {
        let repository = RequestKeyExpirationTimeIndexRepository::default();
        let index = RequestKeyExpirationTimeIndex {
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
        let repository = RequestKeyExpirationTimeIndexRepository::default();
        let index = RequestKeyExpirationTimeIndex {
            expiration_dt: 10,
            request_id: [1; 16],
        };

        repository.insert(index.clone());
        repository.insert(RequestKeyExpirationTimeIndex {
            expiration_dt: 11,
            request_id: [2; 16],
        });

        let criteria = RequestKeyExpirationTimeIndexCriteria {
            request_id: [1; 16],
            from_dt: None,
            to_dt: Some(10),
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&index.request_id));
    }
}
