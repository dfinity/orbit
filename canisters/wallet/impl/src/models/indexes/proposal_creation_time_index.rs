use crate::models::Proposal;
use candid::{CandidType, Deserialize};
use ic_canister_core::types::{Timestamp, UUID};
use ic_canister_macros::stable_object;
use std::hash::Hash;

/// Represents a proposal index by execution time.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProposalCreationTimeIndex {
    /// The time the proposal was created.
    pub created_at: Timestamp,
    /// The proposal id, which is a UUID.
    pub proposal_id: UUID,
}

#[derive(Clone, Debug)]
pub struct ProposalCreationTimeIndexCriteria {
    pub from_dt: Option<Timestamp>,
    pub to_dt: Option<Timestamp>,
}

impl Proposal {
    pub fn to_index_by_creation_dt(&self) -> ProposalCreationTimeIndex {
        ProposalCreationTimeIndex {
            created_at: self.created_timestamp,
            proposal_id: self.id,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::proposal_test_utils::mock_proposal;

    #[test]
    fn test_proposal_to_index_by_creation_dt() {
        let mut proposal = mock_proposal();
        proposal.created_timestamp = 5;

        let index = proposal.to_index_by_creation_dt();

        assert_eq!(index.proposal_id, proposal.id);
        assert_eq!(index.created_at, 5);
    }
}
