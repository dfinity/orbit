use super::evaluation::Evaluate;
use crate::{
    errors::EvaluateError,
    models::{
        indexes::request_index::RequestIndexFields,
        request_policy_rule::{
            EvaluateRequestPolicyRule, RequestEvaluationResult, RequestPolicyRule,
            RequestPolicyRuleResult,
        },
        request_specifier::{Match, UserInvolvedInPolicyRuleForRequestResource, UserSpecifier},
        EvaluationStatus, NamedRuleKey, Request, RequestId, User, UserId, UserStatus,
    },
    repositories::{
        request_policy::REQUEST_POLICY_REPOSITORY, NAMED_RULE_REPOSITORY, REQUEST_REPOSITORY,
        USER_REPOSITORY,
    },
};
use anyhow::Context;
use orbit_essentials::{repository::Repository, types::UUID};
use std::{collections::HashSet, sync::Arc};
use uuid::Uuid;

pub struct RequestEvaluator {
    pub policy_rule_evaluator: Arc<dyn EvaluateRequestPolicyRule<RequestPolicyRuleResult>>,
    pub request: Request,
}

impl RequestEvaluator {
    pub fn new(
        policy_rule_evaluator: Arc<dyn EvaluateRequestPolicyRule<RequestPolicyRuleResult>>,
        request: Request,
    ) -> Self {
        Self {
            policy_rule_evaluator,
            request,
        }
    }
}

