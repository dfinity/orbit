use super::indexes::access_control_resource_index::AccessControlPolicyResourceIndexRepository;
use crate::{
    core::{with_memory_manager, Memory, ACCESS_CONTROL_MEMORY_ID},
    models::{
        access_control::{AccessControlPolicy, ResourceSpecifier},
        indexes::access_control_resource_index::AccessControlPolicyResourceIndexCriteria,
    },
};
use ic_canister_core::repository::IndexRepository;
use ic_canister_core::{
    repository::{RefreshIndexMode, Repository},
    types::UUID,
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use std::{cell::RefCell, sync::Arc};

thread_local! {
  static DB: RefCell<StableBTreeMap<UUID, AccessControlPolicy, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(ACCESS_CONTROL_MEMORY_ID))
    )
  })
}

lazy_static! {
    pub static ref ACCESS_CONTROL_REPOSITORY: Arc<AccessControlRepository> =
        Arc::new(AccessControlRepository::default());
}

/// A repository that enables managing access control policies in stable memory.
#[derive(Default, Debug)]
pub struct AccessControlRepository {
    resource_index: AccessControlPolicyResourceIndexRepository,
}

impl Repository<UUID, AccessControlPolicy> for AccessControlRepository {
    fn list(&self) -> Vec<AccessControlPolicy> {
        DB.with(|m| m.borrow().iter().map(|(_, v)| v).collect())
    }

    fn get(&self, key: &UUID) -> Option<AccessControlPolicy> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: UUID, value: AccessControlPolicy) -> Option<AccessControlPolicy> {
        DB.with(|m| {
            let prev = m.borrow_mut().insert(key, value.clone());
            self.resource_index
                .refresh_index_on_modification(RefreshIndexMode::Value {
                    previous: prev.clone().map(|prev| prev.to_index_by_resource()),
                    current: Some(value.to_index_by_resource()),
                });

            prev
        })
    }

    fn remove(&self, key: &UUID) -> Option<AccessControlPolicy> {
        DB.with(|m| {
            let prev = m.borrow_mut().remove(key);
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

impl AccessControlRepository {
    pub fn find_by_resource(&self, resource: &ResourceSpecifier) -> Vec<AccessControlPolicy> {
        self.resource_index
            .find_by_criteria(AccessControlPolicyResourceIndexCriteria {
                resource: resource.to_key(),
            })
            .into_iter()
            .filter_map(|policy_id| self.get(&policy_id))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{
        access_control::{
            access_control_test_utils::mock_access_policy, AddressBookActionSpecifier,
            ResourceSpecifier, ResourceType,
        },
        specifier::CommonSpecifier,
    };

    #[test]
    fn test_crud() {
        let repository = &ACCESS_CONTROL_REPOSITORY;
        let policy = mock_access_policy();

        assert!(repository.get(&policy.id).is_none());

        repository.insert(policy.id.to_owned(), policy.clone());

        assert!(repository.get(&policy.id).is_some());
        assert!(repository.remove(&policy.id).is_some());
        assert!(repository.get(&policy.id).is_none());
    }

    #[test]
    fn test_find_by_resource_and_access() {
        let repository = &ACCESS_CONTROL_REPOSITORY;
        let mut policy = mock_access_policy();
        policy.resource = ResourceSpecifier::Common(
            ResourceType::AddressBook,
            AddressBookActionSpecifier::Read(CommonSpecifier::Any),
        );

        repository.insert(policy.id.to_owned(), policy.clone());

        let policies = repository.find_by_resource(&ResourceSpecifier::Common(
            ResourceType::AddressBook,
            AddressBookActionSpecifier::Read(CommonSpecifier::Any),
        ));

        assert_eq!(policies.len(), 1);
        assert_eq!(policies[0], policy);
    }
}
