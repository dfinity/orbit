use crate::models::{Proposal, ProposalStatusCode};
use orbit_essentials::storable;
use orbit_essentials::types::UUID;
use std::hash::Hash;

/// Represents a proposal index by its status.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProposalStatusIndex {
    /// The status of the proposal.
    pub status: ProposalStatusCode,
    /// The proposal id, which is a UUID.
    pub proposal_id: UUID,
}

#[derive(Clone, Debug)]
pub struct ProposalStatusIndexCriteria {
    pub status: ProposalStatusCode,
}

impl Proposal {
    pub fn to_index_by_status(&self) -> ProposalStatusIndex {
        ProposalStatusIndex {
            status: self.status.to_type(),
            proposal_id: self.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{proposal_test_utils::mock_proposal, ProposalStatus, ProposalStatusCode};

    #[test]
    fn test_proposal_to_index_by_status() {
        let mut proposal = mock_proposal();
        proposal.last_modification_timestamp = 5;
        proposal.status = ProposalStatus::Created;

        let index = proposal.to_index_by_status();

        assert_eq!(index.proposal_id, proposal.id);
        assert_eq!(index.status, ProposalStatusCode::Created);
    }
}
