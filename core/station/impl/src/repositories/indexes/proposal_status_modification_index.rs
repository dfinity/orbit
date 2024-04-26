use crate::{
    core::{with_memory_manager, Memory, PROPOSAL_STATUS_MODIFICATION_INDEX_MEMORY_ID},
    models::indexes::proposal_status_modification_index::{
        ProposalStatusModificationIndex, ProposalStatusModificationIndexCriteria,
    },
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use orbit_essentials::{repository::IndexRepository, types::UUID};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
  static DB: RefCell<StableBTreeMap<ProposalStatusModificationIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(PROPOSAL_STATUS_MODIFICATION_INDEX_MEMORY_ID))
    )
  })
}

#[derive(Default, Debug)]
pub struct ProposalStatusModificationIndexRepository;

impl IndexRepository<ProposalStatusModificationIndex, UUID>
    for ProposalStatusModificationIndexRepository
{
    type FindByCriteria = ProposalStatusModificationIndexCriteria;

    fn exists(&self, index: &ProposalStatusModificationIndex) -> bool {
        DB.with(|m| m.borrow().contains_key(index))
    }

    fn insert(&self, index: ProposalStatusModificationIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &ProposalStatusModificationIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<UUID> {
        DB.with(|db| {
            let start_key = ProposalStatusModificationIndex {
                status: criteria.status.to_owned(),
                modification_timestamp: criteria.from_dt.unwrap_or(u64::MIN),
                proposal_id: [std::u8::MIN; 16],
            };
            let end_key = ProposalStatusModificationIndex {
                status: criteria.status.to_owned(),
                modification_timestamp: criteria.to_dt.unwrap_or(u64::MAX),
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
    use super::*;
    use crate::models::ProposalStatusCode;

    #[test]
    fn test_repository_crud() {
        let repository = ProposalStatusModificationIndexRepository;
        let index = ProposalStatusModificationIndex {
            status: ProposalStatusCode::Created,
            modification_timestamp: 1,
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
        let repository = ProposalStatusModificationIndexRepository;
        let index = ProposalStatusModificationIndex {
            status: ProposalStatusCode::Created,
            modification_timestamp: 1,
            proposal_id: [1; 16],
        };

        repository.insert(index.clone());
        repository.insert(ProposalStatusModificationIndex {
            status: ProposalStatusCode::Created,
            modification_timestamp: 2,
            proposal_id: [2; 16],
        });

        let criteria = ProposalStatusModificationIndexCriteria {
            status: ProposalStatusCode::Created,
            from_dt: Some(0),
            to_dt: Some(1),
        };

        let result = repository.find_by_criteria(criteria);

        assert_eq!(result.len(), 1);
        assert!(result.contains(&index.proposal_id));
    }
}
