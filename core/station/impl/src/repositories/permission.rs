use crate::{
    core::{with_memory_manager, Memory, PERMISSION_MEMORY_ID},
    models::permission::{Permission, PermissionKey},
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use orbit_essentials::repository::Repository;
use std::{cell::RefCell, sync::Arc};

thread_local! {
  static DB: RefCell<StableBTreeMap<PermissionKey, Permission, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(PERMISSION_MEMORY_ID))
    )
  })
}

lazy_static! {
    pub static ref PERMISSION_REPOSITORY: Arc<PermissionRepository> =
        Arc::new(PermissionRepository::default());
}

/// A repository that enables managing permissions in stable memory.
#[derive(Default, Debug)]
pub struct PermissionRepository {}

impl Repository<PermissionKey, Permission> for PermissionRepository {
    fn list(&self) -> Vec<Permission> {
        DB.with(|m| m.borrow().iter().map(|(_, v)| v).collect())
    }

    fn get(&self, key: &PermissionKey) -> Option<Permission> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: PermissionKey, value: Permission) -> Option<Permission> {
        DB.with(|m| m.borrow_mut().insert(key, value.clone()))
    }

    fn remove(&self, key: &PermissionKey) -> Option<Permission> {
        DB.with(|m| m.borrow_mut().remove(key))
    }

    fn len(&self) -> usize {
        DB.with(|m| m.borrow().len()) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::permission::permission_test_utils::mock_permission;
    use crate::models::resource::{Resource, ResourceAction, ResourceId};
    use orbit_essentials::model::ModelKey;

    #[test]
    fn test_crud() {
        let repository = &PERMISSION_REPOSITORY;
        let policy = mock_permission();

        assert!(repository.get(&policy.key()).is_none());

        repository.insert(policy.key(), policy.clone());

        assert!(repository.get(&policy.key()).is_some());
        assert!(repository.remove(&policy.key()).is_some());
        assert!(repository.get(&policy.key()).is_none());
    }

    #[test]
    fn test_find_by_resource_and_access() {
        for _ in 0..3 {
            let policy = mock_permission();
            PERMISSION_REPOSITORY.insert(policy.key(), policy.clone());
        }

        let mut policy = mock_permission();
        policy.resource = Resource::AddressBook(ResourceAction::Read(ResourceId::Any));

        PERMISSION_REPOSITORY.insert(policy.key(), policy.clone());

        let permission = PERMISSION_REPOSITORY
            .get(&Resource::AddressBook(ResourceAction::Read(
                ResourceId::Any,
            )))
            .unwrap();

        assert_eq!(permission, policy);
    }
}
