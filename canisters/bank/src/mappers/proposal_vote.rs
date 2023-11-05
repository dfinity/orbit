use crate::{models::ProposalVote, transport::ProposalVoteDTO};
use ic_canister_core::utils::timestamp_to_rfc3339;
use uuid::Uuid;

impl From<ProposalVote> for ProposalVoteDTO {
    fn from(vote: ProposalVote) -> Self {
        Self {
            user_id: Uuid::from_bytes(vote.user_id).hyphenated().to_string(),
            decided_at: vote.decided_dt.map(|dt| timestamp_to_rfc3339(&dt)),
            read: vote.read,
            status: vote.status.into(),
            status_reason: vote.status_reason,
        }
    }
}
