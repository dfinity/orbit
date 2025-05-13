use super::{request_policy_rule::RequestPolicyRule, request_specifier::RequestSpecifier};
use super::{NamedRuleId, NamedRuleKey};
use crate::errors::{MatchError, RecordValidationError, RequestPolicyError, ValidationError};
use crate::repositories::NAMED_RULE_REPOSITORY;
use candid::{CandidType, Deserialize};
use orbit_essentials::model::ModelKey;
use orbit_essentials::repository::Repository;
use orbit_essentials::storable;
use orbit_essentials::{
    model::{ModelValidator, ModelValidatorResult},
    types::UUID,
};
use uuid::Uuid;

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

/// Check for compatibility between a rule and a specifier:
/// - AllowListed and AllowListedByMetadata are compatible only with Transfer.
///
pub fn validate_rule_for_specifier(
    rule: &RequestPolicyRule,
    specifier: &RequestSpecifier,
    updated_named_rules: &[(&NamedRuleId, &RequestPolicyRule)],
) -> ModelValidatorResult<RequestPolicyError> {
    match rule {
        RequestPolicyRule::AutoApproved => Ok(()),
        RequestPolicyRule::QuorumPercentage(_, _) => Ok(()),
        RequestPolicyRule::Quorum(_, _) => Ok(()),
        RequestPolicyRule::AllowListed | RequestPolicyRule::AllowListedByMetadata(_) => {
            match specifier {
                RequestSpecifier::Transfer(_) => Ok(()),
                _ => Err(RequestPolicyError::InvalidRuleForSpecifier {
                    rule: rule.to_string(),
                    specifier: specifier.to_string(),
                }),
            }
        }

        RequestPolicyRule::And(rules) | RequestPolicyRule::Or(rules) => {
            for rule in rules {
                validate_rule_for_specifier(rule, specifier, updated_named_rules)?;
            }
            Ok(())
        }
        RequestPolicyRule::Not(rule) => {
            validate_rule_for_specifier(rule, specifier, updated_named_rules)
        }
        RequestPolicyRule::NamedRule(named_rule_id) => {
            let rule = if let Some((_, rule)) = updated_named_rules
                .iter()
                .find(|(id, _)| *id == named_rule_id)
            {
                (*rule).clone()
            } else {
                let named_rule = NAMED_RULE_REPOSITORY
                    .get(&NamedRuleKey { id: *named_rule_id })
                    .ok_or(ValidationError::RecordValidationError(
                        RecordValidationError::NotFound {
                            model_name: "NamedRule".to_string(),
                            id: Uuid::from_bytes(*named_rule_id).hyphenated().to_string(),
                        },
                    ))?;

                named_rule.rule
            };

            validate_rule_for_specifier(&rule, specifier, updated_named_rules)
        }
    }
}

impl ModelValidator<RequestPolicyError> for RequestPolicy {
    fn validate(&self) -> ModelValidatorResult<RequestPolicyError> {
        self.specifier.validate()?;
        self.rule.validate()?;
        validate_rule_for_specifier(&self.rule, &self.specifier, &[])?;
        Ok(())
    }
}

#[cfg(test)]
pub mod request_policy_test_utils {
    use super::RequestPolicy;
    use crate::{
        core::CallContext,
        errors::RequestPolicyError,
        models::{
            request_policy_rule::RequestPolicyRule, request_specifier::RequestSpecifier,
            AddNamedRuleOperationInput, AddRequestPolicyOperationInput, AddUserOperationInput,
            MetadataItem, ADMIN_GROUP_ID,
        },
        services::{NAMED_RULE_SERVICE, REQUEST_POLICY_SERVICE, REQUEST_SERVICE, USER_SERVICE},
    };
    use candid::Principal;
    use orbit_essentials::model::ModelValidator;
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
                    RequestPolicyRule::Not(Box::new(RequestPolicyRule::AutoApproved)),
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

    #[test]
    fn test_invalid_rule_for_specifier() {
        let incompatible_rule_1 = NAMED_RULE_SERVICE
            .create(AddNamedRuleOperationInput {
                name: "test_1".to_string(),
                description: None,
                rule: RequestPolicyRule::And(vec![
                    RequestPolicyRule::AllowListed,
                    RequestPolicyRule::AutoApproved,
                ]),
            })
            .expect("Failed to create named rule");

        let incompatible_rule_2 = NAMED_RULE_SERVICE
            .create(AddNamedRuleOperationInput {
                name: "test_2".to_string(),
                description: None,
                rule: RequestPolicyRule::AllowListedByMetadata(MetadataItem {
                    key: "test".to_string(),
                    value: "test".to_string(),
                }),
            })
            .expect("Failed to create named rule");

        let tests = [
            (
                RequestSpecifier::AddAccount,
                RequestPolicyRule::AllowListed,
                Err(RequestPolicyError::InvalidRuleForSpecifier {
                    rule: "AllowListed".to_string(),
                    specifier: "AddAccount".to_string(),
                }),
            ),
            (
                RequestSpecifier::AddAccount,
                RequestPolicyRule::AllowListedByMetadata(MetadataItem {
                    key: "test".to_string(),
                    value: "test".to_string(),
                }),
                Err(RequestPolicyError::InvalidRuleForSpecifier {
                    rule: "AllowListedByMetadata".to_string(),
                    specifier: "AddAccount".to_string(),
                }),
            ),
            (
                RequestSpecifier::AddAccount,
                RequestPolicyRule::NamedRule(incompatible_rule_1.id),
                Err(RequestPolicyError::InvalidRuleForSpecifier {
                    rule: "AllowListed".to_string(),
                    specifier: "AddAccount".to_string(),
                }),
            ),
            (
                RequestSpecifier::AddAccount,
                RequestPolicyRule::NamedRule(incompatible_rule_2.id),
                Err(RequestPolicyError::InvalidRuleForSpecifier {
                    rule: "AllowListedByMetadata".to_string(),
                    specifier: "AddAccount".to_string(),
                }),
            ),
            (
                RequestSpecifier::Transfer(crate::models::resource::ResourceIds::Any),
                RequestPolicyRule::AllowListed,
                Ok(()),
            ),
            (
                RequestSpecifier::Transfer(crate::models::resource::ResourceIds::Any),
                RequestPolicyRule::AllowListedByMetadata(MetadataItem {
                    key: "test".to_string(),
                    value: "test".to_string(),
                }),
                Ok(()),
            ),
        ];

        for (specifier, rule, expected) in tests {
            let policy = RequestPolicy {
                id: *Uuid::new_v4().as_bytes(),
                specifier,
                rule,
            };

            let result = policy.validate();
            assert_eq!(result, expected);
        }
    }
}
