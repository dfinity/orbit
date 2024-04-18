use super::{
    specifier::{
        Match, ProposalHasMetadata, UserInvolvedInCriteriaForProposalResource, UserSpecifier,
    },
    EvaluateError, EvaluationStatus, MetadataItem, Proposal, ProposalOperation, ProposalVoteStatus,
    UserId, UserStatus,
};
use crate::{
    core::{ic_cdk::api::print, utils::calculate_minimum_threshold},
    errors::{MatchError, PolicyError},
    repositories::{UserWhereClause, ADDRESS_BOOK_REPOSITORY, USER_REPOSITORY},
    services::ACCOUNT_SERVICE,
};
use anyhow::{anyhow, Error};
use ic_canister_core::model::{ModelValidator, ModelValidatorResult};
use ic_canister_macros::storable;
use std::sync::Arc;
use std::{cmp, hash::Hash};

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Percentage(pub u16);

impl TryFrom<u16> for Percentage {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value > 100 {
            return Err(anyhow!(
                "invalid percentage value, must be between >= 0 and <= 100"
            ));
        }

        Ok(Percentage(value))
    }
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Criteria {
    // Auto
    AutoAdopted,
    // Votes
    ApprovalThreshold(UserSpecifier, Percentage),
    MinimumVotes(UserSpecifier, u16),
    HasAddressBookMetadata(MetadataItem),
    HasAddressInAddressBook,
    // Logical
    Or(Vec<Criteria>),
    And(Vec<Criteria>),
    Not(Box<Criteria>),
}

impl ModelValidator<PolicyError> for Criteria {
    fn validate(&self) -> ModelValidatorResult<PolicyError> {
        match self {
            Criteria::AutoAdopted
            | Criteria::HasAddressBookMetadata(_)
            | Criteria::HasAddressInAddressBook => Ok(()),

            Criteria::ApprovalThreshold(user_specifier, _)
            | Criteria::MinimumVotes(user_specifier, _) => {
                user_specifier.validate().map_err(|e| match e {
                    crate::errors::UserSpecifierError::ValidationError { info } => {
                        PolicyError::ValidationError {
                            info: format!("Invalid user specifier: {}", info),
                        }
                    }
                })
            }

            Criteria::Or(criterias) | Criteria::And(criterias) => {
                for criteria in criterias {
                    criteria.validate()?;
                }
                Ok(())
            }
            Criteria::Not(criteria) => criteria.validate(),
        }
    }
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ApprovalCriteriaInput {
    Remove,
    Set(Criteria),
}

impl From<ProposalVoteStatus> for EvaluationStatus {
    fn from(value: ProposalVoteStatus) -> Self {
        match value {
            ProposalVoteStatus::Accepted => EvaluationStatus::Adopted,
            ProposalVoteStatus::Rejected => EvaluationStatus::Rejected,
            // TODO: Abstain
        }
    }
}

pub trait EvaluateCriteria<
    Status = EvaluationStatus,
    Context = (Arc<Proposal>, Arc<Criteria>),
    Error = EvaluateError,
>: Sync + Send
{
    fn evaluate(&self, ctx: Context) -> Result<Status, Error>;
}

#[derive(Clone)]
pub struct CriteriaEvaluator {
    pub user_matcher: Arc<dyn Match<UserInvolvedInCriteriaForProposalResource>>,
    pub address_book_metadata_matcher: Arc<dyn Match<ProposalHasMetadata>>,
}

struct ProposalVoteSummary {
    total_possible_votes: usize,
    adopted_votes: usize,
    rejected_votes: usize,
}

impl ProposalVoteSummary {
    /// Evaluates the proposal vote summary and returns the evaluation status based on the
    /// minimum votes required.
    ///
    /// If the proposal does not yet have enough votes to meet the minimum votes required but has
    /// enough uncasted votes that could be casted to meet the minimum votes required, then the evaluation
    /// is kept in the `Pending` state.
    fn evaluate(&self, min_votes: &usize) -> EvaluationStatus {
        let min_votes = *cmp::min(min_votes, &self.total_possible_votes);
        let uncasted_votes = self
            .total_possible_votes
            .saturating_sub(self.adopted_votes)
            .saturating_sub(self.rejected_votes);

        if self.adopted_votes >= min_votes {
            return EvaluationStatus::Adopted;
        }

        if self.adopted_votes.saturating_add(uncasted_votes) < min_votes {
            return EvaluationStatus::Rejected;
        }

        EvaluationStatus::Pending
    }
}

impl CriteriaEvaluator {
    fn evaluate_criterias(
        &self,
        proposal: &Arc<Proposal>,
        criterias: &[Criteria],
    ) -> Result<Vec<EvaluationStatus>, EvaluateError> {
        criterias
            .iter()
            .map(|criteria| self.evaluate((proposal.to_owned(), Arc::new(criteria.to_owned()))))
            .collect()
    }

