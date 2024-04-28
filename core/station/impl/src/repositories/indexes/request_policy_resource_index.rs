use crate::{
    core::{with_memory_manager, Memory, POLICY_RESOURCE_INDEX_MEMORY_ID},
    models::indexes::request_policy_resource_index::{
        RequestPolicyResourceIndex, RequestPolicyResourceIndexCriteria,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use orbit_essentials::{repository::IndexRepository, types::UUID};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<RequestPolicyResourceIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(POLICY_RESOURCE_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables finding requests based on the requester in stable memory.
#[derive(Default, Debug)]
pub struct RequestPolicyResourceIndexRepository {}

#[cfg(test)]
impl RequestPolicyResourceIndexRepository {
    pub fn len(&self) -> usize {
        DB.with(|m| m.borrow().len() as usize)
    }

    pub fn is_empty(&self) -> bool {
        DB.with(|m| m.borrow().is_empty())
    }

    pub fn list(&self) -> Vec<RequestPolicyResourceIndex> {
        DB.with(|m| m.borrow().iter().map(|(k, _)| k.clone()).collect())
    }
}

impl IndexRepository<RequestPolicyResourceIndex, UUID> for RequestPolicyResourceIndexRepository {
    type FindByCriteria = RequestPolicyResourceIndexCriteria;

    fn exists(&self, index: &RequestPolicyResourceIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: RequestPolicyResourceIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &RequestPolicyResourceIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<UUID> {
        DB.with(|db| {
            let start_key = RequestPolicyResourceIndex {
                resource: criteria.resource.clone(),
                policy_id: [u8::MIN; 16],
            };
            let end_key = RequestPolicyResourceIndex {
                resource: criteria.resource,
                policy_id: [u8::MAX; 16],
            };

            db.borrow()
                .range(start_key..=end_key)
                .map(|(index, _)| index.policy_id)
                .collect::<HashSet<UUID>>()
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::models::resource::{Resource, UserResourceAction};

    use super::*;

    #[test]
    fn test_repository_crud() {
        let repository = RequestPolicyResourceIndexRepository::default();
        let index = RequestPolicyResourceIndex {
            policy_id: [0; 16],
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
        let repository = RequestPolicyResourceIndexRepository::default();
        let index = RequestPolicyResourceIndex {
            resource: Resource::User(UserResourceAction::Create),
            policy_id: [0; 16],
        };

        repository.insert(index.clone());

        let criteria = RequestPolicyResourceIndexCriteria {
            resource: Resource::User(UserResourceAction::Create),
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&index.policy_id));
    }
}
