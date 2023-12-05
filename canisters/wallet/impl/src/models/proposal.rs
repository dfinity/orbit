use super::{
    PolicyStatus, ProposalOperation, ProposalStatus, ProposalVote, ProposalVoteStatus, UserId,
};
use crate::errors::ProposalError;
use crate::{core::ic_cdk::api::time, factories::proposals::ProposalFactory};
use candid::{CandidType, Deserialize};
use ic_canister_core::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use ic_canister_macros::stable_object;
use std::collections::{HashMap, HashSet};

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
    /// The proposal metadata key-value pairs, where the key is unique and the first entry in the tuple.
    pub metadata: Vec<(String, String)>,
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

fn validate_votes(votes: &Vec<ProposalVote>) -> ModelValidatorResult<ProposalError> {
    if votes.len() > Proposal::MAX_VOTES_ENTRIES as usize {
        return Err(ProposalError::ValidationError {
            info: format!(
                "Proposal vote count exceeds the maximum allowed: {}",
                Proposal::MAX_VOTES_ENTRIES
            ),
        });
    }

    votes.iter().try_for_each(|decision| decision.validate())?;

    Ok(())
}

fn validate_metadata(metadata: &Vec<(String, String)>) -> ModelValidatorResult<ProposalError> {
    if metadata.len() > Proposal::MAX_METADATA_ENTRIES as usize {
        return Err(ProposalError::ValidationError {
            info: format!(
                "Proposal metadata count exceeds the maximum allowed: {}",
                Proposal::MAX_METADATA_ENTRIES
            ),
        });
    }

    for (key, value) in metadata.iter() {
        if key.len() > Proposal::MAX_METADATA_KEY_LEN as usize {
            return Err(ProposalError::ValidationError {
                info: format!(
                    "Proposal metadata key length exceeds the maximum allowed: {}",
                    Proposal::MAX_METADATA_KEY_LEN
                ),
            });
        }

        if value.len() > Proposal::MAX_METADATA_VALUE_LEN as usize {
            return Err(ProposalError::ValidationError {
                info: format!(
                    "Proposal metadata value length exceeds the maximum allowed: {}",
                    Proposal::MAX_METADATA_VALUE_LEN
                ),
            });
        }
    }

    Ok(())
}

impl ModelValidator<ProposalError> for Proposal {
    fn validate(&self) -> ModelValidatorResult<ProposalError> {
        validate_metadata(&self.metadata)?;
        validate_votes(&self.votes)?;

        Ok(())
    }
}

impl Proposal {
    pub const MAX_METADATA_KEY_LEN: u8 = 24;
    pub const MAX_METADATA_VALUE_LEN: u8 = 255;
    pub const MAX_METADATA_ENTRIES: u8 = 10;
    pub const MAX_VOTES_ENTRIES: u8 = 10;

    /// Creates a new proposal key from the given key components.
    pub fn key(proposal_id: ProposalId) -> ProposalKey {
        ProposalKey { id: proposal_id }
    }

    pub fn to_key(&self) -> ProposalKey {
        Proposal::key(self.id.to_owned())
    }

    pub fn users(&self) -> HashSet<UserId> {
        let mut users = HashSet::new();
        users.insert(self.proposed_by.to_owned());

        self.votes
            .iter()
            .map(|decision| decision.user_id.to_owned())
            .for_each(|user_id| {
                users.insert(user_id);
            });

        users
    }

    pub fn metadata_map(&self) -> HashMap<String, String> {
        self.metadata
            .iter()
            .map(|(key, value)| (key.to_owned(), value.to_owned()))
            .collect()
    }

    /// Gives the default expiration date for a proposal which is 7 days from the current time.
    pub fn default_expiration_dt_ns() -> Timestamp {
        let time_in_ns: u64 = 7 * 24 * 60 * 60 * 1_000_000_000;

        time() + time_in_ns
    }

    pub fn can_vote(&self, user_id: &UUID) -> bool {
        let validator = ProposalFactory::validator(self);

        validator.can_vote(user_id)
    }

    pub fn can_view(&self, user_id: &UUID) -> bool {
        let validator = ProposalFactory::validator(self);

        validator.can_view(user_id)
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

    pub async fn reevaluate(&mut self) {
        let evaluator = ProposalFactory::evaluator(self);
        let policies = evaluator.evaluate().await;

        // must drop before updating the proposal due to it being borrowed by the evaluator
        drop(evaluator);

        if policies
            .iter()
            .all(|(_, status)| status == &PolicyStatus::Fulfilled)
        {
            self.status = ProposalStatus::Adopted;
        } else if policies
            .iter()
            .any(|(_, status)| status == &PolicyStatus::Failed)
        {
            self.status = ProposalStatus::Rejected;
        }
    }

    pub async fn on_created(&self) {
        let create_hook = ProposalFactory::create_hook(self);
        create_hook.on_created().await;
    }
}

#[cfg(test)]
mod tests {
    use super::proposal_test_utils::mock_proposal;
    use super::*;
    use crate::models::ProposalVoteStatus;

    #[test]
    fn fail_proposal_metadata_too_many_entries() {
        let mut proposal = mock_proposal();
        proposal.metadata = vec![
            ("foo".to_string(), "bar".to_string());
            Proposal::MAX_METADATA_ENTRIES as usize + 1
        ];

        let result = validate_metadata(&proposal.metadata);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ProposalError::ValidationError {
                info: "Proposal metadata count exceeds the maximum allowed: 10".to_string()
            }
        );
    }

    #[test]
    fn test_proposal_metadata_validation() {
        let mut proposal = mock_proposal();
        proposal.metadata = vec![("a".repeat(24), "b".repeat(24)); 10];

        let result = validate_metadata(&proposal.metadata);

        assert!(result.is_ok());
    }

    #[test]
    fn fail_proposal_metadata_key_too_long() {
        let mut proposal = mock_proposal();
        proposal.metadata = vec![("a".repeat(25), "b".repeat(24))];

        let result = validate_metadata(&proposal.metadata);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ProposalError::ValidationError {
                info: "Proposal metadata key length exceeds the maximum allowed: 24".to_string()
            }
        );
    }

    #[test]
    fn fail_proposal_votes_too_many_entries() {
        let mut proposal = mock_proposal();
        proposal.votes = vec![
            ProposalVote {
                user_id: [0; 16],
                status: ProposalVoteStatus::Rejected,
                status_reason: None,
                decided_dt: 0,
                last_modification_timestamp: 0,
            };
            Proposal::MAX_VOTES_ENTRIES as usize + 1
        ];

        let result = validate_votes(&proposal.votes);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ProposalError::ValidationError {
                info: "Proposal vote count exceeds the maximum allowed: 10".to_string()
            }
        );
    }

    #[test]
    fn test_proposal_votes_validation() {
        let mut proposal = mock_proposal();
        proposal.votes = vec![
            ProposalVote {
                user_id: [0; 16],
                status: ProposalVoteStatus::Rejected,
                status_reason: None,
                decided_dt: 0,
                last_modification_timestamp: 0,
            };
            Proposal::MAX_VOTES_ENTRIES as usize - 1
        ];

        let result = validate_votes(&proposal.votes);

        assert!(result.is_ok());
    }
}

#[cfg(test)]
pub mod proposal_test_utils {
    use num_bigint::BigUint;

    use super::*;
    use crate::models::{ProposalVoteStatus, TransferOperation, TransferOperationInput};

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
                    metadata: vec![],
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
            metadata: vec![("foo".to_string(), "bar".to_string())],
            created_timestamp: 0,
            last_modification_timestamp: 0,
        }
    }
}
