use super::{ProposalVoteStatus, UserId};
use crate::errors::ProposalError;
use candid::{CandidType, Deserialize};
use ic_canister_core::{
    model::{ModelValidator, ModelValidatorResult},
    types::Timestamp,
};
use ic_canister_macros::stable_object;

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProposalVote {
    /// The user id associated with the vote.
    pub user_id: UserId,
    /// The status is provided by the associated user.
    pub status: ProposalVoteStatus,
    /// Optional reason for the vote status.
    pub status_reason: Option<String>,
    /// When the vote was decided.
    pub decided_dt: Timestamp,
    /// The last time the record was updated or created.
    pub last_modification_timestamp: Timestamp,
}

pub struct ProposalVoteValidator<'model> {
    task: &'model ProposalVote,
}

impl<'model> ProposalVoteValidator<'model> {
    pub const MAX_REASON_LEN: u8 = 200;

    pub fn new(task: &'model ProposalVote) -> ProposalVoteValidator {
        ProposalVoteValidator { task }
    }

    pub fn validate_reason(&self) -> ModelValidatorResult<ProposalError> {
        if let Some(reason) = &self.task.status_reason {
            if reason.len() > Self::MAX_REASON_LEN as usize {
                return Err(ProposalError::VoteReasonTooLong {
                    max_len: Self::MAX_REASON_LEN,
                });
            }
        }

        Ok(())
    }

    pub fn validate(&self) -> ModelValidatorResult<ProposalError> {
        self.validate_reason()?;

        Ok(())
    }
}

impl ModelValidator<ProposalError> for ProposalVote {
    fn validate(&self) -> ModelValidatorResult<ProposalError> {
        ProposalVoteValidator::new(self).validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fail_proposal_vote_too_big_reason() {
        let mut decision = mock_decision();
        decision.status_reason = Some("a".repeat(201));

        let result = decision.validate();

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            ProposalError::VoteReasonTooLong { max_len: 200 }
        );
    }

    #[test]
    fn test_proposal_vote_with_reason() {
        let mut decision = mock_decision();
        decision.status_reason = Some("a".repeat(200));

        let result = decision.validate();

        assert!(result.is_ok());
    }

    fn mock_decision() -> ProposalVote {
        ProposalVote {
            user_id: [0; 16],
            status: ProposalVoteStatus::Rejected,
            status_reason: None,
            decided_dt: 0,
            last_modification_timestamp: 0,
        }
    }
}
