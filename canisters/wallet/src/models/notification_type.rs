use candid::{CandidType, Deserialize};
use ic_canister_core::types::UUID;
use ic_canister_macros::stable_object;
use std::fmt::{Display, Formatter};

use super::{AccountId, ProposalId};

pub const SYSTEM_MESSAGE_NOTIFICATION_TYPE: &str = "system-message";
pub const PROPOSAL_CREATED_NOTIFICATION_TYPE: &str = "proposal-created";
pub const TRANSFER_PROPOSAL_CREATED_NOTIFICATION_TYPE: &str = "transfer-proposal-created";
pub const ACCOUNT_EDIT_PROPOSAL_CREATED_NOTIFICATION_TYPE: &str = "account-edit-proposal-created";

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NotificationType {
    SystemMessage,
    ProposalCreated(ProposalCreatedNotification),
    TransferProposalCreated(TransferProposalCreatedNotification),
    AccountEditProposalCreated(ProposalId, AccountId),
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProposalCreatedNotification {
    pub proposal_id: UUID,
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TransferProposalCreatedNotification {
    pub proposal_id: UUID,
    pub account_id: UUID,
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
            NotificationType::AccountEditProposalCreated(_, _) => {
                write!(f, "{}", ACCOUNT_EDIT_PROPOSAL_CREATED_NOTIFICATION_TYPE)
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
            })
            .to_string(),
            "transfer-proposal-created"
        );
        assert_eq!(
            NotificationType::AccountEditProposalCreated([0; 16], [1; 16]).to_string(),
            "account-edit-proposal-created"
        );
    }
}
