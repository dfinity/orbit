use crate::models::Proposal;
use candid::{CandidType, Deserialize};
use ic_canister_core::types::{Timestamp, UUID};
use ic_canister_macros::stable_object;

/// Index of proposals by the user id.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProposalProposerIndex {
    /// The user that is associated with this proposal.
    pub user_id: UUID,
    /// The time when the proposal was created.
    pub created_at: Timestamp,
    /// The proposal id, which is a UUID.
    pub proposal_id: UUID,
}

#[derive(Clone, Debug)]
pub struct ProposalProposerIndexCriteria {
    pub user_id: UUID,
    pub from_dt: Option<Timestamp>,
    pub to_dt: Option<Timestamp>,
}

impl Proposal {
    pub fn to_index_for_proposer(&self) -> ProposalProposerIndex {
        ProposalProposerIndex {
            user_id: self.proposed_by.to_owned(),
            proposal_id: self.id.to_owned(),
            created_at: self.created_timestamp.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{proposal_test_utils::mock_proposal, ProposalVote, ProposalVoteStatus};
    use ic_stable_structures::Storable;

    #[test]
    fn valid_model_serialization() {
        let proposal_id = [1; 16];
        let user_id = [u8::MAX; 16];
        let model = ProposalProposerIndex {
            proposal_id,
            user_id,
            created_at: 0,
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = ProposalProposerIndex::from_bytes(serialized_model);

        assert_eq!(model.proposal_id, deserialized_model.proposal_id);
        assert_eq!(model.user_id, deserialized_model.user_id);
    }

    #[test]
    fn valid_user_proposal_indexes() {
        let mut proposal = mock_proposal();
        proposal.id = [1; 16];
        proposal.proposed_by = [u8::MAX; 16];
        proposal.votes = vec![
            ProposalVote {
                user_id: [1; 16],
                status_reason: None,
                decided_dt: 0,
                last_modification_timestamp: 0,
                status: ProposalVoteStatus::Accepted,
            },
            ProposalVote {
                user_id: [2; 16],
                status_reason: None,
                decided_dt: 0,
                last_modification_timestamp: 0,
                status: ProposalVoteStatus::Accepted,
            },
        ];

        let index = proposal.to_index_for_proposer();

        assert_eq!(index.created_at, proposal.created_timestamp);
        assert_eq!(index.user_id, proposal.proposed_by);
    }
}
