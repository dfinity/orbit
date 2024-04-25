use super::evaluation::Evaluate;
use crate::{
    errors::EvaluateError,
    models::{
        criteria::{Criteria, CriteriaResult, EvaluateCriteria, ProposalEvaluationResult},
        specifier::{
            Match, ProposalSpecifier, UserInvolvedInCriteriaForProposalResource, UserSpecifier,
        },
        EvaluationStatus, Proposal, ProposalId, ProposalOperation, ProposalStatusCode, User,
        UserId, UserStatus,
    },
    repositories::{policy::PROPOSAL_POLICY_REPOSITORY, PROPOSAL_REPOSITORY, USER_REPOSITORY},
};
use anyhow::Context;
use ic_canister_core::{repository::Repository, types::UUID};
use std::{collections::HashSet, sync::Arc};

pub struct ProposalEvaluator {
    pub proposal_matcher: Arc<dyn Match<(Proposal, ProposalSpecifier)>>,
    pub criteria_evaluator: Arc<dyn EvaluateCriteria<CriteriaResult>>,
    pub proposal: Proposal,
}

impl ProposalEvaluator {
    pub fn new(
        proposal_matcher: Arc<dyn Match<(Proposal, ProposalSpecifier)>>,
        criteria_evaluator: Arc<dyn EvaluateCriteria<CriteriaResult>>,
        proposal: Proposal,
    ) -> Self {
        Self {
            proposal_matcher,
            criteria_evaluator,
            proposal,
        }
    }
}

impl Evaluate<ProposalEvaluationResult> for ProposalEvaluator {
    fn evaluate(&self) -> Result<ProposalEvaluationResult, EvaluateError> {
        let matching_policies = self
            .proposal
            .operation
            .to_resources()
            .iter()
            .flat_map(|resource| PROPOSAL_POLICY_REPOSITORY.find_by_resource(resource.to_owned()))
            .collect::<Vec<_>>();

        if matching_policies.is_empty() {
            // Since proposals handle security critical operations, we want to reject them by default if
            // they don't match any policy. Users need to explicitly add the necessary policies to evaluate them.
            return Ok(ProposalEvaluationResult {
                proposal_id: self.proposal.id,
                status: EvaluationStatus::Rejected,
                policy_results: vec![],
            });
        }

        let proposal = Arc::new(self.proposal.to_owned());
        let mut evaluation_statuses = Vec::new();

        // Evaluate all matching policies to get the full evaluation result.
        for policy in matching_policies {
            // Evaluate the criteria
            let evaluation_status = self
                .criteria_evaluator
                .evaluate((proposal.to_owned(), Arc::new(policy.criteria)))
                .context("failed to evaluate criteria")?;

            evaluation_statuses.push(evaluation_status);
        }

        Ok(ProposalEvaluationResult {
            proposal_id: self.proposal.id,
            status: {
                if evaluation_statuses
                    .iter()
                    .any(|result| result.status == EvaluationStatus::Adopted)
                {
                    // If any policy adopted the proposal, then the proposal is adopted.
                    EvaluationStatus::Adopted
                } else if evaluation_statuses
                    .iter()
                    .all(|result| result.status == EvaluationStatus::Rejected)
                {
                    // Only if all policies are rejected then the proposal is rejected,
                    // this applies an implicit `OR` between policies.
                    EvaluationStatus::Rejected
                } else {
                    // Since there are matching policies, but none of them adopted or rejected the proposal, we keep it in the
                    // pending status until one of the policies evaluates it as adopted or rejected.
                    EvaluationStatus::Pending
                }
            },
            policy_results: evaluation_statuses,
        })
    }
}

#[derive(Debug, Clone, Default)]
pub struct PossibleVoters {
    pub match_all: bool,
    pub users: HashSet<UUID>,
    pub groups: HashSet<UUID>,
}

/// Evaluates all possible voters for the proposal.
///
/// The possible voters are the users that match the proposal's policies.
pub struct ProposalPossibleVotersFinder<'p> {
    pub proposal_matcher: Arc<dyn Match<(Proposal, ProposalSpecifier)>>,
    pub possible_voters_criteria_evaluator:
        Arc<dyn EvaluateCriteria<PossibleVoters, (Arc<Proposal>, Arc<Criteria>), EvaluateError>>,
    pub proposal: &'p Proposal,
}