    fn find_matching_users<UserMatchReturn>(
        &self,
        proposal: &Arc<Proposal>,
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
                .is_match(UserInvolvedInCriteriaForProposalResource {
                    proposal_operation_resources: proposal.operation.to_resources(),
                    policy_criteria_user_specifier: user_specifier.to_owned(),
                    user_id: user_id.to_owned(),
                    proposal_id: proposal.id,
                })?
            {
                result.push(match_return.clone());
            }
        }

        Ok(result)
    }

    fn calculate_votes(
        &self,
        proposal: &Arc<Proposal>,
        user_specifier: &UserSpecifier,
    ) -> Result<ProposalVoteSummary, MatchError> {
        let casted_votes = self.find_matching_users::<ProposalVoteStatus>(
            proposal,
            proposal
                .votes
                .iter()
                .map(|vote| (vote.user_id.to_owned(), vote.status.to_owned()))
                .collect::<Vec<(UserId, ProposalVoteStatus)>>()
                .as_slice(),
            user_specifier,
        )?;

        let mut total_possible_votes = self
            .find_matching_users::<()>(
                proposal,
                USER_REPOSITORY
                    .find_where(UserWhereClause {
                        statuses: Some(vec![UserStatus::Active]),
                        search_term: None,
                    })
                    .iter()
                    .map(|user| (user.id.to_owned(), ()))
                    .collect::<Vec<(UserId, ())>>()
                    .as_slice(),
                user_specifier,
            )?
            .len();

        // This is to ensure that the if users become inactive or the criteria is misconfigured
        // the total_possible_votes is not less than the casted votes.
        total_possible_votes = cmp::max(casted_votes.len(), total_possible_votes);

        Ok(ProposalVoteSummary {
            total_possible_votes,
            adopted_votes: casted_votes
                .iter()
                .filter(|&v| matches!(v, ProposalVoteStatus::Accepted))
                .count(),
            rejected_votes: casted_votes
                .iter()
                .filter(|&v| matches!(v, ProposalVoteStatus::Rejected))
                .count(),
        })
    }
}

