use crate::{
    core::{with_memory_manager, Memory, ACCESS_POLICY_MEMORY_ID},
    models::{
        access_policy::{AccessPolicy, AccessPolicyKey, Resource},
        indexes::access_policy_allow_level_index::{AccessPolicyAllowLevelIndex, AllowLevel},
    },
};
use ic_canister_core::repository::{IndexRepository, RefreshIndexMode, Repository};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use std::{cell::RefCell, sync::Arc};

use super::indexes::access_policy_allow_level_index::AccessPolicyAllowLevelIndexRepository;

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
pub struct AccessPolicyRepository {
    allow_level_index: AccessPolicyAllowLevelIndexRepository,
}

impl Repository<AccessPolicyKey, AccessPolicy> for AccessPolicyRepository {
    fn list(&self) -> Vec<AccessPolicy> {
        DB.with(|m| m.borrow().iter().map(|(_, v)| v).collect())
    }

    fn get(&self, key: &AccessPolicyKey) -> Option<AccessPolicy> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: AccessPolicyKey, value: AccessPolicy) -> Option<AccessPolicy> {
        DB.with(|m| {
            let prev = m.borrow_mut().insert(key, value.clone());
            self.allow_level_index
                .refresh_index_on_modification(RefreshIndexMode::List {
                    previous: prev
                        .clone()
                        .map_or(Vec::new(), |prev| prev.to_index_by_allow_levels()),
                    current: value.to_index_by_allow_levels(),
                });

            prev
        })
    }

    fn remove(&self, key: &AccessPolicyKey) -> Option<AccessPolicy> {
        DB.with(|m| {
            let prev = m.borrow_mut().remove(key);
            self.allow_level_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupList {
                    current: prev
                        .clone()
                        .map_or(Vec::new(), |prev| prev.to_index_by_allow_levels()),
                });

            prev
        })
    }

    fn len(&self) -> usize {
        DB.with(|m| m.borrow().len()) as usize
    }
}

impl AccessPolicyRepository {
    pub fn exists_by_resource_and_allow_level(
        &self,
        resource: Resource,
        allow: AllowLevel,
    ) -> bool {
        self.allow_level_index.exists(&AccessPolicyAllowLevelIndex {
            allow_level: allow,
            access_policy_key: resource,
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

        let access_policy = ACCESS_POLICY_REPOSITORY
            .get(&Resource::AddressBook(ResourceAction::Read(
                ResourceId::Any,
            )))
            .unwrap();

        assert_eq!(access_policy, policy);
    }
}
