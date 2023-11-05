use crate::{models::Proposal, transport::ProposalDTO};
use ic_canister_core::utils::timestamp_to_rfc3339;
use uuid::Uuid;

impl From<Proposal> for ProposalDTO {
    fn from(proposal: Proposal) -> ProposalDTO {
        ProposalDTO {
            id: Uuid::from_bytes(proposal.id).hyphenated().to_string(),
            proposed_by: proposal
                .proposed_by
                .map(|id| Uuid::from_bytes(id).hyphenated().to_string()),
            status: proposal.status.into(),
            metadata: proposal.metadata,
            operation: proposal.operation.into(),
            created_at: timestamp_to_rfc3339(&proposal.created_timestamp),
            votes: proposal
                .votes
                .iter()
                .map(|vote| vote.to_owned().into())
                .collect(),
        }
    }
}
