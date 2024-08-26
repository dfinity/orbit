use crate::{
    core::{with_memory_manager, Memory, REQUEST_EVALUATION_RESULT_MEMORY_ID},
    models::{RequestEvaluationResult, RequestId},
};
use ic_stable_structures::{memory_manager::VirtualMemory, StableBTreeMap};
use lazy_static::lazy_static;
use orbit_essentials::repository::{Repository, StableDb};
use std::{cell::RefCell, sync::Arc};

thread_local! {
  static DB: RefCell<StableBTreeMap<RequestId, RequestEvaluationResult, VirtualMemory<Memory>>> = with_memory_manager(|memory_manager| {
    RefCell::new(
      StableBTreeMap::init(memory_manager.get(REQUEST_EVALUATION_RESULT_MEMORY_ID))
    )
  })
}

lazy_static! {
    pub static ref REQUEST_EVALUATION_RESULT_REPOSITORY: Arc<EvaluationResultRepository> =
        Arc::new(EvaluationResultRepository::default());
}

/// A repository that stores request evaluation results in stable memory.
#[derive(Default, Debug)]
pub struct EvaluationResultRepository {}

impl StableDb<RequestId, RequestEvaluationResult, VirtualMemory<Memory>>
    for EvaluationResultRepository
{
    fn with_db<F, R>(f: F) -> R
    where
        F: FnOnce(
            &mut StableBTreeMap<RequestId, RequestEvaluationResult, VirtualMemory<Memory>>,
        ) -> R,
    {
        DB.with(|m| f(&mut m.borrow_mut()))
    }
}

impl Repository<RequestId, RequestEvaluationResult, VirtualMemory<Memory>>
    for EvaluationResultRepository
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::request_policy_rule_test_utils;

    #[test]
    fn test_crud() {
        let repository = EvaluationResultRepository::default();
        let evaluation_result = request_policy_rule_test_utils::mock_request_evaluation_result();

        assert!(repository.get(&evaluation_result.request_id).is_none());

        repository.insert(
            evaluation_result.request_id.to_owned(),
            evaluation_result.clone(),
        );

        assert!(repository.get(&evaluation_result.request_id).is_some());
        assert!(repository.remove(&evaluation_result.request_id).is_some());
        assert!(repository.get(&evaluation_result.request_id).is_none());
    }
}
