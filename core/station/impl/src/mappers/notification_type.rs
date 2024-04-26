use crate::models::{ProposalOperation, ProposalOperationType};
use crate::{
    models::{NotificationType, Proposal},
    repositories::PROPOSAL_REPOSITORY,
};
use orbit_essentials::repository::Repository;
use station_api::{NotificationTypeDTO, ProposalCreatedNotificationDTO};
use uuid::Uuid;

use super::notification::NotificationMapperError;

impl TryFrom<NotificationType> for NotificationTypeDTO {
    type Error = NotificationMapperError;
    fn try_from(model: NotificationType) -> Result<NotificationTypeDTO, NotificationMapperError> {
        Ok(match model {
            NotificationType::SystemMessage => NotificationTypeDTO::SystemMessage,
            NotificationType::ProposalCreated(ctx) => {
                let proposal = PROPOSAL_REPOSITORY
                    .get(&Proposal::key(ctx.proposal_id))
                    .ok_or(NotificationMapperError::ProposalNotFound {
                        proposal_id: ctx.proposal_id,
                    })?;

                let account_id = match &proposal.operation {
                    ProposalOperation::Transfer(operation) => Some(operation.input.from_account_id),
                    ProposalOperation::EditAccount(operation) => Some(operation.input.account_id),
                    ProposalOperation::AddAccount(_)
                    | ProposalOperation::AddAddressBookEntry(_)
                    | ProposalOperation::EditAddressBookEntry(_)
                    | ProposalOperation::RemoveAddressBookEntry(_)
                    | ProposalOperation::EditUser(_)
                    | ProposalOperation::AddProposalPolicy(_)
                    | ProposalOperation::AddUser(_)
                    | ProposalOperation::AddUserGroup(_)
                    | ProposalOperation::EditAccessPolicy(_)
                    | ProposalOperation::EditProposalPolicy(_)
                    | ProposalOperation::EditUserGroup(_)
                    | ProposalOperation::RemoveProposalPolicy(_)
                    | ProposalOperation::RemoveUserGroup(_)
                    | ProposalOperation::ChangeCanister(_) => None,
                };

                let user_id: Option<[u8; 16]> = match &proposal.operation {
                    ProposalOperation::EditUser(operation) => Some(operation.input.user_id),
                    ProposalOperation::AddAccount(_)
                    | ProposalOperation::AddAddressBookEntry(_)
                    | ProposalOperation::AddProposalPolicy(_)
                    | ProposalOperation::AddUser(_)
                    | ProposalOperation::AddUserGroup(_)
                    | ProposalOperation::EditAccessPolicy(_)
                    | ProposalOperation::EditAccount(_)
                    | ProposalOperation::EditAddressBookEntry(_)
                    | ProposalOperation::RemoveAddressBookEntry(_)
                    | ProposalOperation::EditProposalPolicy(_)
                    | ProposalOperation::EditUserGroup(_)
                    | ProposalOperation::RemoveProposalPolicy(_)
                    | ProposalOperation::RemoveUserGroup(_)
                    | ProposalOperation::Transfer(_)
                    | ProposalOperation::ChangeCanister(_) => None,
                };

                NotificationTypeDTO::ProposalCreated(ProposalCreatedNotificationDTO {
                    proposal_id: Uuid::from_bytes(ctx.proposal_id).to_string(),
                    operation_type: ProposalOperationType::from(proposal.operation).into(),
                    account_id: account_id.map(|id| Uuid::from_bytes(id).to_string()),
                    user_id: user_id.map(|id| Uuid::from_bytes(id).to_string()),
                })
            }
        })
    }
}
