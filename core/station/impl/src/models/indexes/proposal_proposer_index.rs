use crate::models::Proposal;
use ic_canister_core::types::UUID;
use ic_canister_macros::storable;

/// Index of proposals by the proposer user id.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProposalProposerIndex {
    /// The user who proposed this proposal.
    pub proposer_id: UUID,
    /// The proposal id, which is a UUID.
    pub proposal_id: UUID,
}

#[derive(Clone, Debug)]
pub struct ProposalProposerIndexCriteria {
    pub proposer_id: UUID,
}

impl Proposal {
    pub fn to_index_for_proposer(&self) -> ProposalProposerIndex {
        ProposalProposerIndex {
            proposer_id: self.proposed_by.to_owned(),
            proposal_id: self.id.to_owned(),
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
            proposer_id: user_id,
            proposal_id,
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = ProposalProposerIndex::from_bytes(serialized_model);

        assert_eq!(model.proposal_id, deserialized_model.proposal_id);
        assert_eq!(model.proposer_id, deserialized_model.proposer_id);
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

        assert_eq!(index.proposal_id, proposal.id);
        assert_eq!(index.proposer_id, proposal.proposed_by);
    }
}
