use crate::models::Proposal;
use candid::{CandidType, Deserialize};
use ic_canister_core::types::UUID;
use ic_canister_macros::stable_object;

/// Index of proposals to use for sorting.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProposalSortIndex {
    /// The proposal id, which is a UUID.
    pub key: ProposalSortIndexKey,
    /// The proposal's last modification timestamp.
    pub value: ProposalSortIndexValue,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProposalSortIndexKey {
    pub proposal_id: UUID,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProposalSortIndexValue {
    /// The proposal's last modification timestamp.
    pub modification_timestamp: u64,
    /// The proposal's creation timestamp.
    pub creation_timestamp: u64,
    /// The proposal's expiration_dt.
    pub expiration_timestamp: u64,
}

#[derive(Clone, Debug)]
pub struct ProposalSortIndexCriteria {
    pub proposal_id: UUID,
}

impl Proposal {
    pub fn to_index_for_sorting(&self) -> ProposalSortIndex {
        ProposalSortIndex {
            key: ProposalSortIndexKey {
                proposal_id: self.id,
            },
            value: ProposalSortIndexValue {
                modification_timestamp: self.last_modification_timestamp,
                creation_timestamp: self.created_timestamp,
                expiration_timestamp: self.expiration_dt,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::proposal_test_utils::mock_proposal;
    use ic_stable_structures::Storable;

    #[test]
    fn valid_model_serialization() {
        let proposal_id = [1; 16];
        let model = ProposalSortIndex {
            key: ProposalSortIndexKey { proposal_id },
            value: ProposalSortIndexValue {
                creation_timestamp: 1,
                modification_timestamp: 2,
                expiration_timestamp: 3,
            },
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = ProposalSortIndex::from_bytes(serialized_model);

        assert_eq!(model.key.proposal_id, deserialized_model.key.proposal_id);
        assert_eq!(
            model.value.creation_timestamp,
            deserialized_model.value.creation_timestamp
        );
        assert_eq!(
            model.value.modification_timestamp,
            deserialized_model.value.modification_timestamp
        );
        assert_eq!(
            model.value.expiration_timestamp,
            deserialized_model.value.expiration_timestamp
        );
    }

    #[test]
    fn valid_user_voter_indexes() {
        let mut proposal = mock_proposal();
        proposal.id = [1; 16];
        proposal.proposed_by = [u8::MAX; 16];
        proposal.created_timestamp = 1;
        proposal.last_modification_timestamp = 2;
        proposal.expiration_dt = 3;

        let index = proposal.to_index_for_sorting();

        assert_eq!(index.key.proposal_id, proposal.id);
        assert_eq!(index.value.creation_timestamp, proposal.created_timestamp);
        assert_eq!(
            index.value.modification_timestamp,
            proposal.last_modification_timestamp
        );
        assert_eq!(index.value.expiration_timestamp, proposal.expiration_dt);
    }
}
