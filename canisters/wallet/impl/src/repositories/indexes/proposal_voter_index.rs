use crate::{
    core::{with_memory_manager, Memory, PROPOSAL_VOTER_INDEX_MEMORY_ID},
    models::{
        indexes::proposal_voter_index::{ProposalVoterIndex, ProposalVoterIndexCriteria},
        ProposalId,
    },
};
use ic_canister_core::repository::IndexRepository;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<ProposalVoterIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(PROPOSAL_VOTER_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables finding proposals based on the voter in stable memory.
#[derive(Default, Debug)]
pub struct ProposalVoterIndexRepository {}

impl IndexRepository<ProposalVoterIndex, ProposalId> for ProposalVoterIndexRepository {
    type FindByCriteria = ProposalVoterIndexCriteria;

    fn exists(&self, index: &ProposalVoterIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: ProposalVoterIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &ProposalVoterIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<ProposalId> {
        DB.with(|db| {
            let start_key = ProposalVoterIndex {
                voter_id: criteria.voter_id.to_owned(),
                proposal_id: [u8::MIN; 16],
            };
            let end_key = ProposalVoterIndex {
                voter_id: criteria.voter_id.to_owned(),
                proposal_id: [u8::MAX; 16],
            };

            db.borrow()
                .range(start_key..=end_key)
                .map(|(index, _)| index.proposal_id)
                .collect::<HashSet<ProposalId>>()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repository_crud() {
        let repository = ProposalVoterIndexRepository::default();
        let index = ProposalVoterIndex {
            proposal_id: [0; 16],
            voter_id: [1; 16],
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_criteria() {
        let repository = ProposalVoterIndexRepository::default();
        let index = ProposalVoterIndex {
            proposal_id: [0; 16],
            voter_id: [1; 16],
        };

        repository.insert(index.clone());

        let criteria = ProposalVoterIndexCriteria {
            voter_id: [1; 16],
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&index.proposal_id));
    }
}
