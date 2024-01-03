use candid::{CandidType, Deserialize};
use ic_canister_core::types::UUID;
use ic_canister_macros::stable_object;
use std::fmt::{Display, Formatter};
use wallet_api::{PROPOSAL_CREATED_NOTIFICATION_TYPE, SYSTEM_MESSAGE_NOTIFICATION_TYPE};

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NotificationType {
    SystemMessage,
    ProposalCreated(ProposalCreatedNotification),
}

#[stable_object]
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProposalCreatedNotification {
    pub proposal_id: UUID,
}

impl Display for NotificationType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationType::SystemMessage => write!(f, "{}", SYSTEM_MESSAGE_NOTIFICATION_TYPE),
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
    }
}
