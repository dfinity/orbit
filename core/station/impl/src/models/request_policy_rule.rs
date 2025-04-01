use super::{
    request_specifier::{
        Match, RequestHasMetadata, UserInvolvedInPolicyRuleForRequestResource, UserSpecifier,
    },
    EvaluateError, EvaluationStatus, MetadataItem, NamedRuleId, NamedRuleKey, Percentage, Request,
    RequestApprovalStatus, RequestId, RequestOperation, UserId, UserStatus,
};
use crate::{
    core::{
        ic_cdk::api::print,
        utils::calculate_minimum_threshold,
        validation::{EnsureIdExists, EnsureNamedRule},
    },
    errors::{MatchError, ValidationError},
    repositories::{
        UserWhereClause, ADDRESS_BOOK_REPOSITORY, ASSET_REPOSITORY, NAMED_RULE_REPOSITORY,
        USER_REPOSITORY,
    },
    services::ACCOUNT_SERVICE,
};
use orbit_essentials::storable;
use orbit_essentials::{
    model::{ModelKey, ModelValidator, ModelValidatorResult},
    repository::Repository,
};
use station_api::EvaluationSummaryReasonDTO;
use std::{cmp, hash::Hash};
use std::{collections::HashSet, sync::Arc};
use uuid::Uuid;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RequestPolicyRule {
    AutoApproved,
    QuorumPercentage(UserSpecifier, Percentage),
    Quorum(UserSpecifier, u16),
    AllowListedByMetadata(MetadataItem),
    AllowListed,
    // Logical operators
    Or(Vec<RequestPolicyRule>),
    And(Vec<RequestPolicyRule>),
    Not(Box<RequestPolicyRule>),
    // Named rule
    NamedRule(NamedRuleId),
}

impl RequestPolicyRule {
    pub fn has_named_rule_id(&self, named_rule_id: &NamedRuleId) -> bool {
        match self {
            RequestPolicyRule::NamedRule(id) => id == named_rule_id,
            RequestPolicyRule::And(rules) | RequestPolicyRule::Or(rules) => rules
                .iter()
                .any(|rule| rule.has_named_rule_id(named_rule_id)),
            RequestPolicyRule::Not(rule) => rule.has_named_rule_id(named_rule_id),
            RequestPolicyRule::AutoApproved
            | RequestPolicyRule::QuorumPercentage(..)
            | RequestPolicyRule::Quorum(..)
            | RequestPolicyRule::AllowListedByMetadata(..)
            | RequestPolicyRule::AllowListed => false,
        }
    }
}

impl ModelValidator<ValidationError> for RequestPolicyRule {
    fn validate(&self) -> ModelValidatorResult<ValidationError> {
        match self {
            RequestPolicyRule::AutoApproved
            | RequestPolicyRule::AllowListedByMetadata(_)
            | RequestPolicyRule::AllowListed => Ok(()),

            RequestPolicyRule::QuorumPercentage(user_specifier, _)
            | RequestPolicyRule::Quorum(user_specifier, _) => user_specifier.validate(),

            RequestPolicyRule::Or(policy_rules) | RequestPolicyRule::And(policy_rules) => {
                for rule in policy_rules {
                    rule.validate()?;
                }
                Ok(())
            }
            RequestPolicyRule::Not(rule) => rule.validate(),

            RequestPolicyRule::NamedRule(rule_id) => {
                EnsureNamedRule::id_exists(rule_id).map_err(ValidationError::RecordValidationError)
            }
        }
    }
}

#[storable]
#[derive(Debug, Clone, PartialEq)]
pub enum EvaluatedRequestPolicyRule {
    AutoApproved,
    QuorumPercentage {
        min_approved: usize,
        total_possible_approvers: usize,
        approvers: Vec<UserId>,
    },
    Quorum {
        min_approved: usize,
        total_possible_approvers: usize,
        approvers: Vec<UserId>,
    },
    AllowListedByMetadata {
        metadata: MetadataItem,
    },
    AllowListed,
    // Logical operators
    Or(Vec<RequestPolicyRuleResult>),
    And(Vec<RequestPolicyRuleResult>),
    Not(Box<RequestPolicyRuleResult>),
}

