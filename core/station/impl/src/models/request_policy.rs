use super::{request_policy_rule::RequestPolicyRule, request_specifier::RequestSpecifier};
use crate::errors::{MatchError, RequestPolicyError};
use candid::{CandidType, Deserialize};
use orbit_essentials::model::ModelKey;
use orbit_essentials::storable;
use orbit_essentials::{
    model::{ModelValidator, ModelValidatorResult},
    types::UUID,
};

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EvaluationStatus {
    Approved,
    Rejected,
    Pending,
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RequestPolicy {
    pub id: UUID,
    pub specifier: RequestSpecifier,
    pub rule: RequestPolicyRule,
}

impl ModelKey<UUID> for RequestPolicy {
    fn key(&self) -> UUID {
        self.id
    }
}

#[derive(CandidType, Deserialize, Debug, Clone)]
pub struct RequestPolicyCallerPrivileges {
    pub id: UUID,
    pub can_edit: bool,
    pub can_delete: bool,
}

#[derive(Debug, thiserror::Error)]
pub enum EvaluateError {
    #[error("unauthorized")]
    Unauthorized,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl From<MatchError> for EvaluateError {
    fn from(value: MatchError) -> Self {
        match value {
            MatchError::UnexpectedError(err) => EvaluateError::UnexpectedError(err),
        }
    }
}

impl ModelValidator<RequestPolicyError> for RequestPolicy {
    fn validate(&self) -> ModelValidatorResult<RequestPolicyError> {
        self.specifier.validate()?;
        self.rule.validate()?;
        Ok(())
    }
}

#[cfg(test)]
pub mod request_policy_test_utils {
    use super::RequestPolicy;
    use crate::{
        core::CallContext,
        models::{
            request_policy_rule::RequestPolicyRule, request_specifier::RequestSpecifier,
            AddNamedRuleOperationInput, AddRequestPolicyOperationInput, AddUserOperationInput,
            ADMIN_GROUP_ID,
        },
        services::{NAMED_RULE_SERVICE, REQUEST_POLICY_SERVICE, REQUEST_SERVICE, USER_SERVICE},
    };
    use candid::Principal;
    use station_api::CreateRequestInput;
    use uuid::Uuid;

    pub fn mock_request_policy() -> RequestPolicy {
        RequestPolicy {
            id: *Uuid::new_v4().as_bytes(),
            specifier: RequestSpecifier::AddAccount,
            rule: RequestPolicyRule::AutoApproved,
        }
    }

    #[tokio::test]
    async fn test_named_rule_traversal() {
        let named_rule_1 = NAMED_RULE_SERVICE
            .create(AddNamedRuleOperationInput {
                description: None,
                name: "auto_approver".to_string(),
                rule: RequestPolicyRule::AutoApproved,
            })
            .expect("Failed to create named rule");

        let named_rule_2 = NAMED_RULE_SERVICE
            .create(AddNamedRuleOperationInput {
                description: None,
                name: "test".to_string(),
                rule: RequestPolicyRule::Or(vec![
                    RequestPolicyRule::NamedRule(named_rule_1.id),
                    RequestPolicyRule::AllowListed,
                ]),
            })
            .expect("Failed to create named rule");

        REQUEST_POLICY_SERVICE
            .add_request_policy(AddRequestPolicyOperationInput {
                specifier: RequestSpecifier::AddUser,
                rule: RequestPolicyRule::NamedRule(named_rule_2.id),
            })
            .expect("Failed to add request policy");

        let user = USER_SERVICE
            .add_user(AddUserOperationInput {
                groups: vec![*ADMIN_GROUP_ID],
                identities: vec![Principal::from_slice(&[1; 29])],
                name: "admin".to_string(),
                status: crate::models::UserStatus::Active,
            })
            .expect("Failed to add user");

        let ctx = CallContext::new(user.identities[0]);

        let request = REQUEST_SERVICE
            .create_request(
                CreateRequestInput {
                    operation: station_api::RequestOperationInput::AddUser(
                        station_api::AddUserOperationInput {
                            groups: vec![],
                            identities: vec![],
                            name: "test".to_string(),
                            status: station_api::UserStatusDTO::Active,
                        },
                    ),
                    title: None,
                    summary: None,
                    execution_plan: None,
                    expiration_dt: None,
                },
                &ctx,
            )
            .await
            .expect("Failed to create request");

        assert!(matches!(
            request.status,
            crate::models::RequestStatus::Approved
        ));
    }
}
