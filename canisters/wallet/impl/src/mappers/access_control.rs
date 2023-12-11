use super::HelperMapper;
use crate::{
    core::{ic_cdk::api::trap, CallContext},
    models::{
        access_control::{
            AccountActionSpecifier, AccountSpecifier, ProposalActionSpecifier, ResourceSpecifier,
            ResourceType, TransferActionSpecifier, UpgradeActionSpecifier,
        },
        specifier::{AddressSpecifier, CommonSpecifier},
        Transfer,
    },
    repositories::TRANSFER_REPOSITORY,
    services::USER_SERVICE,
};
use ic_canister_core::repository::Repository;
use ic_canister_core::types::UUID;
use wallet_api::ProposalOperationInput;

impl From<&wallet_api::GetAccountInput> for ResourceSpecifier {
    fn from(input: &wallet_api::GetAccountInput) -> Self {
        let account_id = *HelperMapper::to_uuid(input.account_id.to_owned())
            .expect("Invalid account id")
            .as_bytes();

        ResourceSpecifier::Common(
            ResourceType::Account,
            AccountActionSpecifier::Read(CommonSpecifier::Id([account_id].to_vec())),
        )
    }
}

impl From<&wallet_api::FetchAccountBalancesInput> for ResourceSpecifier {
    fn from(input: &wallet_api::FetchAccountBalancesInput) -> Self {
        let account_ids = input
            .account_ids
            .iter()
            .map(|account_id| {
                let account_id = *HelperMapper::to_uuid(account_id.to_owned())
                    .expect("Invalid account id")
                    .as_bytes();

                account_id
            })
            .collect::<Vec<UUID>>();

        ResourceSpecifier::Common(
            ResourceType::Account,
            AccountActionSpecifier::Read(CommonSpecifier::Id(account_ids.to_vec())),
        )
    }
}

impl From<&wallet_api::GetTransferInput> for ResourceSpecifier {
    fn from(input: &wallet_api::GetTransferInput) -> Self {
        let transfer_id = *HelperMapper::to_uuid(input.transfer_id.to_owned())
            .expect("Invalid transfer id")
            .as_bytes();

        let account = match TRANSFER_REPOSITORY.get(&Transfer::key(transfer_id)) {
            Some(transfer) => AccountSpecifier::Id([transfer.from_account].to_vec()),
            // When the account is not found, we assume that the user is trying to read any account, this
            // avoids leaking information about the existence of a transfer.
            None => AccountSpecifier::Any,
        };

        ResourceSpecifier::Transfer(TransferActionSpecifier::Read(
            account,
            AddressSpecifier::Any,
        ))
    }
}

impl From<&wallet_api::GetTransfersInput> for ResourceSpecifier {
    fn from(input: &wallet_api::GetTransfersInput) -> Self {
        let transfer_ids = input
            .transfer_ids
            .iter()
            .map(|transfer_id| {
                let transfer_id = *HelperMapper::to_uuid(transfer_id.to_owned())
                    .expect("Invalid transfer id")
                    .as_bytes();

                transfer_id
            })
            .collect::<Vec<UUID>>();

        let transfers = transfer_ids
            .iter()
            .map(|transfer_id| {
                TRANSFER_REPOSITORY
                    .get(&Transfer::key(*transfer_id))
                    .unwrap_or_else(|| trap("Failed to unwrap transfers input"))
            })
            .collect::<Vec<Transfer>>();

        let account_ids = transfers
            .iter()
            .map(|transfer| transfer.from_account)
            .collect();

        ResourceSpecifier::Transfer(TransferActionSpecifier::Read(
            AccountSpecifier::Id(account_ids),
            AddressSpecifier::Any,
        ))
    }
}

impl From<&wallet_api::ListAccountTransfersInput> for ResourceSpecifier {
    fn from(input: &wallet_api::ListAccountTransfersInput) -> Self {
        let account_id = *HelperMapper::to_uuid(input.account_id.to_owned())
            .expect("Invalid account id")
            .as_bytes();

        ResourceSpecifier::Transfer(TransferActionSpecifier::Read(
            AccountSpecifier::Id([account_id].to_vec()),
            AddressSpecifier::Any,
        ))
    }
}

