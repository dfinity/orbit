use crate::{
    core::{with_memory_manager, Memory, ACCESS_CONTROL_MEMORY_ID},
    models::access_control::AccessControlPolicy,
};
use ic_canister_core::{repository::Repository, types::UUID};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use std::cell::RefCell;

thread_local! {
  static DB: RefCell<StableBTreeMap<UUID, AccessControlPolicy, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(ACCESS_CONTROL_MEMORY_ID))
    )
  })
}

lazy_static! {
    pub static ref ACCESS_CONTROL_REPOSITORY: AccessControlRepository =
        AccessControlRepository::default();
}

/// A repository that enables managing access control policies in stable memory.
#[derive(Default, Debug)]
pub struct AccessControlRepository {}

impl Repository<UUID, AccessControlPolicy> for AccessControlRepository {
    fn list(&self) -> Vec<AccessControlPolicy> {
        DB.with(|m| m.borrow().iter().map(|(_, v)| v).collect())
    }

    fn get(&self, key: &UUID) -> Option<AccessControlPolicy> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(&self, key: UUID, value: AccessControlPolicy) -> Option<AccessControlPolicy> {
        DB.with(|m| m.borrow_mut().insert(key, value.clone()))
    }

    fn remove(&self, key: &UUID) -> Option<AccessControlPolicy> {
        DB.with(|m| m.borrow_mut().remove(key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::access_control::access_control_test_utils::mock_access_policy;

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
}
