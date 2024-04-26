use crate::models::{Proposal, ProposalId, ProposalStatusCode};
use orbit_essentials::storable;
use orbit_essentials::types::Timestamp;
use std::hash::Hash;

/// Represents a proposal index by its status and the last modification timestamp.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProposalStatusModificationIndex {
    /// The status of the proposal.
    pub status: ProposalStatusCode,
    /// The last modification timestamp of the proposal.
    pub modification_timestamp: Timestamp,
    /// The proposal id, which is a UUID.
    pub proposal_id: ProposalId,
}

#[derive(Clone, Debug)]
pub struct ProposalStatusModificationIndexCriteria {
    pub status: ProposalStatusCode,
    pub from_dt: Option<Timestamp>,
    pub to_dt: Option<Timestamp>,
}

impl Proposal {
    pub fn to_index_by_status_and_modification(&self) -> ProposalStatusModificationIndex {
        ProposalStatusModificationIndex {
            status: self.status.to_type(),
            modification_timestamp: self.last_modification_timestamp,
            proposal_id: self.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{proposal_test_utils::mock_proposal, ProposalStatus, ProposalStatusCode};

    #[test]
    fn test_proposal_to_index_by_status_and_modification() {
        let mut proposal = mock_proposal();
        proposal.last_modification_timestamp = 5;
        proposal.status = ProposalStatus::Created;

        let index = proposal.to_index_by_status_and_modification();

        assert_eq!(index.proposal_id, proposal.id);
        assert_eq!(index.status, ProposalStatusCode::Created);
        assert_eq!(index.modification_timestamp, 5);
    }
}
