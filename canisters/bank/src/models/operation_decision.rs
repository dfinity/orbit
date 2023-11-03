use super::{OperationStatus, UserId};
use crate::errors::OperationError;
use candid::{CandidType, Deserialize};
use ic_canister_core::{
    model::{ModelValidator, ModelValidatorResult},
    types::Timestamp,
};
use ic_canister_macros::stable_object;

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct OperationDecision {
    /// The user id that this operation task is assigned to.
    pub user_id: UserId,
    /// If the operation is marked as read by the user that it is associated with.
    pub read: bool,
    /// The status is provided by the associated user.
    pub status: OperationStatus,
    /// Optional reason for the operation status.
    pub status_reason: Option<String>,
    /// When the operation was acted on.
    pub decided_dt: Option<Timestamp>,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
}

pub struct OperationDecisionValidator<'model> {
    task: &'model OperationDecision,
}

impl<'model> OperationDecisionValidator<'model> {
    pub const MAX_REASON_LEN: u8 = 200;

    pub fn new(task: &'model OperationDecision) -> OperationDecisionValidator {
        OperationDecisionValidator { task }
    }

    pub fn validate_reason(&self) -> ModelValidatorResult<OperationError> {
        if let Some(reason) = &self.task.status_reason {
            if reason.len() > Self::MAX_REASON_LEN as usize {
                return Err(OperationError::TaskReasonTooLong {
                    max_len: Self::MAX_REASON_LEN,
                });
            }
        }

        Ok(())
    }

    pub fn validate(&self) -> ModelValidatorResult<OperationError> {
        self.validate_reason()?;

        Ok(())
    }
}

impl ModelValidator<OperationError> for OperationDecision {
    fn validate(&self) -> ModelValidatorResult<OperationError> {
        OperationDecisionValidator::new(self).validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fail_operation_decision_too_big_reason() {
        let mut decision = mock_decision();
        decision.status_reason = Some("a".repeat(201));

        let result = decision.validate();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            OperationError::TaskReasonTooLong { max_len: 200 }
        );
    }

    #[test]
    fn test_operation_decision_with_reason() {
        let mut decision = mock_decision();
        decision.status_reason = Some("a".repeat(200));

        let result = decision.validate();

        assert!(result.is_ok());
    }

    fn mock_decision() -> OperationDecision {
        OperationDecision {
            user_id: [0; 16],
            read: false,
            status: OperationStatus::Rejected,
            status_reason: None,
            decided_dt: None,
            last_modification_timestamp: 0,
        }
    }
}
