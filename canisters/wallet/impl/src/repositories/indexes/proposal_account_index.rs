use crate::{
    core::{with_memory_manager, Memory, PROPOSAL_ACCOUNT_INDEX_MEMORY_ID},
    models::{
        indexes::proposal_account_index::{ProposalAccountIndex, ProposalAccountIndexCriteria},
        ProposalId,
    },
};
use ic_canister_core::repository::IndexRepository;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<ProposalAccountIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(PROPOSAL_ACCOUNT_INDEX_MEMORY_ID))
    )
  })
}

/// A repository that enables finding proposals based on the account id in stable memory.
#[derive(Default, Debug)]
pub struct ProposalAccountIndexRepository {}

impl IndexRepository<ProposalAccountIndex, ProposalId> for ProposalAccountIndexRepository {
    type FindByCriteria = ProposalAccountIndexCriteria;

    fn exists(&self, key: &ProposalAccountIndex) -> bool {
        DB.with(|m| m.borrow().get(key).is_some())
    }

    fn insert(&self, key: ProposalAccountIndex) {
        DB.with(|m| m.borrow_mut().insert(key, ()));
    }

    fn remove(&self, key: &ProposalAccountIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(key).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<ProposalId> {
        DB.with(|db| {
            let start_key = ProposalAccountIndex {
                account_id: criteria.account_id.to_owned(),
                proposal_id: [u8::MIN; 16],
            };
            let end_key = ProposalAccountIndex {
                account_id: criteria.account_id.to_owned(),
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
        let repository = ProposalAccountIndexRepository::default();
        let index = ProposalAccountIndex {
            proposal_id: [0; 16],
            account_id: [1; 16],
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_criteria() {
        let repository = ProposalAccountIndexRepository::default();
        let index = ProposalAccountIndex {
            proposal_id: [0; 16],
            account_id: [1; 16],
        };

        repository.insert(index.clone());

        let criteria = ProposalAccountIndexCriteria {
            account_id: [1; 16],
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&index.proposal_id));
    }
}