impl From<&wallet_api::GetUserInput> for ResourceSpecifier {
    fn from(input: &wallet_api::GetUserInput) -> Self {
        match &input.user_id {
            Some(user_id) => {
                let user_id = *HelperMapper::to_uuid(user_id.to_owned())
                    .expect("Invalid user id")
                    .as_bytes();

                ResourceSpecifier::Common(
                    ResourceType::User,
                    AccountActionSpecifier::Read(CommonSpecifier::Id([user_id].to_vec())),
                )
            }
            _ => {
                let ctx = CallContext::get();
                let user = USER_SERVICE.get_user_by_identity(&ctx.caller(), &ctx);
                match user {
                    Ok(user) => ResourceSpecifier::Common(
                        ResourceType::User,
                        AccountActionSpecifier::Read(CommonSpecifier::Id([user.id].to_vec())),
                    ),
                    Err(_) => ResourceSpecifier::Common(
                        ResourceType::User,
                        AccountActionSpecifier::Read(CommonSpecifier::Any),
                    ),
                }
            }
        }
    }
}

impl From<&wallet_api::CreateProposalInput> for ResourceSpecifier {
    fn from(input: &wallet_api::CreateProposalInput) -> Self {
        match &input.operation {
            ProposalOperationInput::AddAccount(_) => {
                ResourceSpecifier::Common(ResourceType::Account, AccountActionSpecifier::Create)
            }
            ProposalOperationInput::EditAccount(input) => {
                let account_id = *HelperMapper::to_uuid(input.account_id.to_owned())
                    .expect("Invalid account id")
                    .as_bytes();

                ResourceSpecifier::Common(
                    ResourceType::Account,
                    AccountActionSpecifier::Update(CommonSpecifier::Id([account_id].to_vec())),
                )
            }
            ProposalOperationInput::Transfer(input) => {
                let account_id = *HelperMapper::to_uuid(input.from_account_id.to_owned())
                    .expect("Invalid account id")
                    .as_bytes();

                ResourceSpecifier::Transfer(TransferActionSpecifier::Create(
                    AccountSpecifier::Id([account_id].to_vec()),
                    AddressSpecifier::Any,
                ))
            }
            ProposalOperationInput::AddUser(_) => {
                ResourceSpecifier::Common(ResourceType::User, AccountActionSpecifier::Create)
            }
            ProposalOperationInput::EditUser(input) => {
                let user_id = *HelperMapper::to_uuid(input.id.to_owned())
                    .expect("Invalid user id")
                    .as_bytes();

                ResourceSpecifier::Common(
                    ResourceType::User,
                    AccountActionSpecifier::Update(CommonSpecifier::Id([user_id].to_vec())),
                )
            }
            ProposalOperationInput::EditUserStatus(input) => {
                let user_id = *HelperMapper::to_uuid(input.id.to_owned())
                    .expect("Invalid user id")
                    .as_bytes();

                ResourceSpecifier::Common(
                    ResourceType::User,
                    AccountActionSpecifier::Update(CommonSpecifier::Id([user_id].to_vec())),
                )
            }
            ProposalOperationInput::AddUserGroup(_) => {
                ResourceSpecifier::Common(ResourceType::UserGroup, AccountActionSpecifier::Create)
            }
            ProposalOperationInput::EditUserGroup(_) => ResourceSpecifier::Common(
                ResourceType::UserGroup,
                AccountActionSpecifier::Update(CommonSpecifier::Any),
            ),
            ProposalOperationInput::RemoveUserGroup(_) => ResourceSpecifier::Common(
                ResourceType::UserGroup,
                AccountActionSpecifier::Delete(CommonSpecifier::Any),
            ),
            ProposalOperationInput::Upgrade(_) => {
                ResourceSpecifier::Upgrade(UpgradeActionSpecifier::Create)
            }
        }
    }
}

impl From<&wallet_api::ListAccountProposalsInput> for ResourceSpecifier {
    fn from(input: &wallet_api::ListAccountProposalsInput) -> Self {
        let account_id = *HelperMapper::to_uuid(input.account_id.to_owned())
            .expect("Invalid account id")
            .as_bytes();

        ResourceSpecifier::Common(
            ResourceType::Account,
            AccountActionSpecifier::Read(CommonSpecifier::Id([account_id].to_vec())),
        )
    }
}

impl From<&wallet_api::GetProposalInput> for ResourceSpecifier {
    fn from(input: &wallet_api::GetProposalInput) -> Self {
        let proposal_id = *HelperMapper::to_uuid(input.proposal_id.to_owned())
            .expect("Invalid proposal id")
            .as_bytes();

        ResourceSpecifier::Proposal(ProposalActionSpecifier::Read(CommonSpecifier::Id(
            [proposal_id].to_vec(),
        )))
    }
}
