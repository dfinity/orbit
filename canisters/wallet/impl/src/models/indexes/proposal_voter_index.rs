use crate::models::Proposal;
use candid::{CandidType, Deserialize};
use ic_canister_core::types::{Timestamp, UUID};
use ic_canister_macros::stable_object;
use std::collections::HashSet;

/// Index of proposals by the voters' user id.
#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProposalVoterIndex {
    /// The user that is associated with this proposal.
    pub user_id: UUID,
    /// The time when the proposal was created.
    pub created_at: Timestamp,
    /// The proposal id, which is a UUID.
    pub proposal_id: UUID,
}

#[derive(Clone, Debug)]
pub struct ProposalVoterIndexCriteria {
    pub user_id: UUID,
    pub from_dt: Option<Timestamp>,
    pub to_dt: Option<Timestamp>,
}

impl Proposal {
    pub fn to_index_for_voters(&self) -> Vec<ProposalVoterIndex> {
        let mut users = HashSet::<UUID>::new();
        self.votes.iter().for_each(|d| {
            users.insert(d.user_id);
        });

        users
            .iter()
            .map(|user_id| ProposalVoterIndex {
                proposal_id: self.id.to_owned(),
                created_at: self.created_timestamp.to_owned(),
                user_id: user_id.to_owned(),
            })
            .collect()
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
        let model = ProposalVoterIndex {
            proposal_id,
            user_id,
            created_at: 0,
        };

        let serialized_model = model.to_bytes();
        let deserialized_model = ProposalVoterIndex::from_bytes(serialized_model);

        assert_eq!(model.proposal_id, deserialized_model.proposal_id);
        assert_eq!(model.user_id, deserialized_model.user_id);
    }

    #[test]
    fn valid_user_voter_indexes() {
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

        let indexes = proposal.to_index_for_voters();

        assert_eq!(indexes.len(), 2);
        assert!(indexes.iter().any(|i| i.user_id == [1; 16]));
        assert!(indexes.iter().any(|i| i.user_id == [2; 16]));
    }
}
