use super::{
    EvaluationStatus, ProposalOperation, ProposalStatus, ProposalVote, ProposalVoteStatus, UserId,
};
use crate::core::evaluation::{
    Evaluate, CRITERIA_EVALUATOR, PROPOSAL_MATCHER, PROPOSAL_POSSIBLE_VOTERS_CRITERIA_EVALUATOR,
    PROPOSAL_VOTE_RIGHTS_CRITERIA_EVALUATOR,
};
use crate::core::ic_cdk::api::{print, time};
use crate::core::proposal::{
    ProposalEvaluator, ProposalPossibleVotersFinder, ProposalVoteRightsEvaluator,
};
use crate::errors::{EvaluateError, ProposalError};
use candid::{CandidType, Deserialize};
use ic_canister_core::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use ic_canister_macros::stable_object;
use std::collections::HashSet;

/// The proposal id, which is a UUID.
pub type ProposalId = UUID;

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProposalExecutionPlan {
    Immediate,
    Scheduled { execution_time: Timestamp },
}

/// Represents a proposal within the system.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Proposal {
    /// The proposal id, which is a UUID.
    pub id: ProposalId,
    /// The title of the proposal.
    pub title: String,
    /// The summary of the proposal, this is a longer description of the proposal.
    pub summary: Option<String>,
    /// The user id that resulted in the proposal creation.
    pub proposed_by: UserId,
    /// The status that the proposal is in.
    pub status: ProposalStatus,
    /// An operation that the proposal should execute, e.g. "transfer".
    pub operation: ProposalOperation,
    /// The expiration date of the proposal.
    pub expiration_dt: Timestamp,
    /// The execution plan of the proposal.
    pub execution_plan: ProposalExecutionPlan,
    /// The votes that the proposal has received.
    pub votes: Vec<ProposalVote>,
    /// The timestamp of the proposal creation.
    pub created_timestamp: Timestamp,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProposalKey {
    /// The proposal id, which is a UUID.
    pub id: ProposalId,
}

fn validate_title(title: &str) -> ModelValidatorResult<ProposalError> {
    if title.len() > Proposal::MAX_TITLE_LEN as usize {
        return Err(ProposalError::ValidationError {
            info: format!(
                "Proposal title length exceeds the maximum allowed: {}",
                Proposal::MAX_TITLE_LEN
            ),
        });
    }

    Ok(())
}

fn validate_summary(summary: &Option<String>) -> ModelValidatorResult<ProposalError> {
    if let Some(summary) = summary {
        if summary.len() > Proposal::MAX_SUMMARY_LEN as usize {
            return Err(ProposalError::ValidationError {
                info: format!(
                    "Proposal summary length exceeds the maximum allowed: {}",
                    Proposal::MAX_SUMMARY_LEN
                ),
            });
        }
    }

    Ok(())
}

impl ModelValidator<ProposalError> for Proposal {
    fn validate(&self) -> ModelValidatorResult<ProposalError> {
        validate_title(&self.title)?;
        validate_summary(&self.summary)?;

        Ok(())
    }
}

impl Proposal {
    pub const MAX_TITLE_LEN: u8 = 255;
    pub const MAX_SUMMARY_LEN: u16 = 1000;

    /// Creates a new proposal key from the given key components.
    pub fn key(proposal_id: ProposalId) -> ProposalKey {
        ProposalKey { id: proposal_id }
    }

    pub fn to_key(&self) -> ProposalKey {
        Proposal::key(self.id.to_owned())
    }

    pub fn voters(&self) -> HashSet<UserId> {
        let mut users = HashSet::new();

        self.votes
            .iter()
            .map(|decision| decision.user_id.to_owned())
            .for_each(|user_id| {
                users.insert(user_id);
            });

        users
    }

    /// Gives the default expiration date for a proposal which is 7 days from the current time.
    pub fn default_expiration_dt_ns() -> Timestamp {
        let time_in_ns: u64 = 7 * 24 * 60 * 60 * 1_000_000_000;

        time() + time_in_ns
    }

    pub async fn can_vote(&self, user_id: &UUID) -> bool {
        let validator = ProposalVoteRightsEvaluator {
            proposal: self,
            voter_id: *user_id,
            proposal_matcher: PROPOSAL_MATCHER.to_owned(),
            vote_rights_evaluator: PROPOSAL_VOTE_RIGHTS_CRITERIA_EVALUATOR.clone(),
        };

        match validator.evaluate().await {
            Ok(can_vote) => can_vote,
            Err(_) => {
                print(format!(
                    "Failed to evaluate voting rights for proposal: {:?}",
                    self
                ));

                false
            }
        }
    }

