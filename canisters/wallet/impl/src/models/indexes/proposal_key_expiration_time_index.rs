use crate::models::Proposal;
use candid::{CandidType, Deserialize};
use ic_canister_core::types::{Timestamp, UUID};
use ic_canister_macros::stable_object;
use std::hash::Hash;

/// Represents a proposal index by expiration time prefixed by the proposal id.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProposalKeyExpirationTimeIndex {
    /// The proposal id, which is a UUID.
    pub proposal_id: UUID,
    /// The time the proposal is scheduled to be set as expired if still pending.
    pub expiration_dt: Timestamp,
}

#[derive(Clone, Debug)]
pub struct ProposalKeyExpirationTimeIndexCriteria {
    pub proposal_id: UUID,
    pub from_dt: Option<Timestamp>,
    pub to_dt: Option<Timestamp>,
}

impl Proposal {
    pub fn to_index_by_key_and_expiration_dt(&self) -> ProposalKeyExpirationTimeIndex {
        ProposalKeyExpirationTimeIndex {
            proposal_id: self.id,
            expiration_dt: self.expiration_dt,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::models::proposal_test_utils::mock_proposal;

    #[test]
    fn test_proposal_to_index_by_key_and_expiration_dt() {
        let mut proposal = mock_proposal();
        proposal.expiration_dt = 5;

        let index = proposal.to_index_by_key_and_expiration_dt();

        assert_eq!(index.proposal_id, proposal.id);
        assert_eq!(index.expiration_dt, 5);
    }
}
