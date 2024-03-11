use crate::{
    core::{with_memory_manager, Memory, ACCESS_POLICY_INDEX_MEMORY_ID},
    models::{access_policy::AllowKey, indexes::access_policy_index::{AccessPolicyIndex, AccessPolicyIndexCriteria}},
};
use ic_canister_core::{repository::IndexRepository, types::UUID};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<AccessPolicyIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
        StableBTreeMap::init(memory_manager.get(ACCESS_POLICY_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables fetching efficiently access policies by their resource and allowed users.
#[derive(Default, Debug)]
pub struct AccessPolicyIndexRepository {}

impl IndexRepository<AccessPolicyIndex, UUID> for AccessPolicyIndexRepository {
    type FindByCriteria = AccessPolicyIndexCriteria;

    fn exists(&self, index: &AccessPolicyIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    /// Inserts the given index into the repository.
    ///
    /// If another index with the same resource and allow exists, it will be removed.
    fn insert(&self, index: AccessPolicyIndex) {
        let result = self.find_by_criteria(AccessPolicyIndexCriteria {
            resource: index.resource.clone(),
            allow: Some(index.allow.clone()),
        });

        result.iter().for_each(|id| {
            self.remove(&AccessPolicyIndex {
                resource: index.resource.clone(),
                allow: index.allow.clone(),
                policy_id: *id,
            });
        });

        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &AccessPolicyIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<UUID> {
        DB.with(|db| {
            let start_key = AccessPolicyIndex {
                resource: criteria.resource.to_owned(),
                allow: criteria.allow.clone().unwrap_or_else(|| AllowKey::Any),
                policy_id: [u8::MIN; 16],
            };
            let end_key = AccessPolicyIndex {
                resource: criteria.resource.to_owned(),
                allow: criteria.allow.unwrap_or_else(|| AllowKey::UserGroups),
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
    use crate::models::access_policy::{AccountResourceAction, AllowKey, Resource};

    #[test]
    fn test_index_repository() {
        let repository = AccessPolicyIndexRepository::default();
        let index = AccessPolicyIndex {
            resource: Resource::Account(AccountResourceAction::Create),
            allow: AllowKey::Any,
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
        let repository = AccessPolicyIndexRepository::default();
        let generate_items_nr = 10;
        for i in 0..generate_items_nr {
            let index = if i % 2 == 0 {
                AccessPolicyIndex {
                    resource: Resource::Account(AccountResourceAction::Create),
                    allow: AllowKey::Any,
                    policy_id: [i; 16],
                }
            } else {
                AccessPolicyIndex {
                    resource: Resource::Account(AccountResourceAction::Create),
                    allow: AllowKey::Authenticated,
                    policy_id: [i; 16],
                }
            };
            repository.insert(index.clone());
        }

        let result = repository.find_by_criteria(AccessPolicyIndexCriteria {
            resource: Resource::Account(AccountResourceAction::Create),
            allow: None,
        });

        assert_eq!(result.len(), 10);

        let result = repository.find_by_criteria(AccessPolicyIndexCriteria {
            resource: Resource::Account(AccountResourceAction::Create),
            allow: Some(AllowKey::Any),
        });

        assert_eq!(result.len(), 5);
    }
}
