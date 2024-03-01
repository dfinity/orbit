use crate::{
    core::{with_memory_manager, Memory, PROPOSAL_STATUS_INDEX_MEMORY_ID},
    models::indexes::proposal_status_index::{ProposalStatusIndex, ProposalStatusIndexCriteria},
};
use ic_canister_core::{repository::IndexRepository, types::UUID};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<ProposalStatusIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(PROPOSAL_STATUS_INDEX_MEMORY_ID))
    )
  })
}

#[derive(Default, Debug)]
pub struct ProposalStatusIndexRepository {}

impl IndexRepository<ProposalStatusIndex, UUID> for ProposalStatusIndexRepository {
    type FindByCriteria = ProposalStatusIndexCriteria;

    fn exists(&self, index: &ProposalStatusIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: ProposalStatusIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &ProposalStatusIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<UUID> {
        DB.with(|db| {
            let start_key = ProposalStatusIndex {
                status: criteria.status.to_owned(),
                proposal_id: [std::u8::MIN; 16],
            };
            let end_key = ProposalStatusIndex {
                status: criteria.status.to_owned(),
                proposal_id: [std::u8::MAX; 16],
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
    use crate::models::ProposalStatus;

    use super::*;

    #[test]
    fn test_repository_crud() {
        let repository = ProposalStatusIndexRepository::default();
        let index = ProposalStatusIndex {
            status: ProposalStatus::Created.to_string(),
            proposal_id: [1; 16],
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_criteria() {
        let repository = ProposalStatusIndexRepository::default();
        let index = ProposalStatusIndex {
            status: ProposalStatus::Created.to_string(),
            proposal_id: [1; 16],
        };

        repository.insert(index.clone());
        repository.insert(ProposalStatusIndex {
            status: ProposalStatus::Created.to_string(),
            proposal_id: [2; 16],
        });

        let criteria = ProposalStatusIndexCriteria {
            status: ProposalStatus::Created.to_string(),
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&index.proposal_id));
    }
}
