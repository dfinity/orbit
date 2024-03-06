use crate::models::Proposal;
use candid::{CandidType, Deserialize};
use ic_canister_core::types::{Timestamp, UUID};
use ic_canister_macros::stable_object;
use std::hash::Hash;

/// Represents a proposal index by its status and the last modification timestamp.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProposalStatusModificationIndex {
    /// The status of the proposal.
    pub status: String,
    /// The last modification timestamp of the proposal.
    pub modification_timestamp: Timestamp,
    /// The proposal id, which is a UUID.
    pub proposal_id: UUID,
}

#[derive(Clone, Debug)]
pub struct ProposalStatusModificationIndexCriteria {
    pub status: String,
    pub from_dt: Option<Timestamp>,
    pub to_dt: Option<Timestamp>,
}

impl Proposal {
    pub fn to_index_by_status_and_modification(&self) -> ProposalStatusModificationIndex {
        ProposalStatusModificationIndex {
            status: self.status.to_type().to_string(),
            modification_timestamp: self.last_modification_timestamp,
            proposal_id: self.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{proposal_test_utils::mock_proposal, ProposalStatus};

    #[test]
    fn test_proposal_to_index_by_status_and_modification() {
        let mut proposal = mock_proposal();
        proposal.last_modification_timestamp = 5;
        proposal.status = ProposalStatus::Created;

        let index = proposal.to_index_by_status_and_modification();

        assert_eq!(index.proposal_id, proposal.id);
        assert_eq!(index.status, "created");
        assert_eq!(index.modification_timestamp, 5);
    }
}
