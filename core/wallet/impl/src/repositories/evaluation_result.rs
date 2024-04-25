use crate::{
    core::{with_memory_manager, Memory, EVALUATION_RESULT_MEMORY_ID},
    models::{ProposalEvaluationResult, ProposalId},
};
use ic_canister_core::{repository::Repository, types::UUID};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use std::{cell::RefCell, sync::Arc};

thread_local! {
  static DB: RefCell<StableBTreeMap<ProposalId, ProposalEvaluationResult, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(EVALUATION_RESULT_MEMORY_ID))
    )
  })
}

lazy_static! {
    pub static ref EVALUATION_RESULT_REPOSITORY: Arc<EvaluationResultRepository> =
        Arc::new(EvaluationResultRepository::default());
}

/// A repository that stores proposal evaluation results in stable memory.
#[derive(Default, Debug)]
pub struct EvaluationResultRepository {}

impl Repository<ProposalId, ProposalEvaluationResult> for EvaluationResultRepository {
    fn list(&self) -> Vec<ProposalEvaluationResult> {
        DB.with(|m| m.borrow().iter().map(|(_, v)| v).collect())
    }

    fn get(&self, key: &UUID) -> Option<ProposalEvaluationResult> {
        DB.with(|m| m.borrow().get(key))
    }

    fn insert(
        &self,
        key: UUID,
        value: ProposalEvaluationResult,
    ) -> Option<ProposalEvaluationResult> {
        DB.with(|m| {
            let prev = m.borrow_mut().insert(key, value);

            prev
        })
    }

    fn remove(&self, key: &UUID) -> Option<ProposalEvaluationResult> {
        DB.with(|m| {
            let prev = m.borrow_mut().remove(key);

            prev
        })
    }

    fn len(&self) -> usize {
        DB.with(|m| m.borrow().len()) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::criteria_test_utils;

    #[test]
    fn test_crud() {
        let repository = EvaluationResultRepository::default();
        let evaluation_result = criteria_test_utils::mock_proposal_evaluation_result();

        assert!(repository.get(&evaluation_result.proposal_id).is_none());

        repository.insert(
            evaluation_result.proposal_id.to_owned(),
            evaluation_result.clone(),
        );

        assert!(repository.get(&evaluation_result.proposal_id).is_some());
        assert!(repository.remove(&evaluation_result.proposal_id).is_some());
        assert!(repository.get(&evaluation_result.proposal_id).is_none());
    }
}
