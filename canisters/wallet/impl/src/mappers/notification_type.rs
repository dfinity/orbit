use crate::models::NotificationType;
use uuid::Uuid;
use wallet_api::{
    AccountProposalCreatedNotificationDTO, NotificationTypeDTO, ProposalCreatedNotificationDTO,
    TransferProposalCreatedNotificationDTO,
};

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
