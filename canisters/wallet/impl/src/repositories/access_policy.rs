use crate::{
    core::{with_memory_manager, Memory, ACCESS_POLICY_MEMORY_ID},
    models::access_policy::{AccessPolicy, AccessPolicyKey, AllowLevel, Resource},
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

impl AccessPolicyRepository {
    pub fn find_by_resource_and_allow_level(
        &self,
        resource: Resource,
        allow: AllowLevel,
    ) -> Option<AccessPolicy> {
        self.get(&AccessPolicyKey {
            resource,
            allow_level: allow,
        })
    }

    pub fn exists_by_resource_and_allow_level(
        &self,
        resource: Resource,
        allow: AllowLevel,
    ) -> bool {
        DB.with(|m| {
            m.borrow().contains_key(&AccessPolicyKey {
                resource,
                allow_level: allow,
            })
        })
    }

    pub fn find_by_resource(&self, resource: Resource) -> Vec<AccessPolicy> {
        DB.with(|db| {
            let start_key = AccessPolicyKey {
                resource: resource.to_owned(),
                allow_level: AllowLevel::Any,
            };
            let end_key = AccessPolicyKey {
                resource,
                allow_level: AllowLevel::UserGroups,
            };

            db.borrow()
                .range(start_key..=end_key)
                .map(|(_, policy)| policy)
                .collect::<Vec<_>>()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::access_policy::{
        access_policy_test_utils::mock_access_policy, ResourceAction, ResourceId,
    };
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

        let policies = ACCESS_POLICY_REPOSITORY
            .find_by_resource(Resource::AddressBook(ResourceAction::Read(ResourceId::Any)));

        assert_eq!(policies.len(), 1);
        assert_eq!(policies[0], policy);
    }
}
