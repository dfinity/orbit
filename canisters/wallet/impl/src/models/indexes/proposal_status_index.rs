use crate::models::Proposal;
use candid::{CandidType, Deserialize};
use ic_canister_core::types::{Timestamp, UUID};
use ic_canister_macros::stable_object;
use std::hash::Hash;

/// Represents a proposal index by its status.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProposalStatusIndex {
    /// The status of the proposal.
    pub status: String,
    /// The last time the proposal was modified.
    pub last_modification_timestamp: Timestamp,
    /// The proposal id, which is a UUID.
    pub proposal_id: UUID,
}

#[derive(Clone, Debug)]
pub struct ProposalStatusIndexCriteria {
    pub status: String,
    pub from_dt: Option<Timestamp>,
    pub to_dt: Option<Timestamp>,
}

impl Proposal {
    pub fn to_index_by_status(&self) -> ProposalStatusIndex {
        ProposalStatusIndex {
            status: self.status.to_string(),
            last_modification_timestamp: self.last_modification_timestamp,
            proposal_id: self.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{proposal_test_utils::mock_proposal, ProposalStatus};

    #[test]
    fn test_proposal_to_index_by_status() {
        let mut proposal = mock_proposal();
        proposal.last_modification_timestamp = 5;
        proposal.status = ProposalStatus::Created;

        let index = proposal.to_index_by_status();

        assert_eq!(index.proposal_id, proposal.id);
        assert_eq!(index.last_modification_timestamp, 5);
        assert_eq!(index.status, "created");
    }
}
