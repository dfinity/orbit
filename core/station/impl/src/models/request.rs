use super::request_policy_rule::RequestEvaluationResult;
use super::{
    DisplayUser, EvaluationStatus, RequestApproval, RequestApprovalStatus, RequestOperation,
    RequestStatus, UserId, UserKey,
};
use crate::core::evaluation::{
    Evaluate, REQUEST_APPROVE_RIGHTS_REQUEST_POLICY_RULE_EVALUATOR, REQUEST_POLICY_RULE_EVALUATOR,
    REQUEST_POSSIBLE_APPROVERS_REQUEST_POLICY_RULE_EVALUATOR,
};
use crate::core::ic_cdk::api::print;
use crate::core::ic_cdk::next_time;
use crate::core::request::{
    RequestApprovalRightsEvaluator, RequestEvaluator, RequestPossibleApproversFinder,
};
use crate::core::validation::{StringFieldValidator, StringFieldValidatorBuilder, ValidateField};
use crate::errors::{EvaluateError, RequestError};
use crate::repositories::USER_REPOSITORY;
use candid::{CandidType, Deserialize};
use lazy_static::lazy_static;
use orbit_essentials::model::ModelKey;
use orbit_essentials::repository::Repository;
use orbit_essentials::storable;
use orbit_essentials::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use std::collections::HashSet;

/// The request id, which is a UUID.
pub type RequestId = UUID;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RequestExecutionPlan {
    Immediate,
    Scheduled { execution_time: Timestamp },
}

/// Represents a request within the system.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Request {
    /// The request id, which is a UUID.
    pub id: RequestId,
    /// The title of the request.
    pub title: String,
    /// The summary of the request, this is a longer description of the request.
    pub summary: Option<String>,
    /// The user id that resulted in the request creation.
    pub requested_by: UserId,
    /// The status that the request is in.
    pub status: RequestStatus,
    /// An operation that the request should execute, e.g. "transfer".
    pub operation: RequestOperation,
    /// The expiration date of the request.
    pub expiration_dt: Timestamp,
    /// The execution plan of the request.
    pub execution_plan: RequestExecutionPlan,
    /// The list of user approvals on the request.
    pub approvals: Vec<RequestApproval>,
    /// The timestamp of the request creation.
    pub created_timestamp: Timestamp,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RequestKey {
    /// The request id, which is a UUID.
    pub id: RequestId,
}