#[storable]
#[derive(Debug, Clone, PartialEq)]
pub struct RequestPolicyRuleResult {
    pub status: EvaluationStatus,
    pub evaluated_rule: EvaluatedRequestPolicyRule,
}

type EvaluationSummaryReason = EvaluationSummaryReasonDTO;

impl RequestPolicyRuleResult {
    pub fn get_status_reason(
        &self,
        final_status: EvaluationStatus,
    ) -> Vec<EvaluationSummaryReason> {
        let mut reasons = vec![];

        match &self.evaluated_rule {
            EvaluatedRequestPolicyRule::AutoApproved => {
                if final_status == EvaluationStatus::Approved {
                    reasons.push(EvaluationSummaryReason::AutoApproved)
                }
            }
            EvaluatedRequestPolicyRule::QuorumPercentage { .. }
            | EvaluatedRequestPolicyRule::Quorum { .. } => {
                if final_status == self.status {
                    reasons.push(EvaluationSummaryReason::ApprovalQuorum);
                }
            }
            EvaluatedRequestPolicyRule::AllowListedByMetadata { .. } => {
                if final_status == self.status {
                    reasons.push(EvaluationSummaryReason::AllowListMetadata);
                }
            }
            EvaluatedRequestPolicyRule::AllowListed => {
                if final_status == self.status {
                    reasons.push(EvaluationSummaryReason::AllowList);
                }
            }
            EvaluatedRequestPolicyRule::Or(rule_results)
            | EvaluatedRequestPolicyRule::And(rule_results) => {
                for rule_result in rule_results {
                    if final_status == self.status {
                        reasons.extend(rule_result.get_status_reason(final_status.clone()));
                    }
                }
            }
            EvaluatedRequestPolicyRule::Not(rule_result) => match final_status {
                EvaluationStatus::Approved => {
                    if rule_result.status == EvaluationStatus::Rejected {
                        reasons.extend(rule_result.get_status_reason(EvaluationStatus::Rejected));
                    }
                }
                EvaluationStatus::Rejected => {
                    if rule_result.status == EvaluationStatus::Approved {
                        reasons.extend(rule_result.get_status_reason(EvaluationStatus::Approved));
                    }
                }
                EvaluationStatus::Pending => {
                    if rule_result.status == EvaluationStatus::Pending {
                        reasons.extend(rule_result.get_status_reason(EvaluationStatus::Pending));
                    }
                }
            },
        }

        reasons
    }
}

#[storable]
#[derive(Debug, Clone)]
pub struct RequestEvaluationResult {
    pub request_id: RequestId,
    pub status: EvaluationStatus,
    pub policy_results: Vec<RequestPolicyRuleResult>,
}

impl ModelKey<RequestId> for RequestEvaluationResult {
    fn key(&self) -> RequestId {
        self.request_id
    }
}

