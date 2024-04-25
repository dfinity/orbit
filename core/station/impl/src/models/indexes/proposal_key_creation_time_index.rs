use crate::models::Proposal;
use ic_canister_core::types::{Timestamp, UUID};
use ic_canister_macros::storable;
use std::hash::Hash;

/// Represents a proposal index by creation time prefixed by the proposal id.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProposalKeyCreationTimeIndex {
    /// The proposal id, which is a UUID.
    pub proposal_id: UUID,
    /// The time the proposal was created.
    pub created_at: Timestamp,
}

#[derive(Clone, Debug)]
pub struct ProposalKeyCreationTimeIndexCriteria {
    pub proposal_id: UUID,
    pub from_dt: Option<Timestamp>,
    pub to_dt: Option<Timestamp>,
}

impl Proposal {
    pub fn to_index_by_key_and_creation_dt(&self) -> ProposalKeyCreationTimeIndex {
        ProposalKeyCreationTimeIndex {
            proposal_id: self.id,
            created_at: self.created_timestamp,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::proposal_test_utils::mock_proposal;

    #[test]
    fn test_proposal_to_index_by_key_and_creation_dt() {
        let mut proposal = mock_proposal();
        proposal.created_timestamp = 5;

        let index = proposal.to_index_by_key_and_creation_dt();

        assert_eq!(index.proposal_id, proposal.id);
        assert_eq!(index.created_at, 5);
    }
}