impl ModelKey<RequestKey> for Request {
    fn key(&self) -> RequestKey {
        RequestKey { id: self.id }
    }
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct RequestCallerPrivileges {
    pub id: UUID,
    pub can_approve: bool,
}

#[derive(Deserialize, Debug, Clone)]
pub struct RequestAdditionalInfo {
    pub id: UUID,
    pub requester_name: String,
    pub approvers: Vec<DisplayUser>,
    pub evaluation_result: Option<RequestEvaluationResult>,
}

lazy_static! {
    pub static ref REQUEST_TITLE_VALIDATOR: StringFieldValidator = {
        StringFieldValidatorBuilder::new("title".to_string())
            .min_length(1)
            .max_length(Request::MAX_TITLE_LEN as usize)
            .build()
    };
    pub static ref REQUEST_SUMMARY_VALIDATOR: StringFieldValidator = {
        StringFieldValidatorBuilder::new("summary".to_string())
            .min_length(0)
            .max_length(Request::MAX_SUMMARY_LEN as usize)
            .build()
    };
    pub static ref REQUEST_CANCEL_REASON_VALIDATOR: StringFieldValidator = {
        StringFieldValidatorBuilder::new("cancel reason".to_string())
            .min_length(1)
            .max_length(Request::MAX_CANCEL_REASON_LEN as usize)
            .build()
    };
}

fn validate_expiration_dt(expiration_dt: &Timestamp) -> ModelValidatorResult<RequestError> {
    if *expiration_dt <= next_time() {
        return Err(RequestError::ValidationError {
            info: "The expiration date must be in the future".to_owned(),
        });
    }

    Ok(())
}

fn validate_status(status: &RequestStatus) -> ModelValidatorResult<RequestError> {
    match status {
        RequestStatus::Cancelled {
            reason: Some(reason),
        } => {
            REQUEST_CANCEL_REASON_VALIDATOR.validate_field(reason)?;
            Ok(())
        }
        RequestStatus::Created
        | RequestStatus::Rejected
        | RequestStatus::Approved
        | RequestStatus::Completed { .. }
        | RequestStatus::Failed { .. }
        | RequestStatus::Scheduled { .. }
        | RequestStatus::Processing { .. }
        | RequestStatus::Cancelled { reason: None } => Ok(()),
    }
}

fn validate_execution_plan(
    execution_plan: &RequestExecutionPlan,
) -> ModelValidatorResult<RequestError> {
    match execution_plan {
        RequestExecutionPlan::Scheduled { execution_time } => {
            if *execution_time <= next_time() {
                return Err(RequestError::ValidationError {
                    info: "The execution time must be in the future".to_owned(),
                });
            }
        }
        RequestExecutionPlan::Immediate => (),
    }

    Ok(())
}

fn validate_requested_by(requested_by: &UserId) -> ModelValidatorResult<RequestError> {
    USER_REPOSITORY
        .get(&UserKey { id: *requested_by })
        .ok_or(RequestError::ValidationError {
            info: "The requested_by user does not exist".to_owned(),
        })?;
    Ok(())
}

impl ModelValidator<RequestError> for Request {
    fn validate(&self) -> ModelValidatorResult<RequestError> {
        REQUEST_TITLE_VALIDATOR.validate_field(&self.title)?;

        if let Some(summary) = &self.summary {
            REQUEST_SUMMARY_VALIDATOR.validate_field(summary)?;
        }

        validate_requested_by(&self.requested_by)?;

        let must_not_be_expired = match self.status {
            RequestStatus::Created => true,
            RequestStatus::Approved
            | RequestStatus::Rejected
            | RequestStatus::Scheduled { .. }
            | RequestStatus::Cancelled { .. }
            | RequestStatus::Processing { .. }
            | RequestStatus::Completed { .. }
            | RequestStatus::Failed { .. } => false,
        };
        if must_not_be_expired {
            validate_expiration_dt(&self.expiration_dt)?;
            validate_execution_plan(&self.execution_plan)?;
        }

        validate_status(&self.status)?;
        self.operation.validate()?;

        Ok(())
    }
}

impl Request {
    pub const MAX_TITLE_LEN: u8 = 255;
    pub const MAX_CANCEL_REASON_LEN: u16 = 1000;
    pub const MAX_SUMMARY_LEN: u16 = 1000;

    /// Creates a new request key from the given key components.
    pub fn key(request_id: RequestId) -> RequestKey {
        RequestKey { id: request_id }
    }

    pub fn to_key(&self) -> RequestKey {
        Request::key(self.id.to_owned())
    }

    pub fn approvers(&self) -> HashSet<UserId> {
        let mut approvers = HashSet::new();

        self.approvals
            .iter()
            .map(|decision| decision.approver_id.to_owned())
            .for_each(|user_id| {
                approvers.insert(user_id);
            });

        approvers
    }

    /// Gives the default expiration date for a request which is 30 days from the current time.
    pub fn default_expiration_dt_ns() -> Timestamp {
        let time_in_ns: u64 = 30 * 24 * 60 * 60 * 1_000_000_000;

        next_time() + time_in_ns
    }

    /// Checks if the user can approve the request.
    pub fn can_approve(&self, user_id: &UUID) -> bool {
        // Only requests that are in the created state can be approved.
        if self.status != RequestStatus::Created {
            return false;
        }

        // If the user has already added their approval, they can't add again.
        if self
            .approvals
            .iter()
            .any(|approval| approval.approver_id == *user_id)
        {
            return false;
        }

        let approval_rights_evaluator = RequestApprovalRightsEvaluator {
            request: &self.index_fields(),
            approver_id: *user_id,
            approval_rights_evaluator: REQUEST_APPROVE_RIGHTS_REQUEST_POLICY_RULE_EVALUATOR.clone(),
        };

        match approval_rights_evaluator.evaluate() {
            Ok(has_approval_right) => has_approval_right,
            Err(_) => {
                print(format!(
                    "Failed to evaluate voting rights for request: {:?}",
                    self
                ));

                false
            }
        }
    }

    pub fn add_approval(
        &mut self,
        user_id: UUID,
        decision: RequestApprovalStatus,
        reason: Option<String>,
    ) -> ModelValidatorResult<RequestError> {
        if self
            .approvals
            .iter()
            .any(|approval| approval.approver_id == user_id)
        {
            // users can only approval once per request
            return Err(RequestError::ApprovalNotAllowed);
        }

        let now = next_time();
        let approval = RequestApproval {
            approver_id: user_id,
            status: decision,
            status_reason: reason,
            decided_dt: now,
            last_modification_timestamp: now,
        };

        approval.validate()?;

        self.approvals.push(approval);

        Ok(())
    }

