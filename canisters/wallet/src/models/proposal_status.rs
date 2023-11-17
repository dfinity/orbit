use candid::{CandidType, Deserialize};
use ic_canister_core::types::Timestamp;
use ic_canister_macros::stable_object;
use std::fmt::{Display, Formatter};

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ProposalStatus {
    Created,
    Adopted,
    Rejected,
    Scheduled { scheduled_at: Timestamp },
    Cancelled { reason: Option<String> },
    Processing { started_at: Timestamp },
    Completed { completed_at: Timestamp },
    Failed { reason: Option<String> },
}

impl Display for ProposalStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProposalStatus::Created => write!(f, "created"),
            ProposalStatus::Adopted => write!(f, "adopted"),
            ProposalStatus::Rejected => write!(f, "rejected"),
            ProposalStatus::Scheduled { .. } => write!(f, "scheduled"),
            ProposalStatus::Processing { .. } => write!(f, "processing"),
            ProposalStatus::Completed { .. } => write!(f, "completed"),
            ProposalStatus::Failed { .. } => write!(f, "failed"),
            ProposalStatus::Cancelled { .. } => write!(f, "cancelled"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_string_representation() {
        assert_eq!(ProposalStatus::Created.to_string(), "created");
        assert_eq!(ProposalStatus::Adopted.to_string(), "adopted");
        assert_eq!(ProposalStatus::Rejected.to_string(), "rejected");
        assert_eq!(
            ProposalStatus::Scheduled { scheduled_at: 0 }.to_string(),
            "scheduled"
        );
        assert_eq!(
            ProposalStatus::Processing { started_at: 0 }.to_string(),
            "processing"
        );
        assert_eq!(
            ProposalStatus::Completed { completed_at: 0 }.to_string(),
            "completed"
        );
        assert_eq!(
            ProposalStatus::Failed { reason: None }.to_string(),
            "failed"
        );
        assert_eq!(
            ProposalStatus::Cancelled { reason: None }.to_string(),
            "cancelled"
        );
    }
}
