use super::{AccountId, ProposalId, TransferId};
use candid::{CandidType, Deserialize};
use ic_canister_macros::stable_object;
use std::fmt::{Display, Formatter};

pub const SYSTEM_MESSAGE_NOTIFICATION_TYPE: &str = "system-message";
pub const PROPOSAL_CREATED_NOTIFICATION_TYPE: &str = "proposal-created";
pub const TRANSFER_PROPOSAL_CREATED_NOTIFICATION_TYPE: &str = "transfer-proposal-created";

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NotificationType {
    SystemMessage,
    ProposalCreated(ProposalCreatedNotification),
    TransferProposalCreated(TransferProposalCreatedNotification),
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProposalCreatedNotification {
    pub proposal_id: ProposalId,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferProposalCreatedNotification {
    pub proposal_id: ProposalId,
    pub account_id: AccountId,
    pub transfer_id: TransferId,
}

impl Display for NotificationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationType::SystemMessage => write!(f, "{}", SYSTEM_MESSAGE_NOTIFICATION_TYPE),
            NotificationType::TransferProposalCreated(_) => {
                write!(f, "{}", TRANSFER_PROPOSAL_CREATED_NOTIFICATION_TYPE)
            }
            NotificationType::ProposalCreated(_) => {
                write!(f, "{}", PROPOSAL_CREATED_NOTIFICATION_TYPE)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_string_representation() {
        assert_eq!(
            NotificationType::SystemMessage.to_string(),
            "system-message"
        );
        assert_eq!(
            NotificationType::ProposalCreated(ProposalCreatedNotification {
                proposal_id: [0; 16]
            })
            .to_string(),
            "proposal-created"
        );
        assert_eq!(
            NotificationType::TransferProposalCreated(TransferProposalCreatedNotification {
                proposal_id: [0; 16],
                account_id: [1; 16],
                transfer_id: [2; 16],
            })
            .to_string(),
            "transfer-proposal-created"
        );
    }
}
