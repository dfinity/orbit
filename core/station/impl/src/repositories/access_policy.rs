use crate::{
    core::{with_memory_manager, Memory, ACCESS_POLICY_MEMORY_ID},
    models::access_policy::{AccessPolicy, AccessPolicyKey},
};
use ic_canister_core::repository::Repository;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use std::{cell::RefCell, sync::Arc};

thread_local! {
  static DB: RefCell<StableBTreeMap<AccessPolicyKey, AccessPolicy, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(ACCESS_POLICY_MEMORY_ID))
    )
  })
}

lazy_static! {
    pub static ref ACCESS_POLICY_REPOSITORY: Arc<AccessPolicyRepository> =
        Arc::new(AccessPolicyRepository::default());
}

/// A repository that enables managing access policies in stable memory.
#[derive(Default, Debug)]
pub struct AccessPolicyRepository {}

impl Repository<AccessPolicyKey, AccessPolicy> for AccessPolicyRepository {
    fn list(&self) -> Vec<AccessPolicy> {
        DB.with(|m| m.borrow().iter().map(|(_, v)| v).collect())
    }

    fn get(&self, key: &AccessPolicyKey) -> Option<AccessPolicy> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: AccessPolicyKey, value: AccessPolicy) -> Option<AccessPolicy> {
        DB.with(|m| m.borrow_mut().insert(key, value.clone()))
    }

    fn remove(&self, key: &AccessPolicyKey) -> Option<AccessPolicy> {
        DB.with(|m| m.borrow_mut().remove(key))
    }

    fn len(&self) -> usize {
        DB.with(|m| m.borrow().len()) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::access_policy::access_policy_test_utils::mock_access_policy;
    use crate::models::resource::{Resource, ResourceAction, ResourceId};
    use ic_canister_core::model::ModelKey;

    #[test]
    fn test_crud() {
        let repository = &ACCESS_POLICY_REPOSITORY;
        let policy = mock_access_policy();

        assert!(repository.get(&policy.key()).is_none());

        repository.insert(policy.key(), policy.clone());

        assert!(repository.get(&policy.key()).is_some());
        assert!(repository.remove(&policy.key()).is_some());
        assert!(repository.get(&policy.key()).is_none());
    }

    #[test]
    fn test_find_by_resource_and_access() {
        for _ in 0..3 {
            let policy = mock_access_policy();
            ACCESS_POLICY_REPOSITORY.insert(policy.key(), policy.clone());
        }

        let mut policy = mock_access_policy();
        policy.resource = Resource::AddressBook(ResourceAction::Read(ResourceId::Any));

        ACCESS_POLICY_REPOSITORY.insert(policy.key(), policy.clone());

        let access_policy = ACCESS_POLICY_REPOSITORY
            .get(&Resource::AddressBook(ResourceAction::Read(
                ResourceId::Any,
            )))
            .unwrap();

        assert_eq!(access_policy, policy);
    }
}
