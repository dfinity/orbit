use crate::core::ic_cdk::api::trap;
use crate::models::{ProposalOperation, ProposalOperationType};
use crate::{
    models::{NotificationType, Proposal},
    repositories::PROPOSAL_REPOSITORY,
};
use ic_canister_core::repository::Repository;
use uuid::Uuid;
use wallet_api::{NotificationTypeDTO, ProposalCreatedNotificationDTO};

impl From<NotificationType> for NotificationTypeDTO {
    fn from(model: NotificationType) -> Self {
        match model {
            NotificationType::SystemMessage => NotificationTypeDTO::SystemMessage,
            NotificationType::ProposalCreated(ctx) => {
                let proposal = PROPOSAL_REPOSITORY
                    .get(&Proposal::key(ctx.proposal_id))
                    .unwrap_or_else(|| {
                        trap(&format!(
                            "Proposal not found: {}",
                            Uuid::from_bytes(ctx.proposal_id).hyphenated()
                        ))
                    });

                let account_id = match &proposal.operation {
                    ProposalOperation::Transfer(operation) => Some(operation.input.from_account_id),
                    ProposalOperation::EditAccount(operation) => Some(operation.input.account_id),
                    _ => None,
                };

                let user_id: Option<[u8; 16]> = match &proposal.operation {
                    ProposalOperation::EditUser(operation) => Some(operation.input.user_id),
                    _ => None,
                };

                NotificationTypeDTO::ProposalCreated(ProposalCreatedNotificationDTO {
                    proposal_id: Uuid::from_bytes(ctx.proposal_id).to_string(),
                    operation_type: ProposalOperationType::from(proposal.operation).into(),
                    account_id: account_id.map(|id| Uuid::from_bytes(id).to_string()),
                    user_id: user_id.map(|id| Uuid::from_bytes(id).to_string()),
                })
            }
        }
    }
}