impl RequestEvaluationResult {
    pub fn get_status_reason(&self) -> Vec<EvaluationSummaryReason> {
        let mut reasons = HashSet::new();

        for policy_result in &self.policy_results {
            reasons.extend(policy_result.get_status_reason(self.status.clone()));
        }

        reasons.into_iter().collect()
    }
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum RequestPolicyRuleInput {
    Remove,
    Set(RequestPolicyRule),
}

impl From<RequestApprovalStatus> for EvaluationStatus {
    fn from(value: RequestApprovalStatus) -> Self {
        match value {
            RequestApprovalStatus::Approved => EvaluationStatus::Approved,
            RequestApprovalStatus::Rejected => EvaluationStatus::Rejected,
            // TODO: Abstain
        }
    }
}

pub trait EvaluateRequestPolicyRule<
    Status = EvaluationStatus,
    Context = (Arc<Request>, Arc<RequestPolicyRule>),
    Error = EvaluateError,
>: Sync + Send
{
    fn evaluate(&self, ctx: Context) -> Result<Status, Error>;
}

#[derive(Clone)]
pub struct RequestPolicyRuleEvaluator {
    pub user_matcher: Arc<dyn Match<UserInvolvedInPolicyRuleForRequestResource>>,
    pub address_book_metadata_matcher: Arc<dyn Match<RequestHasMetadata>>,
}

struct RequestApprovalSummary {
    total_possible_approvers: usize,
    approvers: Vec<UserId>,
    approved: usize,
    rejected: usize,
}

impl RequestApprovalSummary {
    /// Evaluates the request approval summary and returns the evaluation status based on the
    /// minimum approvals required.
    ///
    /// If the request does not yet have enough approvals to meet the minimum approvals required but has
    /// enough uncast approvals that could be cast to meet the minimum approvals required, then the evaluation
    /// is kept in the `Pending` state.
    fn evaluate(&self, min_approved: usize) -> EvaluationStatus {
        let min_approved = cmp::min(min_approved, self.total_possible_approvers);
        let uncasted_approvals = self
            .total_possible_approvers
            .saturating_sub(self.approved)
            .saturating_sub(self.rejected);

        if self.approved >= min_approved {
            return EvaluationStatus::Approved;
        }

        if self.approved.saturating_add(uncasted_approvals) < min_approved {
            return EvaluationStatus::Rejected;
        }

        EvaluationStatus::Pending
    }
}

impl RequestPolicyRuleEvaluator {
    fn evaluate_policy_rules(
        &self,
        request: &Arc<Request>,
        policy_rules: &[RequestPolicyRule],
    ) -> Result<Vec<RequestPolicyRuleResult>, EvaluateError> {
        policy_rules
            .iter()
            .map(|rule| self.evaluate((request.to_owned(), Arc::new(rule.to_owned()))))
            .collect()
    }

    fn find_matching_users<UserMatchReturn>(
        &self,
        request: &Arc<Request>,
        users: &[(UserId, UserMatchReturn)],
        user_specifier: &UserSpecifier,
    ) -> Result<Vec<UserMatchReturn>, MatchError>
    where
        UserMatchReturn: Clone,
    {
        let mut result = vec![];

        for (user_id, match_return) in users {
            if self
                .user_matcher
                .is_match(UserInvolvedInPolicyRuleForRequestResource {
                    request_operation_resources: request.operation.to_resources(),
                    policy_rule_user_specifier: user_specifier.to_owned(),
                    user_id: user_id.to_owned(),
                    request_id: request.id,
                })?
            {
                result.push(match_return.clone());
            }
        }

        Ok(result)
    }

