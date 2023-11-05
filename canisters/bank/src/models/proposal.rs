use super::{ProposalOperation, ProposalStatus, ProposalVote, UserId};
use crate::errors::ProposalError;
use candid::{CandidType, Deserialize};
use ic_canister_core::{
    model::{ModelValidator, ModelValidatorResult},
    types::{Timestamp, UUID},
};
use ic_canister_macros::stable_object;
use std::collections::{HashMap, HashSet};

/// The proposal id, which is a UUID.
pub type ProposalId = UUID;

/// Represents a proposal within the system.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Proposal {
    /// The proposal id, which is a UUID.
    pub id: ProposalId,
    /// The user id that resulted in the proposal creation.
    ///
    /// When the proposal is created by the system, this field is `None`.
    pub proposed_by: Option<UserId>,
    /// The status that the proposal is in.
    pub status: ProposalStatus,
    /// An proposal that the proposal should execute, e.g. "transfer".
    pub operation: ProposalOperation,
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

pub struct ProposalValidator<'model> {
    proposal: &'model Proposal,
}

impl<'proposal> ProposalValidator<'proposal> {
    pub const MAX_METADATA_KEY_LEN: u8 = 24;
    pub const MAX_METADATA_VALUE_LEN: u8 = 255;
    pub const MAX_METADATA_ENTRIES: u8 = 10;
    pub const MAX_VOTES_ENTRIES: u8 = 10;

    pub fn new(proposal: &'proposal Proposal) -> ProposalValidator {
        ProposalValidator { proposal }
    }

    pub fn validate_votes(&self) -> ModelValidatorResult<ProposalError> {
        if self.proposal.votes.len() > Self::MAX_VOTES_ENTRIES as usize {
            return Err(ProposalError::ValidationError {
                info: format!(
                    "Proposal vote count exceeds the maximum allowed: {}",
                    Self::MAX_VOTES_ENTRIES
                ),
            });
        }

        self.proposal
            .votes
            .iter()
            .try_for_each(|decision| decision.validate())?;

        Ok(())
    }

    pub fn validate_metadata(&self) -> ModelValidatorResult<ProposalError> {
        if self.proposal.metadata.len() > Self::MAX_METADATA_ENTRIES as usize {
            return Err(ProposalError::ValidationError {
                info: format!(
                    "Proposal metadata count exceeds the maximum allowed: {}",
                    Self::MAX_METADATA_ENTRIES
                ),
            });
        }

        for (key, value) in self.proposal.metadata.iter() {
            if key.len() > Self::MAX_METADATA_KEY_LEN as usize {
                return Err(ProposalError::ValidationError {
                    info: format!(
                        "Proposal metadata key length exceeds the maximum allowed: {}",
                        Self::MAX_METADATA_KEY_LEN
                    ),
                });
            }

            if value.len() > Self::MAX_METADATA_VALUE_LEN as usize {
                return Err(ProposalError::ValidationError {
                    info: format!(
                        "Proposal metadata value length exceeds the maximum allowed: {}",
                        Self::MAX_METADATA_VALUE_LEN
                    ),
                });
            }
        }

        Ok(())
    }

    pub fn validate(&self) -> ModelValidatorResult<ProposalError> {
        self.validate_metadata()?;
        self.validate_votes()?;

        Ok(())
    }
}

impl ModelValidator<ProposalError> for Proposal {
    fn validate(&self) -> ModelValidatorResult<ProposalError> {
        ProposalValidator::new(self).validate()
    }
}

impl Proposal {
    /// Creates a new proposal key from the given key components.
    pub fn key(proposal_id: ProposalId) -> ProposalKey {
        ProposalKey { id: proposal_id }
    }

    pub fn to_key(&self) -> ProposalKey {
        Proposal::key(self.id.to_owned())
    }

    pub fn users(&self) -> HashSet<UserId> {
        let mut users = HashSet::new();
        if let Some(proposed_by) = self.proposed_by.to_owned() {
            users.insert(proposed_by);
        }

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
            ProposalValidator::MAX_METADATA_ENTRIES as usize + 1
        ];

        let result = ProposalValidator::new(&proposal).validate_metadata();

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

        let result = ProposalValidator::new(&proposal).validate_metadata();

        assert!(result.is_ok());
    }

    #[test]
    fn fail_proposal_metadata_key_too_long() {
        let mut proposal = mock_proposal();
        proposal.metadata = vec![("a".repeat(25), "b".repeat(24))];

        let result = ProposalValidator::new(&proposal).validate_metadata();

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
                read: false,
                status: ProposalVoteStatus::Rejected,
                status_reason: None,
                decided_dt: None,
                last_modification_timestamp: 0,
            };
            ProposalValidator::MAX_VOTES_ENTRIES as usize + 1
        ];

        let result = ProposalValidator::new(&proposal).validate_votes();

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
                read: false,
                status: ProposalVoteStatus::Rejected,
                status_reason: None,
                decided_dt: None,
                last_modification_timestamp: 0,
            };
            ProposalValidator::MAX_VOTES_ENTRIES as usize - 1
        ];

        let result = ProposalValidator::new(&proposal).validate_votes();

        assert!(result.is_ok());
    }
}

#[cfg(test)]
pub mod proposal_test_utils {
    use super::*;
    use crate::models::{ProposalVoteStatus, TransferOperationContext};

    pub fn mock_proposal() -> Proposal {
        Proposal {
            id: [0; 16],
            proposed_by: Some([1; 16]),
            status: ProposalStatus::Adopted,
            operation: ProposalOperation::Transfer(TransferOperationContext {
                transfer_id: [0; 16],
                account_id: [1; 16],
            }),
            votes: vec![ProposalVote {
                user_id: [1; 16],
                read: true,
                status: ProposalVoteStatus::Adopted,
                status_reason: None,
                decided_dt: Some(0),
                last_modification_timestamp: 0,
            }],
            metadata: vec![("foo".to_string(), "bar".to_string())],
            created_timestamp: 0,
            last_modification_timestamp: 0,
        }
    }
}
