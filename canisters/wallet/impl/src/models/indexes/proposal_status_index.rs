use crate::models::{Proposal, ProposalStatusType};
use ic_canister_core::types::UUID;
use ic_canister_macros::storable;
use std::hash::Hash;

/// Represents a proposal index by its status.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProposalStatusIndex {
    /// The status of the proposal.
    pub status: ProposalStatusType,
    /// The proposal id, which is a UUID.
    pub proposal_id: UUID,
}

#[derive(Clone, Debug)]
pub struct ProposalStatusIndexCriteria {
    pub status: ProposalStatusType,
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
    use crate::models::{proposal_test_utils::mock_proposal, ProposalStatus, ProposalStatusType};

    #[test]
    fn test_proposal_to_index_by_status() {
        let mut proposal = mock_proposal();
        proposal.last_modification_timestamp = 5;
        proposal.status = ProposalStatus::Created;

        let index = proposal.to_index_by_status();

        assert_eq!(index.proposal_id, proposal.id);
        assert_eq!(index.status, ProposalStatusType::Created);
    }
}
