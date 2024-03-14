use crate::{
    core::{with_memory_manager, Memory, ACCESS_POLICY_ALLOW_LEVEL_INDEX},
    models::{
        access_policy::{AccessPolicyKey, Resource},
        indexes::access_policy_allow_level_index::{
            AccessPolicyAllowLevelIndex, AccessPolicyAllowLevelIndexCriteria,
        },
    },
};
use ic_canister_core::repository::IndexRepository;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
static DB: RefCell<StableBTreeMap<AccessPolicyAllowLevelIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
  RefCell::new(
    StableBTreeMap::init(memory_manager.get(ACCESS_POLICY_ALLOW_LEVEL_INDEX))
  )
})
}

/// A repository that enables finding access policies based on the allow level in stable memory.
#[derive(Default, Debug)]
pub struct AccessPolicyAllowLevelIndexRepository {}

impl IndexRepository<AccessPolicyAllowLevelIndex, AccessPolicyKey>
    for AccessPolicyAllowLevelIndexRepository
{
    type FindByCriteria = AccessPolicyAllowLevelIndexCriteria;

    fn exists(&self, index: &AccessPolicyAllowLevelIndex) -> bool {
        DB.with(|m| m.borrow().contains_key(index))
    }

    fn insert(&self, index: AccessPolicyAllowLevelIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &AccessPolicyAllowLevelIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<AccessPolicyKey> {
        DB.with(|db| {
            let start_key = AccessPolicyAllowLevelIndex {
                allow_level: criteria.allow_level.clone(),
                access_policy_key: Resource::min(),
            };
            let end_key = AccessPolicyAllowLevelIndex {
                allow_level: criteria.allow_level,
                access_policy_key: Resource::max(),
            };

            db.borrow()
                .range(start_key..=end_key)
                .map(|(index, _)| index.access_policy_key)
                .collect::<HashSet<_>>()
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{
        access_policy::{Resource, ResourceAction},
        indexes::access_policy_allow_level_index::AllowLevel,
    };

    use super::*;

    #[test]
    fn test_repository_crud() {
        let repository = AccessPolicyAllowLevelIndexRepository::default();
        let index = AccessPolicyAllowLevelIndex {
            allow_level: AllowLevel::Any,
            access_policy_key: Resource::AddressBook(ResourceAction::Create),
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_criteria() {
        let repository = AccessPolicyAllowLevelIndexRepository::default();
        repository.insert(AccessPolicyAllowLevelIndex {
            allow_level: AllowLevel::Authenticated,
            access_policy_key: Resource::AddressBook(ResourceAction::Create),
        });

        repository.insert(AccessPolicyAllowLevelIndex {
            allow_level: AllowLevel::Any,
            access_policy_key: Resource::ProposalPolicy(ResourceAction::Create),
        });

        let criteria = AccessPolicyAllowLevelIndexCriteria {
            allow_level: AllowLevel::Any,
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&Resource::ProposalPolicy(ResourceAction::Create)));
    }
}