impl<'p> ProposalPossibleVotersFinder<'p> {
    pub fn new(
        proposal_matcher: Arc<dyn Match<(Proposal, ProposalSpecifier)>>,
        possible_voters_criteria_evaluator: Arc<
            dyn EvaluateCriteria<PossibleVoters, (Arc<Proposal>, Arc<Criteria>), EvaluateError>,
        >,
        proposal: &'p Proposal,
    ) -> Self {
        Self {
            proposal_matcher,
            possible_voters_criteria_evaluator,
            proposal,
        }
    }
}

impl Evaluate<HashSet<UUID>> for ProposalPossibleVotersFinder<'_> {
    fn evaluate(&self) -> Result<HashSet<UUID>, EvaluateError> {
        let mut possible_voters = HashSet::new();
        let mut matching_groups = HashSet::new();
        let mut matching_policies = Vec::new();
        for policy in PROPOSAL_POLICY_REPOSITORY.list() {
            if self
                .proposal_matcher
                .is_match((self.proposal.to_owned(), policy.specifier.to_owned()))
                .context("failed to match proposal")?
            {
                matching_policies.push(policy.to_owned());
            }
        }

        for policy in matching_policies {
            let result = self.possible_voters_criteria_evaluator.evaluate((
                Arc::new(self.proposal.to_owned()),
                Arc::new(policy.criteria.to_owned()),
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
                possible_voters.extend(
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

            possible_voters.extend(users);
        }

        Ok(possible_voters)
    }
}

pub struct ProposalPossibleVotersCriteriaEvaluator;

impl EvaluateCriteria<PossibleVoters, (Arc<Proposal>, Arc<Criteria>), EvaluateError>
    for ProposalPossibleVotersCriteriaEvaluator
{
    fn evaluate(
        &self,
        (proposal, criteria): (Arc<Proposal>, Arc<Criteria>),
    ) -> Result<PossibleVoters, EvaluateError> {
        let mut possible_voters = PossibleVoters::default();
        match criteria.as_ref() {
            Criteria::ApprovalThreshold(voter_specifier, _)
            | Criteria::MinimumVotes(voter_specifier, _) => match voter_specifier {
                UserSpecifier::Any => {
                    possible_voters.match_all = true;

                    Ok(possible_voters)
                }
                UserSpecifier::Id(user_ids) => {
                    possible_voters.users.extend(user_ids.to_owned());

                    Ok(possible_voters)
                }
                UserSpecifier::Group(group_ids) => {
                    possible_voters.groups.extend(group_ids.to_owned());

                    Ok(possible_voters)
                }
                UserSpecifier::Proposer => {
                    possible_voters
                        .users
                        .insert(proposal.proposed_by.to_owned());

                    Ok(possible_voters)
                }
                UserSpecifier::Owner => {
                    match &proposal.operation {
                        ProposalOperation::EditUser(operation) => {
                            possible_voters
                                .users
                                .insert(operation.input.user_id.to_owned());
                        }
                        ProposalOperation::EditAccount(_)
                        | ProposalOperation::Transfer(_)
                        | ProposalOperation::AddAccount(_)
                        | ProposalOperation::AddAddressBookEntry(_)
                        | ProposalOperation::AddProposalPolicy(_)
                        | ProposalOperation::AddUser(_)
                        | ProposalOperation::AddUserGroup(_)
                        | ProposalOperation::EditAddressBookEntry(_)
                        | ProposalOperation::RemoveAddressBookEntry(_)
                        | ProposalOperation::EditAccessPolicy(_)
                        | ProposalOperation::EditProposalPolicy(_)
                        | ProposalOperation::EditUserGroup(_)
                        | ProposalOperation::RemoveProposalPolicy(_)
                        | ProposalOperation::RemoveUserGroup(_)
                        | ProposalOperation::ChangeCanister(_) => {}
                    };

                    Ok(possible_voters)
                }
            },
            Criteria::HasAddressInAddressBook | Criteria::HasAddressBookMetadata(_) => {
                Ok(possible_voters)
            }
            Criteria::And(criterias) | Criteria::Or(criterias) => {
                for criteria in criterias.iter() {
                    let result = self.evaluate((proposal.clone(), Arc::new(criteria.clone())));

                    match result {
                        Ok(evaluated) => {
                            if evaluated.match_all {
                                possible_voters.match_all = true;
                                break;
                            }

                            possible_voters.users.extend(evaluated.users);
                            possible_voters.groups.extend(evaluated.groups);
                        }
                        Err(e) => return Err(e),
                    }
                }

                Ok(possible_voters)
            }
            Criteria::Not(criteria) => {
                let result =
                    self.evaluate((proposal.to_owned(), Arc::new(criteria.as_ref().to_owned())))?;

                if result.match_all {
                    possible_voters.match_all = true;
                }

                possible_voters.users.extend(result.users);
                possible_voters.groups.extend(result.groups);

                Ok(possible_voters)
            }
            Criteria::AutoAdopted => Ok(possible_voters),
        }
    }
}

/// Evaluates if the user has the right to vote on the proposal.
///
/// The user has the right to vote if:
///
/// - The proposal is not adopted or rejected
/// - There are matching policies for the proposal and the user is a part of the group that is allowed to vote
/// - The user has not already voted on the proposal
pub struct ProposalVoteRightsEvaluator {
    pub proposal_matcher: Arc<dyn Match<(Proposal, ProposalSpecifier)>>,
    pub vote_rights_evaluator: Arc<VoteRightsEvaluate>,
    pub voter_id: UserId,
    pub proposal_id: ProposalId,
}

pub type VoteRightsEvaluate =
    dyn EvaluateCriteria<bool, (Arc<ProposalId>, Arc<UserId>, Arc<Criteria>), EvaluateError>;

impl ProposalVoteRightsEvaluator {
    pub fn new(
        proposal_matcher: Arc<dyn Match<(Proposal, ProposalSpecifier)>>,
        vote_rights_evaluator: Arc<VoteRightsEvaluate>,
        voter_id: UserId,
        proposal_id: ProposalId,
    ) -> Self {
        Self {
            proposal_matcher,
            vote_rights_evaluator,
            voter_id,
            proposal_id,
        }
    }
}

impl Evaluate<bool> for ProposalVoteRightsEvaluator {
    fn evaluate(&self) -> Result<bool, EvaluateError> {
        if PROPOSAL_REPOSITORY.exists_voter(&self.proposal_id, &self.voter_id)
            || !PROPOSAL_REPOSITORY.exists_status(&self.proposal_id, ProposalStatusCode::Created)
        {
            return Ok(false);
        }

        let matching_policies = PROPOSAL_REPOSITORY
            .get_resources(&self.proposal_id)
            .iter()
            .flat_map(|resource| PROPOSAL_POLICY_REPOSITORY.find_by_resource(resource.to_owned()))
            .collect::<Vec<_>>();

        for policy in matching_policies {
            if self.vote_rights_evaluator.evaluate((
                Arc::new(self.proposal_id.to_owned()),
                Arc::new(self.voter_id),
                Arc::new(policy.criteria.to_owned()),
            ))? {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

pub struct ProposalVoteRightsCriteriaEvaluator {
    pub voter_matcher: Arc<dyn Match<UserInvolvedInCriteriaForProposalResource>>,
}

impl EvaluateCriteria<bool, (Arc<ProposalId>, Arc<UserId>, Arc<Criteria>), EvaluateError>
    for ProposalVoteRightsCriteriaEvaluator
{
    fn evaluate(
        &self,
        (proposal_id, voter_id, criteria): (Arc<ProposalId>, Arc<UserId>, Arc<Criteria>),
    ) -> Result<bool, EvaluateError> {
        match criteria.as_ref() {
            Criteria::ApprovalThreshold(voter_specifier, _)
            | Criteria::MinimumVotes(voter_specifier, _) => {
                let can_vote = self
                    .voter_matcher
                    .is_match(UserInvolvedInCriteriaForProposalResource {
                        proposal_operation_resources: PROPOSAL_REPOSITORY
                            .get_resources(&proposal_id),
                        policy_criteria_user_specifier: voter_specifier.to_owned(),
                        user_id: voter_id.as_ref().to_owned(),
                        proposal_id: proposal_id.as_ref().to_owned(),
                    })
                    .context("failed to match proposal voters")?;

                Ok(can_vote)
            }
            Criteria::HasAddressInAddressBook | Criteria::HasAddressBookMetadata(_) => Ok(false),
            Criteria::And(criterias) | Criteria::Or(criterias) => {
                let proposal = &proposal_id;
                let voter_id = &voter_id;

                for criteria in criterias.iter() {
                    let can_vote = self.evaluate((
                        proposal.to_owned(),
                        voter_id.to_owned(),
                        Arc::new(criteria.to_owned()),
                    ))?;

                    if can_vote {
                        return Ok(true);
                    }
                }

                Ok(false)
            }
            Criteria::Not(criteria) => {
                let can_vote = self.evaluate((
                    proposal_id.to_owned(),
                    voter_id.to_owned(),
                    Arc::new(criteria.as_ref().to_owned()),
                ))?;

                Ok(can_vote)
            }
            Criteria::AutoAdopted => Ok(false),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        core::{
            evaluation::{CRITERIA_EVALUATOR, PROPOSAL_MATCHER},
            middlewares::call_context,
            set_mock_caller,
        },
        models::{
            criteria::Percentage,
            proposal_policy_test_utils::mock_proposal_policy,
            proposal_test_utils::mock_proposal,
            proposal_vote_test_utils::{mock_accepted_with_user, mock_rejected_with_user},
            resource::ResourceIds,
            user_test_utils::{self, mock_user},
            Account, AccountKey, AddUserGroupOperation, AddUserGroupOperationInput, Blockchain,
            BlockchainStandard, EvaluatedCriteria, Metadata, MetadataItem, ProposalPolicy,
            ProposalStatus, ADMIN_GROUP_ID,
        },
        repositories::{
            policy::PROPOSAL_POLICY_REPOSITORY, ACCOUNT_REPOSITORY, EVALUATION_RESULT_REPOSITORY,
            PROPOSAL_REPOSITORY,
        },
        services::ProposalService,
    };
    use candid::Principal;
    use ic_canister_core::repository::Repository;
    use uuid::Uuid;
    use wallet_api::VoteOnProposalInput;

    #[tokio::test]
    async fn is_rejected_when_no_policy_is_available() {
        let proposal = mock_proposal();

        PROPOSAL_REPOSITORY.insert(proposal.to_key(), proposal.clone());

        let evaluator = ProposalEvaluator {
            proposal: proposal.to_owned(),
            proposal_matcher: PROPOSAL_MATCHER.to_owned(),
            criteria_evaluator: CRITERIA_EVALUATOR.to_owned(),
        };

        let result = evaluator.evaluate().unwrap();

        assert_eq!(result.status, EvaluationStatus::Rejected);
    }

    #[tokio::test]
    async fn succeeds_when_all_criterias_are_adopted() {
        let mut proposal = mock_proposal();
        let mut policy = mock_proposal_policy();
        let user = user_test_utils::add_user(&[1; 16]);

        proposal.operation = ProposalOperation::AddUserGroup(AddUserGroupOperation {
            user_group_id: None,
            input: AddUserGroupOperationInput {
                name: "test".to_string(),
            },
        });
        proposal.proposed_by = user.id;
        proposal.votes = vec![mock_accepted_with_user(user.id)];

        PROPOSAL_REPOSITORY.insert(proposal.to_key(), proposal.clone());

        policy.specifier = ProposalSpecifier::AddUserGroup;
        policy.criteria = Criteria::ApprovalThreshold(UserSpecifier::Proposer, Percentage(100));

        PROPOSAL_POLICY_REPOSITORY.insert(policy.id, policy.clone());

        let evaluator = ProposalEvaluator {
            proposal: proposal.to_owned(),
            proposal_matcher: PROPOSAL_MATCHER.to_owned(),
            criteria_evaluator: CRITERIA_EVALUATOR.to_owned(),
        };

        let result = evaluator.evaluate().unwrap();

        assert_eq!(result.status, EvaluationStatus::Adopted);
    }

    #[tokio::test]
    async fn is_pending_when_votes_are_not_reached() {
        let mut proposal = mock_proposal();
        let mut policy = mock_proposal_policy();
        let user = user_test_utils::add_user(&[1; 16]);
        user_test_utils::add_user(&[2; 16]);

        proposal.operation = ProposalOperation::AddUserGroup(AddUserGroupOperation {
            user_group_id: None,
            input: AddUserGroupOperationInput {
                name: "test".to_string(),
            },
        });
        proposal.proposed_by = user.id;
        proposal.votes = vec![mock_accepted_with_user(user.id)];

        PROPOSAL_REPOSITORY.insert(proposal.to_key(), proposal.clone());

        policy.specifier = ProposalSpecifier::AddUserGroup;
        policy.criteria = Criteria::ApprovalThreshold(UserSpecifier::Any, Percentage(100));

        PROPOSAL_POLICY_REPOSITORY.insert(policy.id, policy.clone());

        let evaluator = ProposalEvaluator {
            proposal: proposal.to_owned(),
            proposal_matcher: PROPOSAL_MATCHER.to_owned(),
            criteria_evaluator: CRITERIA_EVALUATOR.to_owned(),
        };

        let result = evaluator.evaluate().unwrap();

        assert_eq!(result.status, EvaluationStatus::Pending);
    }

    #[tokio::test]
    async fn is_rejected_when_votes_are_not_reached() {
        let mut proposal = mock_proposal();
        let mut policy = mock_proposal_policy();
        let user = user_test_utils::add_user(&[1; 16]);
        let user1 = user_test_utils::add_user(&[2; 16]);

        proposal.operation = ProposalOperation::AddUserGroup(AddUserGroupOperation {
            user_group_id: None,
            input: AddUserGroupOperationInput {
                name: "test".to_string(),
            },
        });
        proposal.proposed_by = user.id;
        proposal.votes = vec![
            mock_accepted_with_user(user.id),
            mock_rejected_with_user(user1.id),
        ];

        PROPOSAL_REPOSITORY.insert(proposal.to_key(), proposal.clone());

        policy.specifier = ProposalSpecifier::AddUserGroup;
        policy.criteria = Criteria::ApprovalThreshold(UserSpecifier::Any, Percentage(100));

        PROPOSAL_POLICY_REPOSITORY.insert(policy.id, policy.clone());

        let evaluator = ProposalEvaluator {
            proposal: proposal.to_owned(),
            proposal_matcher: PROPOSAL_MATCHER.to_owned(),
            criteria_evaluator: CRITERIA_EVALUATOR.to_owned(),
        };

        let result = evaluator.evaluate().unwrap();

        assert_eq!(result.status, EvaluationStatus::Rejected);
    }

    #[tokio::test]
    async fn is_accepted_disregarding_inactive_users() {
        let mut proposal = mock_proposal();
        let mut policy = mock_proposal_policy();
        let user = user_test_utils::add_user(&[1; 16]);
        user_test_utils::add_inactive_user(&[2; 16]);

        proposal.operation = ProposalOperation::AddUserGroup(AddUserGroupOperation {
            user_group_id: None,
            input: AddUserGroupOperationInput {
                name: "test".to_string(),
            },
        });
        proposal.proposed_by = user.id;
        proposal.votes = vec![mock_accepted_with_user(user.id)];

        PROPOSAL_REPOSITORY.insert(proposal.to_key(), proposal.clone());

        policy.specifier = ProposalSpecifier::AddUserGroup;
        policy.criteria = Criteria::ApprovalThreshold(UserSpecifier::Any, Percentage(100));

        PROPOSAL_POLICY_REPOSITORY.insert(policy.id, policy.clone());

        let evaluator = ProposalEvaluator {
            proposal: proposal.to_owned(),
            proposal_matcher: PROPOSAL_MATCHER.to_owned(),
            criteria_evaluator: CRITERIA_EVALUATOR.to_owned(),
        };

        let result = evaluator.evaluate().unwrap();

        assert_eq!(result.status, EvaluationStatus::Adopted);
    }

    #[tokio::test]
    async fn misconfigured_min_votes_when_not_enough_voters_should_still_adopt() {
        let mut proposal = mock_proposal();
        let mut policy = mock_proposal_policy();
        let user = user_test_utils::add_user(&[1; 16]);

        proposal.operation = ProposalOperation::AddUserGroup(AddUserGroupOperation {
            user_group_id: None,
            input: AddUserGroupOperationInput {
                name: "test".to_string(),
            },
        });
        proposal.proposed_by = user.id;
        proposal.votes = vec![mock_accepted_with_user(user.id)];

        PROPOSAL_REPOSITORY.insert(proposal.to_key(), proposal.clone());

        policy.specifier = ProposalSpecifier::AddUserGroup;
        policy.criteria = Criteria::MinimumVotes(UserSpecifier::Any, 2);

        PROPOSAL_POLICY_REPOSITORY.insert(policy.id, policy.clone());

        let evaluator = ProposalEvaluator {
            proposal: proposal.to_owned(),
            proposal_matcher: PROPOSAL_MATCHER.to_owned(),
            criteria_evaluator: CRITERIA_EVALUATOR.to_owned(),
        };

        let result = evaluator.evaluate().unwrap();

        assert_eq!(result.status, EvaluationStatus::Adopted);
    }

    #[tokio::test]
    async fn returns_correct_evaluation_result() {
        let mut proposal = mock_proposal();
        proposal.status = ProposalStatus::Created;

        PROPOSAL_REPOSITORY.insert(proposal.to_key(), proposal.clone());

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
                blockchain: Blockchain::InternetComputer,
                address: "a".to_owned(),
                standard: BlockchainStandard::Native,
                symbol: "S".to_owned(),
                decimals: 1,
                name: "test".to_owned(),
                balance: None,
                metadata: Metadata::default(),
                transfer_approval_policy_id: None,
                update_approval_policy_id: None,
                last_modification_timestamp: 0,
            },
        );

        let result = proposal
            .reevaluate()
            .await
            .expect("failed to reevaluate proposal")
            .expect("proposal state is not Created");

        // no policies affecting the proposal means it should be rejected
        assert!(result.policy_results.is_empty());
        assert_eq!(result.status, EvaluationStatus::Rejected);

        // add policy
        PROPOSAL_POLICY_REPOSITORY.insert(
            [1; 16],
            ProposalPolicy {
                id: [1; 16],
                criteria: Criteria::Or(vec![
                    Criteria::MinimumVotes(
                        UserSpecifier::Id(vec![user_1.id, user_2.id, user_3.id]),
                        1,
                    ),
                    Criteria::HasAddressBookMetadata(MetadataItem {
                        key: "test".to_string(),
                        value: "test".to_string(),
                    }),
                ]),
                specifier: ProposalSpecifier::Transfer(ResourceIds::Any),
            },
        );

        proposal.status = ProposalStatus::Created;

        let result = proposal
            .reevaluate()
            .await
            .expect("failed to reevaluate proposal")
            .expect("proposal state is not Created");

        // 2 policies affecting the proposal
        assert_eq!(result.policy_results.len(), 1);
        // no votes yet, so it should be pending
        assert_eq!(result.status, EvaluationStatus::Pending);

        assert_eq!(
            result.policy_results[0],
            CriteriaResult {
                status: EvaluationStatus::Pending,
                evaluated_criteria: EvaluatedCriteria::Or(vec![
                    CriteriaResult {
                        status: EvaluationStatus::Pending,
                        evaluated_criteria: EvaluatedCriteria::MinimumVotes {
                            min_required_votes: 1,
                            votes: vec![],
                            total_possible_votes: 3,
                        }
                    },
                    CriteriaResult {
                        status: EvaluationStatus::Rejected,
                        evaluated_criteria: EvaluatedCriteria::HasAddressBookMetadata {
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

        ProposalService::default()
            .vote_on_proposal(
                VoteOnProposalInput {
                    proposal_id: Uuid::from_bytes(proposal.id).hyphenated().to_string(),
                    approve: true,
                    reason: None,
                },
                &call_context(),
            )
            .await
            .expect("failed to vote on proposal");

        // After voting the result should be stored in the repository
        let evaluation = EVALUATION_RESULT_REPOSITORY.get(&proposal.id).unwrap();

        assert_eq!(evaluation.status, EvaluationStatus::Adopted);

        assert_eq!(
            evaluation.policy_results[0],
            CriteriaResult {
                status: EvaluationStatus::Adopted,
                evaluated_criteria: EvaluatedCriteria::Or(vec![
                    CriteriaResult {
                        status: EvaluationStatus::Adopted,
                        evaluated_criteria: EvaluatedCriteria::MinimumVotes {
                            min_required_votes: 1,
                            votes: vec![user_1.id,],
                            total_possible_votes: 3,
                        }
                    },
                    CriteriaResult {
                        status: EvaluationStatus::Rejected,
                        evaluated_criteria: EvaluatedCriteria::HasAddressBookMetadata {
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