    pub async fn reevaluate(&mut self) -> Result<Option<RequestEvaluationResult>, EvaluateError> {
        if self.status == RequestStatus::Created {
            let evaluator = RequestEvaluator {
                request: self.to_owned(),
                policy_rule_evaluator: REQUEST_POLICY_RULE_EVALUATOR.to_owned(),
            };

            let evaluation_result = evaluator.evaluate()?;

            if evaluation_result.status == EvaluationStatus::Approved {
                self.status = RequestStatus::Approved;
            } else if evaluation_result.status == EvaluationStatus::Rejected {
                self.status = RequestStatus::Rejected;
            }

            Ok(Some(evaluation_result))
        } else {
            Ok(None)
        }
    }

    pub async fn find_all_possible_approvers(&self) -> Result<HashSet<UUID>, EvaluateError> {
        let evaluator = RequestPossibleApproversFinder {
            request: self,
            possible_approvers_policy_rule_evaluator:
                REQUEST_POSSIBLE_APPROVERS_REQUEST_POLICY_RULE_EVALUATOR.to_owned(),
        };

        evaluator.evaluate()
    }

    /// Checks if the request is finalized.
    ///
    /// A request that is finalized won't have its status changed anymore.
    pub fn is_finalized(&self) -> bool {
        matches!(
            self.status,
            RequestStatus::Completed { .. }
                | RequestStatus::Cancelled { .. }
                | RequestStatus::Failed { .. }
                | RequestStatus::Rejected
        )
    }
}

#[cfg(test)]
mod tests {
    use super::request_test_utils::mock_request;
    use super::*;
    use crate::models::user_test_utils::mock_user;
    use crate::repositories::USER_REPOSITORY;

    #[test]
    fn fail_request_cancel_reason_too_big() {
        let mut request = mock_request();
        request.status = RequestStatus::Cancelled {
            reason: Some("a".repeat(Request::MAX_CANCEL_REASON_LEN as usize + 1)),
        };

        let result = validate_status(&request.status);

        assert!(result.is_err());
        let error = result.unwrap_err();
        if let RequestError::ValidationError { info } = error {
            assert!(info.contains("Length cannot be longer than 1000"));
        } else {
            panic!("Expected ValidationError, got: {:?}", error);
        }
    }

    #[test]
    fn fail_request_cancel_reason_empty() {
        let mut request = mock_request();
        request.status = RequestStatus::Cancelled {
            reason: Some("".to_owned()),
        };

        let result = validate_status(&request.status);

        assert!(result.is_err());
        let error = result.unwrap_err();
        if let RequestError::ValidationError { info } = error {
            assert!(info.contains("Length cannot be shorter than 1"));
        } else {
            panic!("Expected ValidationError, got: {:?}", error);
        }
    }

    #[test]
    fn test_request_cancel_reason_is_valid() {
        let mut request = mock_request();
        request.status = RequestStatus::Cancelled {
            reason: Some("a".repeat(Request::MAX_CANCEL_REASON_LEN as usize)),
        };

        let result = validate_status(&request.status);

        assert!(result.is_ok());
    }

    #[test]
    fn fail_request_title_too_big() {
        let mut request = mock_request();
        request.title = "a".repeat(Request::MAX_TITLE_LEN as usize + 1);
        request.status = RequestStatus::Completed { completed_at: 0 }; // Use completed status to avoid expiration validation

        // Create a mock user for the requested_by field
        let mut user = mock_user();
        user.id = request.requested_by;
        USER_REPOSITORY.insert(user.to_key(), user);

        let result = request.validate();

        assert!(result.is_err());
        let error = result.unwrap_err();
        if let RequestError::ValidationError { info } = error {
            assert!(info.contains("Length cannot be longer than 255"));
        } else {
            panic!("Expected ValidationError, got: {:?}", error);
        }
    }

    #[test]
    fn fail_request_expiration_dt_in_past() {
        let mut request = mock_request();
        request.expiration_dt = 0;

        let result = validate_expiration_dt(&request.expiration_dt);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            RequestError::ValidationError {
                info: "The expiration date must be in the future".to_owned()
            }
        );
    }

