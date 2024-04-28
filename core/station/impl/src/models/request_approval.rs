use super::{RequestApprovalStatus, UserId};
use crate::errors::RequestError;
use orbit_essentials::storable;
use orbit_essentials::{
    model::{ModelValidator, ModelValidatorResult},
    types::Timestamp,
};

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RequestApproval {
    /// The user that has recorded the approval decision.
    pub approver_id: UserId,
    /// The status is provided by the associated user.
    pub status: RequestApprovalStatus,
    /// Optional reason for the decision.
    pub status_reason: Option<String>,
    /// The time at which the decision was made.
    pub decided_dt: Timestamp,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
}

impl RequestApproval {
    pub const MAX_REASON_LEN: u8 = 200;
}

fn validate_reason(reason: &Option<String>) -> ModelValidatorResult<RequestError> {
    if let Some(reason) = reason {
        if reason.len() > RequestApproval::MAX_REASON_LEN as usize {
            return Err(RequestError::ApprovalReasonTooLong {
                max_len: RequestApproval::MAX_REASON_LEN,
            });
        }
    }

    Ok(())
}

impl ModelValidator<RequestError> for RequestApproval {
    fn validate(&self) -> ModelValidatorResult<RequestError> {
        validate_reason(&self.status_reason)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fail_request_approval_too_big_reason() {
        let mut decision = request_approval_test_utils::mock_decision();
        decision.status_reason = Some("a".repeat(201));

        let result = decision.validate();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            RequestError::ApprovalReasonTooLong { max_len: 200 }
        );
    }

    #[test]
    fn test_request_approval_with_reason() {
        let mut decision = request_approval_test_utils::mock_decision();
        decision.status_reason = Some("a".repeat(200));

        let result = decision.validate();

        assert!(result.is_ok());
    }
}

#[cfg(test)]
pub mod request_approval_test_utils {
    use super::RequestApproval;
    use crate::models::RequestApprovalStatus;
    use orbit_essentials::types::UUID;

    pub fn mock_decision() -> RequestApproval {
        RequestApproval {
            approver_id: [0; 16],
            status: RequestApprovalStatus::Rejected,
            status_reason: None,
            decided_dt: 0,
            last_modification_timestamp: 0,
        }
    }

    pub fn mock_approved_with_user(approver_id: UUID) -> RequestApproval {
        RequestApproval {
            approver_id,
            status: RequestApprovalStatus::Approved,
            status_reason: None,
            decided_dt: 0,
            last_modification_timestamp: 0,
        }
    }

    pub fn mock_rejected_with_user(approver_id: UUID) -> RequestApproval {
        RequestApproval {
            approver_id,
            status: RequestApprovalStatus::Rejected,
            status_reason: None,
            decided_dt: 0,
            last_modification_timestamp: 0,
        }
    }
}
