use crate::{
    core::{with_memory_manager, Memory, OPERATION_TYPE_TO_PROPOSAL_ID_INDEX_MEMORY_ID},
    models::{
        indexes::operation_type_to_proposal_id_index::{
            OperationTypeToProposalIdIndex, OperationTypeToProposalIdIndexCriteria,
        },
        ProposalId,
    },
};
use ic_canister_core::repository::IndexRepository;
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use std::{cell::RefCell, collections::HashSet};

thread_local! {
static DB: RefCell<StableBTreeMap<OperationTypeToProposalIdIndex, (), VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
  RefCell::new(
    StableBTreeMap::init(memory_manager.get(OPERATION_TYPE_TO_PROPOSAL_ID_INDEX_MEMORY_ID))
  )
})
}

/// A repository that enables finding proposals based on the operation type.
#[derive(Default, Debug)]
pub struct OperationTypeToProposalIdIndexRepository {}

impl IndexRepository<OperationTypeToProposalIdIndex, ProposalId>
    for OperationTypeToProposalIdIndexRepository
{
    type FindByCriteria = OperationTypeToProposalIdIndexCriteria;

    fn exists(&self, index: &OperationTypeToProposalIdIndex) -> bool {
        DB.with(|m| m.borrow().get(index).is_some())
    }

    fn insert(&self, index: OperationTypeToProposalIdIndex) {
        DB.with(|m| m.borrow_mut().insert(index, ()));
    }

    fn remove(&self, index: &OperationTypeToProposalIdIndex) -> bool {
        DB.with(|m| m.borrow_mut().remove(index).is_some())
    }

    fn find_by_criteria(&self, criteria: Self::FindByCriteria) -> HashSet<ProposalId> {
        DB.with(|db| {
            let start_key = OperationTypeToProposalIdIndex {
                operation_type: criteria.operation_type.clone(),
                proposal_id: [u8::MIN; 16],
            };
            let end_key = OperationTypeToProposalIdIndex {
                operation_type: criteria.operation_type,
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
    use crate::models::proposal_operation_filter_type::ProposalOperationFilterType;

    #[test]
    fn test_repository_crud() {
        let repository = OperationTypeToProposalIdIndexRepository::default();
        let index = OperationTypeToProposalIdIndex {
            operation_type: ProposalOperationFilterType::AddAccount,
            proposal_id: [0; 16],
        };

        assert!(!repository.exists(&index));

        repository.insert(index.clone());

        assert!(repository.exists(&index));
        assert!(repository.remove(&index));
        assert!(!repository.exists(&index));
    }

    #[test]
    fn test_find_by_criteria() {
        let repository = OperationTypeToProposalIdIndexRepository::default();
        let index = OperationTypeToProposalIdIndex {
            proposal_id: [0; 16],
            operation_type: ProposalOperationFilterType::Transfer(None),
        };

        repository.insert(index.clone());

        let index = OperationTypeToProposalIdIndex {
            proposal_id: [1; 16],
            operation_type: ProposalOperationFilterType::Transfer(Some([0; 16])),
        };

        repository.insert(index.clone());

        let result = repository.find_by_criteria(OperationTypeToProposalIdIndexCriteria {
            operation_type: ProposalOperationFilterType::AddAccount,
        });

        assert!(result.is_empty());

        let result = repository.find_by_criteria(OperationTypeToProposalIdIndexCriteria {
            operation_type: ProposalOperationFilterType::Transfer(None),
        });

        assert_eq!(result.len(), 1);
        assert!(result.contains(&[0; 16]));

        let result = repository.find_by_criteria(OperationTypeToProposalIdIndexCriteria {
            operation_type: ProposalOperationFilterType::Transfer(Some([0; 16])),
        });

        assert_eq!(result.len(), 1);
    }
}