    fn calculate_approvals(
        &self,
        request: &Arc<Request>,
        user_specifier: &UserSpecifier,
    ) -> Result<RequestApprovalSummary, MatchError> {
        let cast_approvals = self.find_matching_users::<(UserId, RequestApprovalStatus)>(
            request,
            request
                .approvals
                .iter()
                .map(|approval| {
                    (
                        approval.approver_id.to_owned(),
                        (approval.approver_id.to_owned(), approval.status.to_owned()),
                    )
                })
                .collect::<Vec<(UserId, (UserId, RequestApprovalStatus))>>()
                .as_slice(),
            user_specifier,
        )?;

        let mut total_possible_approvers = self
            .find_matching_users::<()>(
                request,
                USER_REPOSITORY
                    .find_where(UserWhereClause {
                        statuses: Some(vec![UserStatus::Active]),
                        groups: None,
                        search_term: None,
                    })
                    .iter()
                    .map(|user| (user.id.to_owned(), ()))
                    .collect::<Vec<(UserId, ())>>()
                    .as_slice(),
                user_specifier,
            )?
            .len();

        // This is to ensure that if users become inactive or the rule is misconfigured
        // the total_possible_approvals is not less than the cast approvals.
        total_possible_approvers = cmp::max(cast_approvals.len(), total_possible_approvers);

        Ok(RequestApprovalSummary {
            total_possible_approvers,
            approved: cast_approvals
                .iter()
                .filter(|&approval| approval.1 == RequestApprovalStatus::Approved)
                .count(),
            rejected: cast_approvals
                .iter()
                .filter(|&approval| approval.1 == RequestApprovalStatus::Rejected)
                .count(),
            approvers: cast_approvals
                .into_iter()
                .map(|(user_id, _)| user_id)
                .collect(),
        })
    }
}

impl
    EvaluateRequestPolicyRule<
        RequestPolicyRuleResult,
        (Arc<Request>, Arc<RequestPolicyRule>),
        EvaluateError,
    > for RequestPolicyRuleEvaluator
{
    fn evaluate(
        &self,
        (request, critera): (Arc<Request>, Arc<RequestPolicyRule>),
    ) -> Result<RequestPolicyRuleResult, EvaluateError> {
        match critera.as_ref() {
            RequestPolicyRule::AutoApproved => Ok(RequestPolicyRuleResult {
                status: EvaluationStatus::Approved,
                evaluated_rule: EvaluatedRequestPolicyRule::AutoApproved,
            }),
            RequestPolicyRule::QuorumPercentage(user_specifier, percentage) => {
                let approval_summary: RequestApprovalSummary =
                    self.calculate_approvals(&request, user_specifier)?;
                let min_approved = calculate_minimum_threshold(
                    percentage,
                    &approval_summary.total_possible_approvers,
                );

                Ok(RequestPolicyRuleResult {
                    status: approval_summary.evaluate(min_approved),
                    evaluated_rule: EvaluatedRequestPolicyRule::QuorumPercentage {
                        total_possible_approvers: approval_summary.total_possible_approvers,
                        approvers: approval_summary.approvers,
                        min_approved,
                    },
                })
            }
            RequestPolicyRule::Quorum(user_specifier, min_approved) => {
                let approval_summary = self.calculate_approvals(&request, user_specifier)?;

                Ok(RequestPolicyRuleResult {
                    status: approval_summary.evaluate(*min_approved as usize),
                    evaluated_rule: EvaluatedRequestPolicyRule::Quorum {
                        total_possible_approvers: approval_summary.total_possible_approvers,
                        approvers: approval_summary.approvers,
                        min_approved: *min_approved as usize,
                    },
                })
            }
            RequestPolicyRule::AllowListedByMetadata(metadata) => {
                let is_match = self
                    .address_book_metadata_matcher
                    .is_match((request.as_ref().to_owned(), metadata.clone()))?;

                Ok(RequestPolicyRuleResult {
                    status: if is_match {
                        EvaluationStatus::Approved
                    } else {
                        EvaluationStatus::Rejected
                    },
                    evaluated_rule: EvaluatedRequestPolicyRule::AllowListedByMetadata {
                        metadata: metadata.clone(),
                    },
                })
            }
            RequestPolicyRule::AllowListed => {
                if let RequestOperation::Transfer(transfer) = &request.operation {
                    let account = ACCOUNT_SERVICE.get_account(&transfer.input.from_account_id);
                    match account {
                        Err(e) => {
                            print(format!(
                                "Rule rejected due to account not being found: {:?}",
                                e
                            ));

                            return Ok(RequestPolicyRuleResult {
                                status: EvaluationStatus::Rejected,
                                evaluated_rule: EvaluatedRequestPolicyRule::AllowListed,
                            });
                        }
                        Ok(account) => {
                            for account_asset in account.assets {
                                let Some(asset) = ASSET_REPOSITORY.get(&account_asset.asset_id)
                                else {
                                    print(format!(
                                        "Asset `{}` not found in account `{}`.",
                                        Uuid::from_bytes(account_asset.asset_id).hyphenated(),
                                        Uuid::from_bytes(account.id).hyphenated()
                                    ));

                                    continue;
                                };

                                let is_in_address_book = ADDRESS_BOOK_REPOSITORY
                                    .exists(asset.blockchain, transfer.input.to.clone());

                                if is_in_address_book {
                                    return Ok(RequestPolicyRuleResult {
                                        status: EvaluationStatus::Approved,
                                        evaluated_rule: EvaluatedRequestPolicyRule::AllowListed,
                                    });
                                }
                            }
                        }
                    }
                }

                Ok(RequestPolicyRuleResult {
                    status: EvaluationStatus::Rejected,
                    evaluated_rule: EvaluatedRequestPolicyRule::AllowListed,
                })
            }
            RequestPolicyRule::And(policy_rules) => {
                let evaluation_statuses = self.evaluate_policy_rules(&request, policy_rules)?;

                if evaluation_statuses
                    .iter()
                    .any(|result| result.status == EvaluationStatus::Rejected)
                {
                    return Ok(RequestPolicyRuleResult {
                        status: EvaluationStatus::Rejected,
                        evaluated_rule: EvaluatedRequestPolicyRule::And(evaluation_statuses),
                    });
                }

                if evaluation_statuses
                    .iter()
                    .all(|result| result.status == EvaluationStatus::Approved)
                {
                    return Ok(RequestPolicyRuleResult {
                        status: EvaluationStatus::Approved,
                        evaluated_rule: EvaluatedRequestPolicyRule::And(evaluation_statuses),
                    });
                }

                Ok(RequestPolicyRuleResult {
                    status: EvaluationStatus::Pending,
                    evaluated_rule: EvaluatedRequestPolicyRule::And(evaluation_statuses),
                })
            }
            RequestPolicyRule::Or(policy_rules) => {
                let evaluation_statuses = self.evaluate_policy_rules(&request, policy_rules)?;

                if evaluation_statuses
                    .iter()
                    .any(|result| result.status == EvaluationStatus::Approved)
                {
                    return Ok(RequestPolicyRuleResult {
                        status: EvaluationStatus::Approved,
                        evaluated_rule: EvaluatedRequestPolicyRule::Or(evaluation_statuses),
                    });
                }

                if evaluation_statuses
                    .iter()
                    .all(|result| result.status == EvaluationStatus::Rejected)
                {
                    return Ok(RequestPolicyRuleResult {
                        status: EvaluationStatus::Rejected,
                        evaluated_rule: EvaluatedRequestPolicyRule::Or(evaluation_statuses),
                    });
                }

                Ok(RequestPolicyRuleResult {
                    status: EvaluationStatus::Pending,
                    evaluated_rule: EvaluatedRequestPolicyRule::Or(evaluation_statuses),
                })
            }
            RequestPolicyRule::Not(policy_rule) => {
                let evaluation_result = self.evaluate((
                    request.to_owned(),
                    Arc::new(policy_rule.as_ref().to_owned()),
                ))?;
                Ok(RequestPolicyRuleResult {
                    status: match evaluation_result.status {
                        EvaluationStatus::Pending => EvaluationStatus::Pending,
                        EvaluationStatus::Approved => EvaluationStatus::Rejected,
                        EvaluationStatus::Rejected => EvaluationStatus::Approved,
                    },
                    evaluated_rule: EvaluatedRequestPolicyRule::Not(Box::new(evaluation_result)),
                })
            }

            RequestPolicyRule::NamedRule(rule_id) => {
                let named_rule = NAMED_RULE_REPOSITORY
                    .get(&NamedRuleKey { id: *rule_id })
                    .ok_or_else(|| {
                        EvaluateError::UnexpectedError(anyhow::anyhow!(
                            "failed to get named rule with id {}",
                            Uuid::from_bytes(*rule_id).hyphenated()
                        ))
                    })?;

                self.evaluate((request.to_owned(), Arc::new(named_rule.rule.to_owned())))
            }
        }
    }
}

#[cfg(test)]
pub mod request_policy_rule_test_utils {
    use super::*;

