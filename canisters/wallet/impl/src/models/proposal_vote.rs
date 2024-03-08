use super::{ProposalVoteStatus, UserId};
use crate::errors::ProposalError;
use ic_canister_core::{
    model::{ModelValidator, ModelValidatorResult},
    types::Timestamp,
};
use ic_canister_macros::storable;

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

impl ProposalVote {
    pub const MAX_REASON_LEN: u8 = 200;
}

fn validate_reason(reason: &Option<String>) -> ModelValidatorResult<ProposalError> {
    if let Some(reason) = reason {
        if reason.len() > ProposalVote::MAX_REASON_LEN as usize {
            return Err(ProposalError::VoteReasonTooLong {
                max_len: ProposalVote::MAX_REASON_LEN,
            });
        }
    }

    Ok(())
}

impl ModelValidator<ProposalError> for ProposalVote {
    fn validate(&self) -> ModelValidatorResult<ProposalError> {
        validate_reason(&self.status_reason)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fail_proposal_vote_too_big_reason() {
        let mut decision = proposal_vote_test_utils::mock_decision();
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
        let mut decision = proposal_vote_test_utils::mock_decision();
        decision.status_reason = Some("a".repeat(200));

        let result = decision.validate();

        assert!(result.is_ok());
    }
}

#[cfg(test)]
pub mod proposal_vote_test_utils {
    use super::ProposalVote;
    use crate::models::ProposalVoteStatus;
    use ic_canister_core::types::UUID;

    pub fn mock_decision() -> ProposalVote {
        ProposalVote {
            user_id: [0; 16],
            status: ProposalVoteStatus::Rejected,
            status_reason: None,
            decided_dt: 0,
            last_modification_timestamp: 0,
        }
    }

    pub fn mock_accepted_with_user(user_id: UUID) -> ProposalVote {
        ProposalVote {
            user_id,
            status: ProposalVoteStatus::Accepted,
            status_reason: None,
            decided_dt: 0,
            last_modification_timestamp: 0,
        }
    }

    pub fn mock_rejected_with_user(user_id: UUID) -> ProposalVote {
        ProposalVote {
            user_id,
            status: ProposalVoteStatus::Rejected,
            status_reason: None,
            decided_dt: 0,
            last_modification_timestamp: 0,
        }
    }
}
