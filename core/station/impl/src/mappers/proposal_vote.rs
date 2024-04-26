use crate::models::ProposalVote;
use orbit_essentials::utils::timestamp_to_rfc3339;
use station_api::ProposalVoteDTO;
use uuid::Uuid;

impl From<ProposalVote> for ProposalVoteDTO {
    fn from(vote: ProposalVote) -> Self {
        Self {
            user_id: Uuid::from_bytes(vote.user_id).hyphenated().to_string(),
            decided_at: timestamp_to_rfc3339(&vote.decided_dt),
            status: vote.status.into(),
            status_reason: vote.status_reason,
        }
    }
}
