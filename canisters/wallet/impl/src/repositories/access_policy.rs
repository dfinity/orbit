use super::indexes::access_policy_index::AccessPolicyIndexRepository;
use crate::{
    core::{ic_cdk::api::trap, with_memory_manager, Memory, ACCESS_POLICY_MEMORY_ID},
    models::{
        access_policy::{AccessPolicy, AccessPolicyId, AccessPolicyKey, AllowKey, Resource},
        indexes::access_policy_index::AccessPolicyIndexCriteria,
    },
};
use ic_canister_core::model::ModelKey;
use ic_canister_core::repository::IndexRepository;
use ic_canister_core::repository::{RefreshIndexMode, Repository};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use std::{cell::RefCell, sync::Arc};

thread_local! {
  static DB: RefCell<StableBTreeMap<AccessPolicyId, AccessPolicy, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
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
    resource_index: AccessPolicyIndexRepository,
}

impl Repository<AccessPolicyId, AccessPolicy> for AccessPolicyRepository {
    fn list(&self) -> Vec<AccessPolicy> {
        DB.with(|m| m.borrow().iter().map(|(_, v)| v).collect())
    }

    fn get(&self, id: &AccessPolicyId) -> Option<AccessPolicy> {
        DB.with(|m| m.borrow().get(id))
    }

    fn insert(&self, id: AccessPolicyId, value: AccessPolicy) -> Option<AccessPolicy> {
        let unique_key = value.key();
        let _ = self.find_by_key(&unique_key).map(|previous| {
            if previous.id != value.id {
                trap("An access policy with the same resource and allow already exists");
            }
        });

        DB.with(|m| {
            let prev = m.borrow_mut().insert(id, value.clone());
            self.resource_index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev.clone().map(|prev| prev.to_index_by_resource()),
                    current: Some(value.to_index_by_resource()),
                });

            prev
        })
    }

    fn remove(&self, id: &AccessPolicyId) -> Option<AccessPolicy> {
        DB.with(|m| {
            let prev = m.borrow_mut().remove(id);
            self.resource_index
                .refresh_index_on_modification(RefreshIndexMode::CleanupValue {
                    current: prev.clone().map(|prev| prev.to_index_by_resource()),
                });

            prev
        })
    }

    fn len(&self) -> usize {
        DB.with(|m| m.borrow().len()) as usize
    }
}

impl AccessPolicyRepository {
    pub fn find_by_key(&self, key: &AccessPolicyKey) -> Option<AccessPolicy> {
        self.resource_index
            .find_by_criteria(AccessPolicyIndexCriteria {
                resource: key.resource.clone(),
                allow: Some(key.allow.clone()),
            })
            .into_iter()
            .next()
            .map(|policy_id| self.get(&policy_id))
            .flatten()
    }

    pub fn find_by_resource(&self, resource: &Resource) -> Vec<AccessPolicy> {
        self.resource_index
            .find_by_criteria(AccessPolicyIndexCriteria {
                resource: resource.to_owned(),
                allow: None,
            })
            .into_iter()
            .filter_map(|policy_id| self.get(&policy_id))
            .collect()
    }

    pub fn find_by_resource_and_allowed_type(
        &self,
        resource: &Resource,
        allow: &AllowKey,
    ) -> Option<AccessPolicy> {
        self.resource_index
            .find_by_criteria(AccessPolicyIndexCriteria {
                resource: resource.to_owned(),
                allow: Some(allow.clone()),
            })
            .into_iter()
            .next()
            .map(|policy_id| self.get(&policy_id))
            .flatten()
    }

    pub fn exists_by_resource_and_allowed_type(
        &self,
        resource: &Resource,
        allow: &AllowKey,
    ) -> bool {
        self.resource_index
            .find_by_criteria(AccessPolicyIndexCriteria {
                resource: resource.to_owned(),
                allow: Some(allow.clone()),
            })
            .into_iter()
            .next()
            .is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::access_policy::{
        access_policy_test_utils::mock_access_policy, ResourceAction, ResourceId,
    };

    #[test]
    fn test_crud() {
        let repository = &ACCESS_POLICY_REPOSITORY;
        let policy = mock_access_policy();

        assert!(repository.get(&policy.id).is_none());

        repository.insert(policy.id, policy.clone());

        assert!(repository.get(&policy.id).is_some());
        assert!(repository.remove(&policy.id).is_some());
        assert!(repository.get(&policy.id).is_none());
    }

    #[test]
    fn test_find_by_resource_and_access() {
        for _ in 0..3 {
            let policy = mock_access_policy();
            ACCESS_POLICY_REPOSITORY.insert(policy.id, policy.clone());
        }

        let mut policy = mock_access_policy();
        policy.resource = Resource::AddressBook(ResourceAction::Read(ResourceId::Any));

        ACCESS_POLICY_REPOSITORY.insert(policy.id, policy.clone());

        let policies = ACCESS_POLICY_REPOSITORY.find_by_resource(&Resource::AddressBook(
            ResourceAction::Read(ResourceId::Any),
        ));

        assert_eq!(policies.len(), 1);
        assert_eq!(policies[0], policy);
    }
}
