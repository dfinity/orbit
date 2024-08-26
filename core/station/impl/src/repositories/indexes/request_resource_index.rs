use crate::{
    core::{with_memory_manager, Memory, REQUEST_RESOURCE_INDEX_MEMORY_ID},
    models::{
        indexes::request_resource_index::{RequestResourceIndex, RequestResourceIndexCriteria},
        resource::Resource,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use orbit_essentials::repository::IndexRepository;
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<RequestResourceIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(REQUEST_RESOURCE_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables finding requests based on the requester in stable memory.
#[derive(Default, Debug)]
pub struct RequestResourceIndexRepository {}

impl RequestResourceIndexRepository {
    /// Clears the repository by removing all the entries.
    pub fn clear(&self) {
        DB.with(|m| m.borrow_mut().clear_new());
    }
}

impl IndexRepository<RequestResourceIndex, Resource> for RequestResourceIndexRepository {
    type FindByCriteria = RequestResourceIndexCriteria;

    fn exists(&self, index: &RequestResourceIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: RequestResourceIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &RequestResourceIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<Resource> {
        DB.with(|db| {
            let start_key = RequestResourceIndex {
                request_id: criteria.request_id,
                resource: Resource::min(),
            };
            let end_key = RequestResourceIndex {
                request_id: criteria.request_id,
                resource: Resource::max(),
            };

            db.borrow()
                .range(start_key..=end_key)
                .map(|(index, _)| index.resource)
                .collect::<HashSet<Resource>>()
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::models::resource::UserResourceAction;

    use super::*;

    #[test]
    fn test_repository_crud() {
        let repository = RequestResourceIndexRepository::default();
        let index = RequestResourceIndex {
            request_id: [0; 16],
            resource: Resource::User(UserResourceAction::Create),
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_criteria() {
        let repository = RequestResourceIndexRepository::default();
        let index = RequestResourceIndex {
            request_id: [0; 16],
            resource: Resource::User(UserResourceAction::Create),
        };

        repository.insert(index.clone());

        let criteria = RequestResourceIndexCriteria {
            request_id: [0; 16],
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&Resource::User(UserResourceAction::Create)));
    }
}