impl Evaluate<RequestEvaluationResult> for RequestEvaluator {
    fn evaluate(&self) -> Result<RequestEvaluationResult, EvaluateError> {
        let matching_policies = self
            .request
            .operation
            .to_resources()
            .iter()
            .flat_map(|resource| REQUEST_POLICY_REPOSITORY.find_by_resource(resource.to_owned()))
            .collect::<Vec<_>>();

        if matching_policies.is_empty() {
            // Since requests handle security critical operations, we want to reject them by default if
            // they don't match any policy. Users need to explicitly add the necessary policies to evaluate them.
            return Ok(RequestEvaluationResult {
                request_id: self.request.id,
                status: EvaluationStatus::Rejected,
                policy_results: vec![],
            });
        }

        let request = Arc::new(self.request.to_owned());
        let mut evaluation_statuses = Vec::new();

        // Evaluate all matching policies to get the full evaluation result.
        for policy in matching_policies {
            // Evaluate the request policy rule.
            let evaluation_status = self
                .policy_rule_evaluator
                .evaluate((request.to_owned(), Arc::new(policy.rule)))
                .context("failed to evaluate policy rule")?;

            evaluation_statuses.push(evaluation_status);
        }

        Ok(RequestEvaluationResult {
            request_id: self.request.id,
            status: {
                if evaluation_statuses
                    .iter()
                    .any(|result| result.status == EvaluationStatus::Approved)
                {
                    // If any policy of the request is approved, then the request is approved.
                    EvaluationStatus::Approved
                } else if evaluation_statuses
                    .iter()
                    .all(|result| result.status == EvaluationStatus::Rejected)
                {
                    // Only if all policies are rejected then the request is rejected,
                    // this applies an implicit `OR` between policies.
                    EvaluationStatus::Rejected
                } else {
                    // Since there are matching policies, but none of them approved or rejected the request, we keep it in the
                    // pending status until one of the policies evaluates it as approved or rejected.
                    EvaluationStatus::Pending
                }
            },
            policy_results: evaluation_statuses,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct PossibleApprovers {
    pub match_all: bool,
    pub users: HashSet<UUID>,
    pub groups: HashSet<UUID>,
}

/// Evaluates all possible approvers for the request.
///
/// The possible approvers are the users that match the request's policies.
pub struct RequestPossibleApproversFinder<'p> {
    pub possible_approvers_policy_rule_evaluator: Arc<
        dyn EvaluateRequestPolicyRule<
            PossibleApprovers,
            (Arc<Request>, Arc<RequestPolicyRule>),
            EvaluateError,
        >,
    >,
    pub request: &'p Request,
}

impl<'p> RequestPossibleApproversFinder<'p> {
    pub fn new(
        possible_approvers_policy_rule_evaluator: Arc<
            dyn EvaluateRequestPolicyRule<
                PossibleApprovers,
                (Arc<Request>, Arc<RequestPolicyRule>),
                EvaluateError,
            >,
        >,
        request: &'p Request,
    ) -> Self {
        Self {
            possible_approvers_policy_rule_evaluator,
            request,
        }
    }
}

impl Evaluate<HashSet<UUID>> for RequestPossibleApproversFinder<'_> {
    fn evaluate(&self) -> Result<HashSet<UUID>, EvaluateError> {
        let mut possible_approvers = HashSet::new();
        let mut matching_groups = HashSet::new();
        let matching_policies = self
            .request
            .operation
            .to_resources()
            .iter()
            .flat_map(|resource| REQUEST_POLICY_REPOSITORY.find_by_resource(resource.to_owned()))
            .collect::<Vec<_>>();

        for policy in matching_policies {
            let result = self.possible_approvers_policy_rule_evaluator.evaluate((
                Arc::new(self.request.to_owned()),
                Arc::new(policy.rule.to_owned()),
            ))?;

            if result.match_all {
                return Ok(USER_REPOSITORY
                    .list()
                    .iter()
                    .filter_map(|user| match user.status {
                        UserStatus::Active => Some(user.id),
                        _ => None,
                    })
                    .collect());
            } else {
                possible_approvers.extend(
                    result
                        .users
                        .iter()
                        .filter_map(|user_id| {
                            USER_REPOSITORY.get(&User::key(*user_id)).map(|u| {
                                if u.status == UserStatus::Active {
                                    Some(u.id)
                                } else {
                                    None
                                }
                            })
                        })
                        .flatten(),
                );
                matching_groups.extend(result.groups);
            }
        }

        for group_id in matching_groups.iter() {
            let users = USER_REPOSITORY
                .find_by_group_and_status(group_id, &UserStatus::Active)
                .iter()
                .map(|user| user.id)
                .collect::<HashSet<UUID>>();

            possible_approvers.extend(users);
        }

        Ok(possible_approvers)
    }
}

pub struct RequestPossibleApproversRequestPolicyRuleEvaluator;

impl
    EvaluateRequestPolicyRule<
        PossibleApprovers,
        (Arc<Request>, Arc<RequestPolicyRule>),
        EvaluateError,
    > for RequestPossibleApproversRequestPolicyRuleEvaluator
{
    fn evaluate(
        &self,
        (request, criteria): (Arc<Request>, Arc<RequestPolicyRule>),
    ) -> Result<PossibleApprovers, EvaluateError> {
        let mut possible_approvers = PossibleApprovers::default();
        match criteria.as_ref() {
            RequestPolicyRule::QuorumPercentage(approver_specifier, _)
            | RequestPolicyRule::Quorum(approver_specifier, _) => match approver_specifier {
                UserSpecifier::Any => {
                    possible_approvers.match_all = true;

                    Ok(possible_approvers)
                }
                UserSpecifier::Id(user_ids) => {
                    possible_approvers.users.extend(user_ids.to_owned());

                    Ok(possible_approvers)
                }
                UserSpecifier::Group(group_ids) => {
                    possible_approvers.groups.extend(group_ids.to_owned());

                    Ok(possible_approvers)
                }
            },
            RequestPolicyRule::AllowListed | RequestPolicyRule::AllowListedByMetadata(_) => {
                Ok(possible_approvers)
            }
            RequestPolicyRule::And(criterias) | RequestPolicyRule::Or(criterias) => {
                for criteria in criterias.iter() {
                    let result = self.evaluate((request.clone(), Arc::new(criteria.clone())));

                    match result {
                        Ok(evaluated) => {
                            if evaluated.match_all {
                                possible_approvers.match_all = true;
                                break;
                            }

                            possible_approvers.users.extend(evaluated.users);
                            possible_approvers.groups.extend(evaluated.groups);
                        }
                        Err(e) => return Err(e),
                    }
                }

                Ok(possible_approvers)
            }
            RequestPolicyRule::Not(criteria) => {
                let result =
                    self.evaluate((request.to_owned(), Arc::new(criteria.as_ref().to_owned())))?;

                if result.match_all {
                    possible_approvers.match_all = true;
                }

                possible_approvers.users.extend(result.users);
                possible_approvers.groups.extend(result.groups);

                Ok(possible_approvers)
            }
            RequestPolicyRule::AutoApproved => Ok(possible_approvers),

            RequestPolicyRule::NamedRule(rule_id) => {
                let named_rule = NAMED_RULE_REPOSITORY
                    .get(&NamedRuleKey { id: *rule_id })
                    .ok_or_else(|| EvaluateError::Failed {
                        reason: format!(
                            "failed to get named rule with id {}",
                            Uuid::from_bytes(*rule_id).hyphenated()
                        ),
                    })?;

                self.evaluate((request.to_owned(), Arc::new(named_rule.rule.to_owned())))
            }
        }
    }
}

/// Evaluates if the user has approval rights to a given request.
///
/// The user has the right to add if:
///
/// - There are matching policies for the request
/// - The user is a part of the group that is allowed to approve
pub struct RequestApprovalRightsEvaluator<'a> {
    pub approval_rights_evaluator: Arc<ApprovalRightsEvaluate>,
    pub approver_id: UserId,
    pub request: &'a RequestIndexFields,
}

pub type ApprovalRightsEvaluate = dyn EvaluateRequestPolicyRule<
    bool,
    (Arc<RequestId>, Arc<UserId>, Arc<RequestPolicyRule>),
    EvaluateError,
>;

impl<'a> RequestApprovalRightsEvaluator<'a> {
    pub fn new(
        approval_rights_evaluator: Arc<ApprovalRightsEvaluate>,
        approver_id: UserId,
        request: &'a RequestIndexFields,
    ) -> Self {
        Self {
            approval_rights_evaluator,
            approver_id,
            request,
        }
    }
}

impl<'a> Evaluate<bool> for RequestApprovalRightsEvaluator<'a> {
    fn evaluate(&self) -> Result<bool, EvaluateError> {
        let matching_policies = self
            .request
            .resources
            .iter()
            .flat_map(|resource| REQUEST_POLICY_REPOSITORY.find_by_resource(resource.to_owned()))
            .collect::<Vec<_>>();

        for policy in matching_policies {
            if self.approval_rights_evaluator.evaluate((
                Arc::new(self.request.id.to_owned()),
                Arc::new(self.approver_id),
                Arc::new(policy.rule.to_owned()),
            ))? {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

pub struct RequesApprovalRightsRequestPolicyRuleEvaluator {
    pub approver_matcher: Arc<dyn Match<UserInvolvedInPolicyRuleForRequestResource>>,
}

impl
    EvaluateRequestPolicyRule<
        bool,
        (Arc<RequestId>, Arc<UserId>, Arc<RequestPolicyRule>),
        EvaluateError,
    > for RequesApprovalRightsRequestPolicyRuleEvaluator
{
    fn evaluate(
        &self,
        (request_id, approver_id, criteria): (Arc<RequestId>, Arc<UserId>, Arc<RequestPolicyRule>),
    ) -> Result<bool, EvaluateError> {
        match criteria.as_ref() {
            RequestPolicyRule::QuorumPercentage(approver_specifier, _)
            | RequestPolicyRule::Quorum(approver_specifier, _) => {
                let can_approve = self
                    .approver_matcher
                    .is_match(UserInvolvedInPolicyRuleForRequestResource {
                        request_operation_resources: REQUEST_REPOSITORY.get_resources(&request_id),
                        policy_rule_user_specifier: approver_specifier.to_owned(),
                        user_id: approver_id.as_ref().to_owned(),
                        request_id: request_id.as_ref().to_owned(),
                    })
                    .context("failed to match request approvers")?;

                Ok(can_approve)
            }
            RequestPolicyRule::AllowListed | RequestPolicyRule::AllowListedByMetadata(_) => {
                Ok(false)
            }
            RequestPolicyRule::And(criterias) | RequestPolicyRule::Or(criterias) => {
                let request = &request_id;
                let approver_id = &approver_id;

                for criteria in criterias.iter() {
                    let can_approve = self.evaluate((
                        request.to_owned(),
                        approver_id.to_owned(),
                        Arc::new(criteria.to_owned()),
                    ))?;

                    if can_approve {
                        return Ok(true);
                    }
                }

                Ok(false)
            }
            RequestPolicyRule::Not(criteria) => {
                let can_approve = self.evaluate((
                    request_id.to_owned(),
                    approver_id.to_owned(),
                    Arc::new(criteria.as_ref().to_owned()),
                ))?;

                Ok(can_approve)
            }
            RequestPolicyRule::AutoApproved => Ok(false),

            RequestPolicyRule::NamedRule(rule_id) => {
                let named_rule = NAMED_RULE_REPOSITORY
                    .get(&NamedRuleKey { id: *rule_id })
                    .ok_or_else(|| EvaluateError::Failed {
                        reason: format!(
                            "failed to get named rule with id {}",
                            Uuid::from_bytes(*rule_id).hyphenated()
                        ),
                    })?;

                self.evaluate((
                    request_id.to_owned(),
                    approver_id.to_owned(),
                    Arc::new(named_rule.rule.to_owned()),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::{
            evaluation::REQUEST_POLICY_RULE_EVALUATOR, middlewares::call_context, set_mock_caller,
        },
        models::{
            request_approval_test_utils::{mock_approved_with_user, mock_rejected_with_user},
            request_policy_test_utils::mock_request_policy,
            request_specifier::RequestSpecifier,
            request_test_utils::mock_request,
            resource::ResourceIds,
            user_test_utils::{self, mock_user},
            Account, AccountKey, AddUserGroupOperation, AddUserGroupOperationInput,
            EvaluatedRequestPolicyRule, Metadata, MetadataItem, Percentage, RequestOperation,
            RequestPolicy, RequestStatus, ADMIN_GROUP_ID,
        },
        repositories::{
            request_policy::REQUEST_POLICY_REPOSITORY, ACCOUNT_REPOSITORY,
            REQUEST_EVALUATION_RESULT_REPOSITORY, REQUEST_REPOSITORY,
        },
        services::RequestService,
    };
    use candid::Principal;
    use orbit_essentials::repository::Repository;
    use station_api::{RequestApprovalStatusDTO, SubmitRequestApprovalInput};
    use uuid::Uuid;

    #[tokio::test]
    async fn is_rejected_when_no_policy_is_available() {
        let request = mock_request();

        REQUEST_REPOSITORY.insert(request.to_key(), request.clone());

        let evaluator = RequestEvaluator {
            request: request.to_owned(),
            policy_rule_evaluator: REQUEST_POLICY_RULE_EVALUATOR.to_owned(),
        };

        let result = evaluator.evaluate().unwrap();

        assert_eq!(result.status, EvaluationStatus::Rejected);
    }

    #[tokio::test]
    async fn succeeds_when_all_criterias_are_approved() {
        let mut request = mock_request();
        let mut policy = mock_request_policy();
        let user = user_test_utils::add_user(&[1; 16]);

        request.operation = RequestOperation::AddUserGroup(AddUserGroupOperation {
            user_group_id: None,
            input: AddUserGroupOperationInput {
                name: "test".to_string(),
            },
        });
        request.requested_by = user.id;
        request.approvals = vec![mock_approved_with_user(user.id)];

        REQUEST_REPOSITORY.insert(request.to_key(), request.clone());

        policy.specifier = RequestSpecifier::AddUserGroup;
        policy.rule = RequestPolicyRule::QuorumPercentage(UserSpecifier::Any, Percentage(100));

        REQUEST_POLICY_REPOSITORY.insert(policy.id, policy.clone());

        let evaluator = RequestEvaluator {
            request: request.to_owned(),
            policy_rule_evaluator: REQUEST_POLICY_RULE_EVALUATOR.to_owned(),
        };

        let result = evaluator.evaluate().unwrap();

        assert_eq!(result.status, EvaluationStatus::Approved);
    }

    #[tokio::test]
    async fn is_pending_when_approved_is_not_reached() {
        let mut request = mock_request();
        let mut policy = mock_request_policy();
        let user = user_test_utils::add_user(&[1; 16]);
        user_test_utils::add_user(&[2; 16]);

        request.operation = RequestOperation::AddUserGroup(AddUserGroupOperation {
            user_group_id: None,
            input: AddUserGroupOperationInput {
                name: "test".to_string(),
            },
        });
        request.requested_by = user.id;
        request.approvals = vec![mock_approved_with_user(user.id)];

        REQUEST_REPOSITORY.insert(request.to_key(), request.clone());

        policy.specifier = RequestSpecifier::AddUserGroup;
        policy.rule = RequestPolicyRule::QuorumPercentage(UserSpecifier::Any, Percentage(100));

        REQUEST_POLICY_REPOSITORY.insert(policy.id, policy.clone());

        let evaluator = RequestEvaluator {
            request: request.to_owned(),
            policy_rule_evaluator: REQUEST_POLICY_RULE_EVALUATOR.to_owned(),
        };

        let result = evaluator.evaluate().unwrap();

        assert_eq!(result.status, EvaluationStatus::Pending);
    }

    #[tokio::test]
    async fn is_rejected_when_approved_is_not_reached() {
        let mut request = mock_request();
        let mut policy = mock_request_policy();
        let user = user_test_utils::add_user(&[1; 16]);
        let user1 = user_test_utils::add_user(&[2; 16]);

        request.operation = RequestOperation::AddUserGroup(AddUserGroupOperation {
            user_group_id: None,
            input: AddUserGroupOperationInput {
                name: "test".to_string(),
            },
        });
        request.requested_by = user.id;
        request.approvals = vec![
            mock_approved_with_user(user.id),
            mock_rejected_with_user(user1.id),
        ];

        REQUEST_REPOSITORY.insert(request.to_key(), request.clone());

        policy.specifier = RequestSpecifier::AddUserGroup;
        policy.rule = RequestPolicyRule::QuorumPercentage(UserSpecifier::Any, Percentage(100));

        REQUEST_POLICY_REPOSITORY.insert(policy.id, policy.clone());

        let evaluator = RequestEvaluator {
            request: request.to_owned(),
            policy_rule_evaluator: REQUEST_POLICY_RULE_EVALUATOR.to_owned(),
        };

        let result = evaluator.evaluate().unwrap();

        assert_eq!(result.status, EvaluationStatus::Rejected);
    }

    #[tokio::test]
    async fn is_approved_disregarding_inactive_users() {
        let mut request = mock_request();
        let mut policy = mock_request_policy();
        let user = user_test_utils::add_user(&[1; 16]);
        user_test_utils::add_inactive_user(&[2; 16]);

        request.operation = RequestOperation::AddUserGroup(AddUserGroupOperation {
            user_group_id: None,
            input: AddUserGroupOperationInput {
                name: "test".to_string(),
            },
        });
        request.requested_by = user.id;
        request.approvals = vec![mock_approved_with_user(user.id)];

        REQUEST_REPOSITORY.insert(request.to_key(), request.clone());

        policy.specifier = RequestSpecifier::AddUserGroup;
        policy.rule = RequestPolicyRule::QuorumPercentage(UserSpecifier::Any, Percentage(100));

        REQUEST_POLICY_REPOSITORY.insert(policy.id, policy.clone());

        let evaluator = RequestEvaluator {
            request: request.to_owned(),
            policy_rule_evaluator: REQUEST_POLICY_RULE_EVALUATOR.to_owned(),
        };

        let result = evaluator.evaluate().unwrap();

        assert_eq!(result.status, EvaluationStatus::Approved);
    }

    #[tokio::test]
    async fn misconfigured_min_approved_when_not_enough_approvers_should_still_approve() {
        let mut request = mock_request();
        let mut policy = mock_request_policy();
        let user = user_test_utils::add_user(&[1; 16]);

        request.operation = RequestOperation::AddUserGroup(AddUserGroupOperation {
            user_group_id: None,
            input: AddUserGroupOperationInput {
                name: "test".to_string(),
            },
        });
        request.requested_by = user.id;
        request.approvals = vec![mock_approved_with_user(user.id)];

        REQUEST_REPOSITORY.insert(request.to_key(), request.clone());

        policy.specifier = RequestSpecifier::AddUserGroup;
        policy.rule = RequestPolicyRule::Quorum(UserSpecifier::Any, 2);

        REQUEST_POLICY_REPOSITORY.insert(policy.id, policy.clone());

        let evaluator = RequestEvaluator {
            request: request.to_owned(),
            policy_rule_evaluator: REQUEST_POLICY_RULE_EVALUATOR.to_owned(),
        };

        let result = evaluator.evaluate().unwrap();

        assert_eq!(result.status, EvaluationStatus::Approved);
    }

    #[tokio::test]
    async fn returns_correct_evaluation_result() {
        let mut request = mock_request();
        request.status = RequestStatus::Created;

        REQUEST_REPOSITORY.insert(request.to_key(), request.clone());

        let mut user_1 = mock_user();
        user_1.identities = vec![Principal::from_slice(&[1; 29])];
        user_1.groups.push(*ADMIN_GROUP_ID);
        USER_REPOSITORY.insert(user_1.to_key(), user_1.clone());

        let mut user_2 = mock_user();
        user_2.identities = vec![Principal::from_slice(&[2; 29])];
        user_2.groups.push(*ADMIN_GROUP_ID);
        USER_REPOSITORY.insert(user_2.to_key(), user_2.clone());

        let mut user_3 = mock_user();
        user_3.identities = vec![Principal::from_slice(&[3; 29])];
        user_3.groups.push(*ADMIN_GROUP_ID);
        USER_REPOSITORY.insert(user_3.to_key(), user_3.clone());

        ACCOUNT_REPOSITORY.insert(
            AccountKey { id: [1; 16] },
            Account {
                id: [1; 16],
                addresses: vec![],
                assets: vec![],
                seed: [0; 16],
                name: "test".to_owned(),
                metadata: Metadata::default(),
                transfer_request_policy_id: None,
                configs_request_policy_id: None,
                last_modification_timestamp: 0,
            },
        );

        let result = request
            .reevaluate()
            .await
            .expect("failed to reevaluate request")
            .expect("request state is not Created");

        // no policies affecting the request means it should be rejected
        assert!(result.policy_results.is_empty());
        assert_eq!(result.status, EvaluationStatus::Rejected);

        // add policy
        REQUEST_POLICY_REPOSITORY.insert(
            [1; 16],
            RequestPolicy {
                id: [1; 16],
                rule: RequestPolicyRule::Or(vec![
                    RequestPolicyRule::Quorum(
                        UserSpecifier::Id(vec![user_1.id, user_2.id, user_3.id]),
                        1,
                    ),
                    RequestPolicyRule::AllowListedByMetadata(MetadataItem {
                        key: "test".to_string(),
                        value: "test".to_string(),
                    }),
                ]),
                specifier: RequestSpecifier::Transfer(ResourceIds::Any),
            },
        );

        request.status = RequestStatus::Created;

        let result = request
            .reevaluate()
            .await
            .expect("failed to reevaluate request")
            .expect("request state is not Created");

        // 2 policies affecting the request
        assert_eq!(result.policy_results.len(), 1);
        // no approvals yet, so it should be pending
        assert_eq!(result.status, EvaluationStatus::Pending);

        assert_eq!(
            result.policy_results[0],
            RequestPolicyRuleResult {
                status: EvaluationStatus::Pending,
                evaluated_rule: EvaluatedRequestPolicyRule::Or(vec![
                    RequestPolicyRuleResult {
                        status: EvaluationStatus::Pending,
                        evaluated_rule: EvaluatedRequestPolicyRule::Quorum {
                            min_approved: 1,
                            approvers: vec![],
                            total_possible_approvers: 3,
                        }
                    },
                    RequestPolicyRuleResult {
                        status: EvaluationStatus::Rejected,
                        evaluated_rule: EvaluatedRequestPolicyRule::AllowListedByMetadata {
                            metadata: MetadataItem {
                                key: "test".to_string(),
                                value: "test".to_string()
                            }
                        }
                    }
                ])
            }
        );

        set_mock_caller(user_1.identities.first().unwrap().to_owned());

        RequestService::default()
            .submit_request_approval(
                SubmitRequestApprovalInput {
                    request_id: Uuid::from_bytes(request.id).hyphenated().to_string(),
                    decision: RequestApprovalStatusDTO::Approved,
                    reason: None,
                },
                &call_context(),
            )
            .await
            .expect("failed to approve on request");

        // After voting the result should be stored in the repository
        let evaluation = REQUEST_EVALUATION_RESULT_REPOSITORY
            .get(&request.id)
            .unwrap();

        assert_eq!(evaluation.status, EvaluationStatus::Approved);

        assert_eq!(
            evaluation.policy_results[0],
            RequestPolicyRuleResult {
                status: EvaluationStatus::Approved,
                evaluated_rule: EvaluatedRequestPolicyRule::Or(vec![
                    RequestPolicyRuleResult {
                        status: EvaluationStatus::Approved,
                        evaluated_rule: EvaluatedRequestPolicyRule::Quorum {
                            min_approved: 1,
                            approvers: vec![user_1.id,],
                            total_possible_approvers: 3,
                        }
                    },
                    RequestPolicyRuleResult {
                        status: EvaluationStatus::Rejected,
                        evaluated_rule: EvaluatedRequestPolicyRule::AllowListedByMetadata {
                            metadata: MetadataItem {
                                key: "test".to_string(),
                                value: "test".to_string()
                            }
                        }
                    }
                ])
            }
        );
    }
}
