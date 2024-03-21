use crate::{
    core::{with_memory_manager, Memory, PROPOSAL_RESOURCE_INDEX_MEMORY_ID},
    models::{
        access_policy::Resource,
        indexes::proposal_resource_index::{ProposalResourceIndex, ProposalResourceIndexCriteria},
    },
};
use ic_canister_core::{repository::IndexRepository, types::UUID};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<ProposalResourceIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(PROPOSAL_RESOURCE_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables finding proposals based on the proposer in stable memory.
#[derive(Default, Debug)]
pub struct ProposalResourceIndexRepository {}

impl IndexRepository<ProposalResourceIndex, UUID> for ProposalResourceIndexRepository {
    type FindByCriteria = ProposalResourceIndexCriteria;

    fn exists(&self, index: &ProposalResourceIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: ProposalResourceIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &ProposalResourceIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<UUID> {
        DB.with(|db| {
            let start_key = ProposalResourceIndex {
                proposal_id: criteria.proposal_id,
                resource: Resource::min(),
            };
            let end_key = ProposalResourceIndex {
                proposal_id: criteria.proposal_id,
                resource: Resource::max(),
            };

            db.borrow()
                .range(start_key..=end_key)
                .map(|(index, _)| index.proposal_id)
                .collect::<HashSet<UUID>>()
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::models::access_policy::UserResourceAction;

    use super::*;

    #[test]
    fn test_repository_crud() {
        let repository = ProposalResourceIndexRepository::default();
        let index = ProposalResourceIndex {
            proposal_id: [0; 16],
            resource: Resource::User(UserResourceAction::Create),
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_criteria() {
        let repository = ProposalResourceIndexRepository::default();
        let index = ProposalResourceIndex {
            proposal_id: [0; 16],
            resource: Resource::User(UserResourceAction::Create),
        };

        repository.insert(index.clone());

        let criteria = ProposalResourceIndexCriteria {
            proposal_id: [0; 16],
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&index.proposal_id));
    }
}
