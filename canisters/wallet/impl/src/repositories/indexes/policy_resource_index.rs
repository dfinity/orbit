use crate::{
    core::{with_memory_manager, Memory, POLICY_RESOURCE_INDEX_MEMORY_ID},
    models::indexes::policy_resource_index::{PolicyResourceIndex, PolicyResourceIndexCriteria},
};
use ic_canister_core::{repository::IndexRepository, types::UUID};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<PolicyResourceIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(POLICY_RESOURCE_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables finding proposals based on the proposer in stable memory.
#[derive(Default, Debug)]
pub struct PolicyResourceIndexRepository {}

#[cfg(test)]
impl PolicyResourceIndexRepository {
    pub fn len(&self) -> usize {
        DB.with(|m| m.borrow().len() as usize)
    }

    pub fn is_empty(&self) -> bool {
        DB.with(|m| m.borrow().is_empty())
    }

    pub fn list(&self) -> Vec<PolicyResourceIndex> {
        DB.with(|m| m.borrow().iter().map(|(k, _)| k.clone()).collect())
    }
}

impl IndexRepository<PolicyResourceIndex, UUID> for PolicyResourceIndexRepository {
    type FindByCriteria = PolicyResourceIndexCriteria;

    fn exists(&self, index: &PolicyResourceIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: PolicyResourceIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &PolicyResourceIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<UUID> {
        DB.with(|db| {
            let start_key = PolicyResourceIndex {
                resource: criteria.resource.clone(),
                policy_id: [u8::MIN; 16],
            };
            let end_key = PolicyResourceIndex {
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
        let repository = PolicyResourceIndexRepository::default();
        let index = PolicyResourceIndex {
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
        let repository = PolicyResourceIndexRepository::default();
        let index = PolicyResourceIndex {
            resource: Resource::User(UserResourceAction::Create),
            policy_id: [0; 16],
        };

        repository.insert(index.clone());

        let criteria = PolicyResourceIndexCriteria {
            resource: Resource::User(UserResourceAction::Create),
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&index.policy_id));
    }
}
