use orbit_essentials::storable;
use orbit_essentials::types::UUID;
use station_api::{PROPOSAL_CREATED_NOTIFICATION_TYPE, SYSTEM_MESSAGE_NOTIFICATION_TYPE};
use std::fmt::{Display, Formatter};

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NotificationType {
    SystemMessage,
    ProposalCreated(ProposalCreatedNotification),
}

#[storable]
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
