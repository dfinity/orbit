use super::evaluation::Evaluate;
use crate::{
    errors::EvaluateError,
    models::{
        criteria::{Criteria, EvaluateCriteria},
        specifier::{Match, ProposalSpecifier, UserSpecifier},
        Account, EvaluationStatus, Proposal, ProposalOperation, ProposalStatus, User, UserId,
        UserStatus,
    },
    repositories::{policy::PROPOSAL_POLICY_REPOSITORY, ACCOUNT_REPOSITORY, USER_REPOSITORY},
};
use anyhow::Context;
use async_trait::async_trait;
use futures::{stream, StreamExt, TryStreamExt};
use ic_canister_core::{repository::Repository, types::UUID};
use std::{collections::HashSet, sync::Arc};

pub struct ProposalEvaluator {
    pub proposal_matcher: Arc<dyn Match<(Proposal, ProposalSpecifier)>>,
    pub criteria_evaluator: Arc<dyn EvaluateCriteria>,
    pub proposal: Proposal,
}

impl ProposalEvaluator {
    pub fn new(
        proposal_matcher: Arc<dyn Match<(Proposal, ProposalSpecifier)>>,
        criteria_evaluator: Arc<dyn EvaluateCriteria>,
        proposal: Proposal,
    ) -> Self {
        Self {
            proposal_matcher,
            criteria_evaluator,
            proposal,
        }
    }
}

