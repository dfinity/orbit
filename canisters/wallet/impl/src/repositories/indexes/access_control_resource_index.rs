use crate::{
    core::{with_memory_manager, Memory, ACCESS_CONTROL_RESOURCE_INDEX_MEMORY_ID},
    models::indexes::access_control_resource_index::{
        AccessControlPolicyResourceIndex, AccessControlPolicyResourceIndexCriteria,
    },
};
use ic_canister_core::{repository::IndexRepository, types::UUID};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<AccessControlPolicyResourceIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
  RefCell::new(
    StableBTreeMap::init(memory_manager.get(ACCESS_CONTROL_RESOURCE_INDEX_MEMORY_ID))
  )
  })
}

/// A repository that enables fetching efficiently access policies by their resource and access modifier.
#[derive(Default, Debug)]
pub struct AccessControlPolicyResourceIndexRepository {}

impl IndexRepository<AccessControlPolicyResourceIndex, UUID>
    for AccessControlPolicyResourceIndexRepository
{
    type FindByCriteria = AccessControlPolicyResourceIndexCriteria;

    fn exists(&self, index: &AccessControlPolicyResourceIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: AccessControlPolicyResourceIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &AccessControlPolicyResourceIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<UUID> {
        DB.with(|db| {
            let start_key = AccessControlPolicyResourceIndex {
                resource: criteria.resource.to_owned(),
                access: criteria.access.to_owned(),
                policy_id: [u8::MIN; 16],
            };
            let end_key = AccessControlPolicyResourceIndex {
                resource: criteria.resource.to_owned(),
                access: criteria.access.to_owned(),
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
    use super::*;
    use crate::models::access_control::{AccessModifier, ResourceSpecifier};

    #[test]
    fn test_index_repository() {
        let repository = AccessControlPolicyResourceIndexRepository::default();
        let index = AccessControlPolicyResourceIndex {
            resource: ResourceSpecifier::AddressBook.to_string(),
            access: AccessModifier::Read,
            policy_id: [1; 16],
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_criteria() {
        let repository = AccessControlPolicyResourceIndexRepository::default();
        let generate_items_nr = 10;
        for i in 0..generate_items_nr {
            let index = AccessControlPolicyResourceIndex {
                resource: ResourceSpecifier::AddressBook.to_string(),
                access: match i % 2 {
                    0 => AccessModifier::Read,
                    _ => AccessModifier::Delete,
                },
                policy_id: [i + generate_items_nr; 16],
            };
            repository.insert(index.clone());
        }

        let result = repository.find_by_criteria(AccessControlPolicyResourceIndexCriteria {
            resource: ResourceSpecifier::AddressBook.to_string(),
            access: AccessModifier::Delete,
        });

        assert_eq!(result.len(), 5);
    }
}