    pub fn add_vote(&mut self, user_id: UUID, vote: ProposalVoteStatus, reason: Option<String>) {
        if self.votes.iter().any(|vote| vote.user_id == user_id) {
            // users can only vote once per proposal
            return;
        }

        self.votes.push(ProposalVote {
            user_id,
            status: vote,
            status_reason: reason,
            decided_dt: time(),
            last_modification_timestamp: time(),
        });
    }

    pub async fn reevaluate(&mut self) -> Result<(), EvaluateError> {
        if let ProposalStatus::Created = self.status {
            let evaluator = ProposalEvaluator {
                proposal: self.to_owned(),
                proposal_matcher: PROPOSAL_MATCHER.to_owned(),
                criteria_evaluator: CRITERIA_EVALUATOR.to_owned(),
            };

            let evaluation_status = evaluator.evaluate().await?;

            if evaluation_status == EvaluationStatus::Adopted {
                self.status = ProposalStatus::Adopted;
            } else if evaluation_status == EvaluationStatus::Rejected {
                self.status = ProposalStatus::Rejected;
            }
        }

        Ok(())
    }

    pub async fn find_all_possible_voters(&self) -> Result<HashSet<UUID>, EvaluateError> {
        let evaluator = ProposalPossibleVotersFinder {
            proposal: self,
            proposal_matcher: PROPOSAL_MATCHER.to_owned(),
            possible_voters_criteria_evaluator: PROPOSAL_POSSIBLE_VOTERS_CRITERIA_EVALUATOR
                .to_owned(),
        };

        evaluator.evaluate().await
    }
}

#[cfg(test)]
mod tests {
    use super::proposal_test_utils::mock_proposal;
    use super::*;

    #[test]
    fn fail_proposal_title_too_big() {
        let mut proposal = mock_proposal();
        proposal.title = "a".repeat(Proposal::MAX_TITLE_LEN as usize + 1);

        let result = validate_title(&proposal.title);

        assert!(result.is_err());
    }

    #[test]
    fn test_proposal_title_is_valid() {
        let mut proposal = mock_proposal();
        proposal.title = "a".repeat(Proposal::MAX_TITLE_LEN as usize);

        let result = validate_title(&proposal.title);

        assert!(result.is_ok());
    }

    #[test]
    fn fail_proposal_summary_too_big() {
        let mut proposal = mock_proposal();
        proposal.summary = Some("a".repeat(Proposal::MAX_SUMMARY_LEN as usize + 1));

        let result = validate_summary(&proposal.summary);

        assert!(result.is_err());
    }

    #[test]
    fn test_proposal_summary_is_valid() {
        let mut proposal = mock_proposal();
        proposal.summary = Some("a".repeat(Proposal::MAX_SUMMARY_LEN as usize));

        let result = validate_summary(&proposal.summary);

        assert!(result.is_ok());
    }
}

#[cfg(test)]
pub mod proposal_test_utils {
    use num_bigint::BigUint;

    use super::*;
    use crate::models::{Metadata, ProposalVoteStatus, TransferOperation, TransferOperationInput};

    pub fn mock_proposal() -> Proposal {
        Proposal {
            id: [0; 16],
            title: "foo".to_string(),
            summary: Some("bar".to_string()),
            proposed_by: [1; 16],
            status: ProposalStatus::Adopted,
            expiration_dt: 100,
            execution_plan: ProposalExecutionPlan::Immediate,
            operation: ProposalOperation::Transfer(TransferOperation {
                transfer_id: None,
                input: TransferOperationInput {
                    network: "mainnet".to_string(),
                    amount: candid::Nat(BigUint::from(100u32)),
                    fee: None,
                    metadata: Metadata::default(),
                    to: "0x1234".to_string(),
                    from_account_id: [1; 16],
                },
            }),
            votes: vec![ProposalVote {
                user_id: [1; 16],
                status: ProposalVoteStatus::Accepted,
                status_reason: None,
                decided_dt: 0,
                last_modification_timestamp: 0,
            }],
            created_timestamp: 0,
            last_modification_timestamp: 0,
        }
    }
}