    #[test]
    fn test_request_expiration_dt_is_valid() {
        let mut request = mock_request();
        request.expiration_dt = Request::default_expiration_dt_ns();

        let result = validate_expiration_dt(&request.expiration_dt);

        assert!(result.is_ok());
    }

    #[test]
    fn fail_request_execution_plan_in_past() {
        let mut request = mock_request();
        request.execution_plan = RequestExecutionPlan::Scheduled { execution_time: 0 };

        let result = validate_execution_plan(&request.execution_plan);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            RequestError::ValidationError {
                info: "The execution time must be in the future".to_owned()
            }
        );
    }

    #[test]
    fn test_request_execution_plan_is_valid() {
        let mut request = mock_request();
        request.execution_plan = RequestExecutionPlan::Scheduled {
            execution_time: Request::default_expiration_dt_ns(),
        };

        let result = validate_execution_plan(&request.execution_plan);

        assert!(result.is_ok());

        let mut request = mock_request();
        request.execution_plan = RequestExecutionPlan::Immediate;

        let result = validate_execution_plan(&request.execution_plan);

        assert!(result.is_ok());
    }

    #[test]
    fn test_request_title_is_valid() {
        let mut request = mock_request();
        request.title = "a".repeat(Request::MAX_TITLE_LEN as usize);
        request.status = RequestStatus::Completed { completed_at: 0 }; // Use completed status to avoid expiration validation

        // Create a mock user for the requested_by field
        let mut user = mock_user();
        user.id = request.requested_by;
        USER_REPOSITORY.insert(user.to_key(), user);

        let result = request.validate();

        assert!(result.is_ok());
    }

    #[test]
    fn fail_request_summary_too_big() {
        let mut request = mock_request();
        request.summary = Some("a".repeat(Request::MAX_SUMMARY_LEN as usize + 1));
        request.status = RequestStatus::Completed { completed_at: 0 }; // Use completed status to avoid expiration validation

        // Create a mock user for the requested_by field
        let mut user = mock_user();
        user.id = request.requested_by;
        USER_REPOSITORY.insert(user.to_key(), user);

        let result = request.validate();

        assert!(result.is_err());
        let error = result.unwrap_err();
        if let RequestError::ValidationError { info } = error {
            assert!(info.contains("Length cannot be longer than 1000"));
        } else {
            panic!("Expected ValidationError, got: {:?}", error);
        }
    }

    #[test]
    fn test_request_summary_is_valid() {
        let mut request = mock_request();
        request.summary = Some("a".repeat(Request::MAX_SUMMARY_LEN as usize));
        request.status = RequestStatus::Completed { completed_at: 0 }; // Use completed status to avoid expiration validation

        // Create a mock user for the requested_by field
        let mut user = mock_user();
        user.id = request.requested_by;
        USER_REPOSITORY.insert(user.to_key(), user);

        let result = request.validate();

        assert!(result.is_ok());
    }
}

#[cfg(any(test, feature = "canbench"))]
pub mod request_test_utils {
    use super::*;
    use crate::models::{
        asset_test_utils::mock_asset, Metadata, RequestApprovalStatus, TokenStandard,
        TransferOperation, TransferOperationInput,
    };
    use num_bigint::BigUint;
    use uuid::Uuid;

    pub fn mock_request() -> Request {
        Request {
            id: *Uuid::new_v4().as_bytes(),
            title: "foo".to_string(),
            summary: Some("bar".to_string()),
            requested_by: [1; 16],
            status: RequestStatus::Approved,
            expiration_dt: 100,
            execution_plan: RequestExecutionPlan::Immediate,
            operation: RequestOperation::Transfer(TransferOperation {
                transfer_id: None,
                fee: None,
                input: TransferOperationInput {
                    network: "mainnet".to_string(),
                    amount: candid::Nat(BigUint::from(100u32)),
                    fee: None,
                    metadata: Metadata::default(),
                    to: "0x1234".to_string(),
                    from_account_id: [1; 16],
                    from_asset_id: [0; 16],
                    with_standard: TokenStandard::InternetComputerNative,
                },
                asset: mock_asset(),
            }),
            approvals: vec![RequestApproval {
                approver_id: [1; 16],
                status: RequestApprovalStatus::Approved,
                status_reason: None,
                decided_dt: 0,
                last_modification_timestamp: 0,
            }],
            created_timestamp: 0,
            last_modification_timestamp: 0,
        }
    }
}