    pub fn mock_request_evaluation_result() -> RequestEvaluationResult {
        RequestEvaluationResult {
            request_id: [0; 16],
            status: EvaluationStatus::Approved,
            policy_results: vec![
                RequestPolicyRuleResult {
                    status: EvaluationStatus::Approved,
                    evaluated_rule: EvaluatedRequestPolicyRule::Or(vec![
                        RequestPolicyRuleResult {
                            status: EvaluationStatus::Approved,
                            evaluated_rule: EvaluatedRequestPolicyRule::AllowListed,
                        },
                        RequestPolicyRuleResult {
                            status: EvaluationStatus::Rejected,
                            evaluated_rule: EvaluatedRequestPolicyRule::QuorumPercentage {
                                min_approved: 2,
                                total_possible_approvers: 3,
                                approvers: vec![[0; 16], [1; 16]],
                            },
                        },
                    ]),
                },
                RequestPolicyRuleResult {
                    status: EvaluationStatus::Rejected,
                    evaluated_rule: EvaluatedRequestPolicyRule::Quorum {
                        min_approved: 2,
                        approvers: vec![[0; 16], [1; 16]],
                        total_possible_approvers: 3,
                    },
                },
            ],
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::core::validation::disable_mock_resource_validation;

    #[test]
    fn fail_critera_with_non_existent_user_specifier() {
        disable_mock_resource_validation();

        RequestPolicyRule::QuorumPercentage(UserSpecifier::Id(vec![[0; 16]]), Percentage(100))
            .validate()
            .expect_err("Rule with non-existent user specifier should fail");

        RequestPolicyRule::QuorumPercentage(UserSpecifier::Group(vec![[0; 16]]), Percentage(100))
            .validate()
            .expect_err("Rule with non-existent user group specifier should fail");

        RequestPolicyRule::Quorum(UserSpecifier::Id(vec![[0; 16]]), 1)
            .validate()
            .expect_err("Rule with non-existent user specifier should fail");

        RequestPolicyRule::Quorum(UserSpecifier::Group(vec![[0; 16]]), 1)
            .validate()
            .expect_err("Rule with non-existent user group specifier should fail");

        RequestPolicyRule::And(vec![RequestPolicyRule::Or(vec![RequestPolicyRule::Not(
            Box::new(RequestPolicyRule::QuorumPercentage(
                UserSpecifier::Id(vec![[0; 16]]),
                Percentage(100),
            )),
        )])])
        .validate()
        .expect_err("Rule with non-existent user specifier should fail");
    }

    #[test]
    fn test_evaluation_reasons() {
        let result = RequestPolicyRuleResult {
            status: EvaluationStatus::Rejected,

            evaluated_rule: EvaluatedRequestPolicyRule::And(vec![
                RequestPolicyRuleResult {
                    status: EvaluationStatus::Pending,
                    evaluated_rule: EvaluatedRequestPolicyRule::Quorum {
                        min_approved: 2,
                        total_possible_approvers: 4,
                        approvers: vec![],
                    },
                },
                RequestPolicyRuleResult {
                    status: EvaluationStatus::Rejected,
                    evaluated_rule: EvaluatedRequestPolicyRule::AllowListedByMetadata {
                        metadata: MetadataItem {
                            key: "k".to_owned(),
                            value: "v".to_owned(),
                        },
                    },
                },
                RequestPolicyRuleResult {
                    status: EvaluationStatus::Approved,
                    evaluated_rule: EvaluatedRequestPolicyRule::Or(vec![
                        RequestPolicyRuleResult {
                            status: EvaluationStatus::Pending,
                            evaluated_rule: EvaluatedRequestPolicyRule::QuorumPercentage {
                                min_approved: 1,
                                total_possible_approvers: 1,
                                approvers: vec![],
                            },
                        },
                        RequestPolicyRuleResult {
                            status: EvaluationStatus::Approved,
                            evaluated_rule: EvaluatedRequestPolicyRule::AllowListed,
                        },
                    ]),
                },
            ]),
        };

        let request_result = RequestEvaluationResult {
            request_id: [0; 16],
            status: result.status.clone(),
            policy_results: vec![result],
        };

        assert_eq!(
            request_result.get_status_reason(),
            vec![EvaluationSummaryReason::AllowListMetadata]
        );
    }
}