#[async_trait]
impl Evaluate<EvaluationStatus> for ProposalEvaluator {
    async fn evaluate(&self) -> Result<EvaluationStatus, EvaluateError> {
        let mut matching_policies = Vec::new();
        for policy in PROPOSAL_POLICY_REPOSITORY.list() {
            if self
                .proposal_matcher
                .is_match((self.proposal.to_owned(), policy.specifier.to_owned()))
                .await
                .context("failed to match proposal")?
            {
                matching_policies.push(policy.to_owned());
            }
        }

        if matching_policies.is_empty() {
            // Since proposals handle security critical operations, we want to reject them by default if
            // they don't match any policy. Users need to explicitly add the necessary policies to evaluate them.
            return Ok(EvaluationStatus::Rejected);
        }

        let proposal = Arc::new(self.proposal.to_owned());
        let mut evaluation_statuses = Vec::new();

        for policy in matching_policies {
            // Evaluate the criteria
            let evaluation_status = self
                .criteria_evaluator
                .evaluate((proposal.to_owned(), Arc::new(policy.criteria)))
                .await
                .context("failed to evaluate criteria")?;

            evaluation_statuses.push(evaluation_status.to_owned());

            if let EvaluationStatus::Adopted = evaluation_status {
                return Ok(evaluation_status);
            }
        }

        // Only if all policies are rejected then the proposal is rejected,
        // this applies an implicit `OR` between policies.
        if evaluation_statuses
            .iter()
            .all(|status| *status == EvaluationStatus::Rejected)
        {
            return Ok(EvaluationStatus::Rejected);
        }

        // Since there are matching policies, but none of them adopted or rejected the proposal, we keep it in the
        // pending status until one of the policies evaluates it as adopted or rejected.
        Ok(EvaluationStatus::Pending)
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

#[async_trait]
impl Evaluate<HashSet<UUID>> for ProposalPossibleVotersFinder<'_> {
    async fn evaluate(&self) -> Result<HashSet<UUID>, EvaluateError> {
        let mut possible_voters = HashSet::new();
        let mut matching_groups = HashSet::new();
        let mut matching_policies = Vec::new();
        for policy in PROPOSAL_POLICY_REPOSITORY.list() {
            if self
                .proposal_matcher
                .is_match((self.proposal.to_owned(), policy.specifier.to_owned()))
                .await
                .context("failed to match proposal")?
            {
                matching_policies.push(policy.to_owned());
            }
        }

        for policy in matching_policies {
            let result = self
                .possible_voters_criteria_evaluator
                .evaluate((
                    Arc::new(self.proposal.to_owned()),
                    Arc::new(policy.criteria.to_owned()),
                ))
                .await?;

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

#[async_trait]
impl EvaluateCriteria<PossibleVoters, (Arc<Proposal>, Arc<Criteria>), EvaluateError>
    for ProposalPossibleVotersCriteriaEvaluator
{
    async fn evaluate(
        &self,
        (proposal, criteria): (Arc<Proposal>, Arc<Criteria>),
    ) -> Result<PossibleVoters, EvaluateError> {
        let mut possible_voters = PossibleVoters::default();
        match criteria.as_ref() {
            Criteria::ApprovalThreshold(voter_specifier, _)
            | Criteria::MinimumVotes(voter_specifier, _) => {
                match voter_specifier {
                    UserSpecifier::Any => {
                        possible_voters.match_all = true;

                        return Ok(possible_voters);
                    }
                    UserSpecifier::Id(user_ids) => {
                        possible_voters.users.extend(user_ids.to_owned());

                        return Ok(possible_voters);
                    }
                    UserSpecifier::Group(group_ids) => {
                        possible_voters.groups.extend(group_ids.to_owned());

                        return Ok(possible_voters);
                    }
                    UserSpecifier::Proposer => {
                        possible_voters
                            .users
                            .insert(proposal.proposed_by.to_owned());

                        return Ok(possible_voters);
                    }
                    UserSpecifier::Owner => {
                        match &proposal.operation {
                            ProposalOperation::Transfer(operation) => {
                                if let Some(account) = ACCOUNT_REPOSITORY
                                    .get(&Account::key(operation.input.from_account_id))
                                {
                                    possible_voters.users.extend(account.owners.to_owned());
                                }
                            }
                            ProposalOperation::EditUser(operation) => {
                                possible_voters
                                    .users
                                    .insert(operation.input.user_id.to_owned());
                            }
                            ProposalOperation::EditAccount(operation) => {
                                if let Some(account) = ACCOUNT_REPOSITORY
                                    .get(&Account::key(operation.input.account_id))
                                {
                                    possible_voters.users.extend(account.owners.to_owned());
                                }
                            }
                            ProposalOperation::AddAccount(_)
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

                        return Ok(possible_voters);
                    }
                };
            }
            Criteria::HasAddressBookMetadata(_) => Ok(possible_voters),
            Criteria::And(criterias) | Criteria::Or(criterias) => {
                for criteria in criterias.iter() {
                    let result = self
                        .evaluate((proposal.clone(), Arc::new(criteria.clone())))
                        .await;

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
                let result = self
                    .evaluate((proposal.to_owned(), Arc::new(criteria.as_ref().to_owned())))
                    .await?;

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
pub struct ProposalVoteRightsEvaluator<'p> {
    pub proposal_matcher: Arc<dyn Match<(Proposal, ProposalSpecifier)>>,
    pub vote_rights_evaluator: Arc<VoteRightsEvaluate>,
    pub voter_id: UserId,
    pub proposal: &'p Proposal,
}

pub type VoteRightsEvaluate =
    dyn EvaluateCriteria<bool, (Arc<Proposal>, Arc<UserId>, Arc<Criteria>), EvaluateError>;

impl<'p> ProposalVoteRightsEvaluator<'p> {
    pub fn new(
        proposal_matcher: Arc<dyn Match<(Proposal, ProposalSpecifier)>>,
        vote_rights_evaluator: Arc<VoteRightsEvaluate>,
        voter_id: UserId,
        proposal: &'p Proposal,
    ) -> Self {
        Self {
            proposal_matcher,
            vote_rights_evaluator,
            voter_id,
            proposal,
        }
    }
}

#[async_trait]
impl Evaluate<bool> for ProposalVoteRightsEvaluator<'_> {
    async fn evaluate(&self) -> Result<bool, EvaluateError> {
        if self.proposal.voters().contains(&self.voter_id)
            || self.proposal.status != ProposalStatus::Created
        {
            return Ok(false);
        }

        let mut matching_policies = Vec::new();
        for policy in PROPOSAL_POLICY_REPOSITORY.list() {
            if self
                .proposal_matcher
                .is_match((self.proposal.to_owned(), policy.specifier.to_owned()))
                .await
                .context("failed to match proposal")?
            {
                matching_policies.push(policy.to_owned());
            }
        }

        for policy in matching_policies {
            if self
                .vote_rights_evaluator
                .evaluate((
                    Arc::new(self.proposal.to_owned()),
                    Arc::new(self.voter_id),
                    Arc::new(policy.criteria.to_owned()),
                ))
                .await?
            {
                return Ok(true);
            }
        }

        Ok(false)
    }
}

pub struct ProposalVoteRightsCriteriaEvaluator {
    pub voter_matcher: Arc<dyn Match<(Proposal, UserId, UserSpecifier)>>,
}

#[async_trait]
impl EvaluateCriteria<bool, (Arc<Proposal>, Arc<UserId>, Arc<Criteria>), EvaluateError>
    for ProposalVoteRightsCriteriaEvaluator
{
    async fn evaluate(
        &self,
        (proposal, voter_id, criteria): (Arc<Proposal>, Arc<UserId>, Arc<Criteria>),
    ) -> Result<bool, EvaluateError> {
        match criteria.as_ref() {
            Criteria::ApprovalThreshold(voter_specifier, _)
            | Criteria::MinimumVotes(voter_specifier, _) => {
                let can_vote = self
                    .voter_matcher
                    .is_match((
                        proposal.as_ref().to_owned(),
                        voter_id.as_ref().to_owned(),
                        voter_specifier.to_owned(),
                    ))
                    .await
                    .context("failed to match proposal voters")?;

                Ok(can_vote)
            }
            Criteria::HasAddressBookMetadata(_) => Ok(false),
            Criteria::And(criterias) | Criteria::Or(criterias) => {
                let proposal = &proposal;
                let voter_id = &voter_id;
                let vote_evaluations = stream::iter(criterias.iter())
                    .then(|criteria| async move {
                        self.evaluate((
                            proposal.to_owned(),
                            voter_id.to_owned(),
                            Arc::new(criteria.to_owned()),
                        ))
                        .await
                    })
                    .try_collect::<Vec<bool>>()
                    .await?;

                Ok(vote_evaluations.contains(&true))
            }
            Criteria::Not(criteria) => {
                let can_vote = self
                    .evaluate((
                        proposal.to_owned(),
                        voter_id.to_owned(),
                        Arc::new(criteria.as_ref().to_owned()),
                    ))
                    .await?;

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
        core::evaluation::{CRITERIA_EVALUATOR, PROPOSAL_MATCHER},
        models::{
            criteria::Percentage,
            proposal_policy_test_utils::mock_proposal_policy,
            proposal_test_utils::mock_proposal,
            proposal_vote_test_utils::{mock_accepted_with_user, mock_rejected_with_user},
            user_test_utils, AddUserGroupOperation, AddUserGroupOperationInput,
        },
        repositories::{policy::PROPOSAL_POLICY_REPOSITORY, PROPOSAL_REPOSITORY},
    };
    use ic_canister_core::repository::Repository;

    #[tokio::test]
    async fn is_rejected_when_no_policy_is_available() {
        let proposal = mock_proposal();

        PROPOSAL_REPOSITORY.insert(proposal.to_key(), proposal.clone());

        let evaluator = ProposalEvaluator {
            proposal: proposal.to_owned(),
            proposal_matcher: PROPOSAL_MATCHER.to_owned(),
            criteria_evaluator: CRITERIA_EVALUATOR.to_owned(),
        };

        let evaluation_status = evaluator.evaluate().await.unwrap();

        assert_eq!(evaluation_status, EvaluationStatus::Rejected);
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

        let evaluation_status = evaluator.evaluate().await.unwrap();

        assert_eq!(evaluation_status, EvaluationStatus::Adopted);
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

        let evaluation_status = evaluator.evaluate().await.unwrap();

        assert_eq!(evaluation_status, EvaluationStatus::Pending);
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

        let evaluation_status = evaluator.evaluate().await.unwrap();

        assert_eq!(evaluation_status, EvaluationStatus::Rejected);
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

        let evaluation_status = evaluator.evaluate().await.unwrap();

        assert_eq!(evaluation_status, EvaluationStatus::Adopted);
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

        let evaluation_status = evaluator.evaluate().await.unwrap();

        assert_eq!(evaluation_status, EvaluationStatus::Adopted);
    }
}
