use super::{RequestApprovalStatus, UserId};
use crate::core::validation::{StringFieldValidator, StringFieldValidatorBuilder, ValidateField};
use crate::errors::RequestError;
use lazy_static::lazy_static;
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

lazy_static! {
    pub static ref REQUEST_APPROVAL_REASON_VALIDATOR: StringFieldValidator = {
        StringFieldValidatorBuilder::new("status reason".to_string())
            .min_length(0)
            .max_length(RequestApproval::MAX_REASON_LEN as usize)
            .build()
    };
}

impl ModelValidator<RequestError> for RequestApproval {
    fn validate(&self) -> ModelValidatorResult<RequestError> {
        if let Some(reason) = &self.status_reason {
            REQUEST_APPROVAL_REASON_VALIDATOR.validate_field(reason)?;
        }

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
        let error = result.unwrap_err();
        if let RequestError::ValidationError { info } = error {
            assert!(info.contains("Length cannot be longer than 200"));
        } else {
            panic!("Expected ValidationError, got: {:?}", error);
        }
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