impl EvaluateCriteria for CriteriaEvaluator {
    fn evaluate(
        &self,
        (proposal, c): (Arc<Proposal>, Arc<Criteria>),
    ) -> Result<EvaluationStatus, EvaluateError> {
        match c.as_ref() {
            Criteria::AutoAdopted => Ok(EvaluationStatus::Adopted),
            Criteria::ApprovalThreshold(user_specifier, percentage) => {
                let votes = self.calculate_votes(&proposal, user_specifier)?;
                let min_votes =
                    calculate_minimum_threshold(percentage, &votes.total_possible_votes);

                Ok(votes.evaluate(&min_votes))
            }
            Criteria::MinimumVotes(user_specifier, min_votes) => {
                let votes = self.calculate_votes(&proposal, user_specifier)?;
                let min_votes = *min_votes as usize;

                Ok(votes.evaluate(&min_votes))
            }
            Criteria::HasAddressBookMetadata(metadata) => {
                let is_match = self
                    .address_book_metadata_matcher
                    .is_match((proposal.as_ref().to_owned(), metadata.clone()))?;
                if is_match {
                    Ok(EvaluationStatus::Adopted)
                } else {
                    Ok(EvaluationStatus::Rejected)
                }
            }
            Criteria::HasAddressInAddressBook => {
                if let ProposalOperation::Transfer(transfer) = &proposal.operation {
                    let account = ACCOUNT_SERVICE.get_account(&transfer.input.from_account_id);
                    match account {
                        Err(e) => {
                            print(format!(
                                "Criteria rejected due to account not being found: {:?}",
                                e
                            ));

                            return Ok(EvaluationStatus::Rejected);
                        }
                        Ok(account) => {
                            let is_in_address_book = ADDRESS_BOOK_REPOSITORY.exists(
                                account.blockchain,
                                account.standard,
                                transfer.input.to.clone(),
                            );

                            if is_in_address_book {
                                return Ok(EvaluationStatus::Adopted);
                            }
                        }
                    }
                }

                Ok(EvaluationStatus::Rejected)
            }
            Criteria::And(criterias) => {
                let evaluation_statuses = self.evaluate_criterias(&proposal, criterias)?;

                if evaluation_statuses
                    .iter()
                    .any(|s| matches!(s, EvaluationStatus::Rejected))
                {
                    return Ok(EvaluationStatus::Rejected);
                }

                if evaluation_statuses
                    .iter()
                    .all(|s| matches!(s, EvaluationStatus::Adopted))
                {
                    return Ok(EvaluationStatus::Adopted);
                }

                Ok(EvaluationStatus::Pending)
            }
            Criteria::Or(criterias) => {
                let evaluation_statuses = self.evaluate_criterias(&proposal, criterias)?;

                if evaluation_statuses
                    .iter()
                    .any(|s| matches!(s, EvaluationStatus::Adopted))
                {
                    return Ok(EvaluationStatus::Adopted);
                }

                if evaluation_statuses
                    .iter()
                    .all(|s| matches!(s, EvaluationStatus::Rejected))
                {
                    return Ok(EvaluationStatus::Rejected);
                }

                Ok(EvaluationStatus::Pending)
            }
            Criteria::Not(criteria) => Ok(
                match self.evaluate((proposal, Arc::new(criteria.as_ref().to_owned())))? {
                    EvaluationStatus::Pending => EvaluationStatus::Pending,
                    EvaluationStatus::Adopted => EvaluationStatus::Rejected,
                    EvaluationStatus::Rejected => EvaluationStatus::Adopted,
                },
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::core::validation::disable_mock_resource_validation;

    use super::*;

    #[test]
    fn fail_critera_with_non_existent_user_specifier() {
        disable_mock_resource_validation();

        Criteria::ApprovalThreshold(UserSpecifier::Id(vec![[0; 16]]), Percentage(100))
            .validate()
            .expect_err("Criteria with non-existent user specifier should fail");

        Criteria::ApprovalThreshold(UserSpecifier::Group(vec![[0; 16]]), Percentage(100))
            .validate()
            .expect_err("Criteria with non-existent user group specifier should fail");

        Criteria::MinimumVotes(UserSpecifier::Id(vec![[0; 16]]), 1)
            .validate()
            .expect_err("Criteria with non-existent user specifier should fail");

        Criteria::MinimumVotes(UserSpecifier::Group(vec![[0; 16]]), 1)
            .validate()
            .expect_err("Criteria with non-existent user group specifier should fail");

        Criteria::And(vec![Criteria::Or(vec![Criteria::Not(Box::new(
            Criteria::ApprovalThreshold(UserSpecifier::Id(vec![[0; 16]]), Percentage(100)),
        ))])])
        .validate()
        .expect_err("Criteria with non-existent user specifier should fail");
    }
}
