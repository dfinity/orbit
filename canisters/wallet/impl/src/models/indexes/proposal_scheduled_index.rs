use crate::models::{Proposal, ProposalStatus};
use ic_canister_core::types::{Timestamp, UUID};
use ic_canister_macros::storable;
use std::hash::Hash;

/// Represents a proposal index by its status.
#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProposalScheduledIndex {
    /// The time the proposal is scheduled to be executed.
    pub schedule_dt: Timestamp,
    /// The proposal id, which is a UUID.
    pub proposal_id: UUID,
}

#[derive(Clone, Debug)]
pub struct ProposalScheduledIndexCriteria {
    pub from_dt: Option<Timestamp>,
    pub to_dt: Option<Timestamp>,
}

impl Proposal {
    pub fn to_index_by_scheduled(&self) -> Option<ProposalScheduledIndex> {
        if let ProposalStatus::Scheduled { scheduled_at } = &self.status {
            return Some(ProposalScheduledIndex {
                schedule_dt: *scheduled_at,
                proposal_id: self.id,
            });
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use crate::models::{proposal_test_utils::mock_proposal, ProposalStatus};

    #[test]
    fn test_proposal_to_index_by_scheduled() {
        let mut proposal = mock_proposal();
        proposal.last_modification_timestamp = 5;
        proposal.status = ProposalStatus::Scheduled { scheduled_at: 0 };

        let index = proposal.to_index_by_scheduled().unwrap();

        assert_eq!(index.proposal_id, proposal.id);
        assert_eq!(index.schedule_dt, 0);
    }
}
