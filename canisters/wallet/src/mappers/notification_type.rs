use crate::{
    models::{
        NotificationType, ACCOUNT_PROPOSAL_CREATED_NOTIFICATION_TYPE,
        PROPOSAL_CREATED_NOTIFICATION_TYPE, SYSTEM_MESSAGE_NOTIFICATION_TYPE,
        TRANSFER_PROPOSAL_CREATED_NOTIFICATION_TYPE,
    },
    transport::{
        AccountProposalCreatedNotificationDTO, NotificationTypeDTO, NotificationTypeInput,
        ProposalCreatedNotificationDTO, TransferProposalCreatedNotificationDTO,
    },
};
use std::fmt::{Display, Formatter};
use uuid::Uuid;

impl Display for NotificationTypeInput {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NotificationTypeInput::SystemMessage => {
                write!(f, "{}", SYSTEM_MESSAGE_NOTIFICATION_TYPE)
            }
            NotificationTypeInput::TransferProposalCreated => {
                write!(f, "{}", TRANSFER_PROPOSAL_CREATED_NOTIFICATION_TYPE)
            }
            NotificationTypeInput::ProposalCreated => {
                write!(f, "{}", PROPOSAL_CREATED_NOTIFICATION_TYPE)
            }
            NotificationTypeInput::AccountProposalCreated => {
                write!(f, "{}", ACCOUNT_PROPOSAL_CREATED_NOTIFICATION_TYPE)
            }
        }
    }
}

impl From<NotificationType> for NotificationTypeDTO {
    fn from(model: NotificationType) -> Self {
        match model {
            NotificationType::SystemMessage => NotificationTypeDTO::SystemMessage,
            NotificationType::ProposalCreated(ctx) => {
                NotificationTypeDTO::ProposalCreated(ProposalCreatedNotificationDTO {
                    proposal_id: Uuid::from_bytes(ctx.proposal_id).to_string(),
                })
            }
            NotificationType::TransferProposalCreated(ctx) => {
                NotificationTypeDTO::TransferProposalCreated(
                    TransferProposalCreatedNotificationDTO {
                        proposal_id: Uuid::from_bytes(ctx.proposal_id).to_string(),
                        account_id: Uuid::from_bytes(ctx.account_id).to_string(),
                    },
                )
            }
            NotificationType::AccountProposalCreated(proposal_id, account_id) => {
                NotificationTypeDTO::AccountProposalCreated(AccountProposalCreatedNotificationDTO {
                    account_id: Uuid::from_bytes(account_id).to_string(),
                    proposal_id: Uuid::from_bytes(proposal_id).to_string(),
                })
            }
        }
    }
}
