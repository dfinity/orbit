use super::HelperMapper;
use crate::{
    core::ic_cdk::api::trap,
    models::{
        access_control::{
            AccountActionSpecifier, AccountSpecifier, CommonActionSpecifier,
            ProposalActionSpecifier, ResourceSpecifier, ResourceType, TransferActionSpecifier,
            UpgradeActionSpecifier,
        },
        specifier::{AddressSpecifier, CommonSpecifier},
        Transfer,
    },
    repositories::TRANSFER_REPOSITORY,
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

        let transfer = TRANSFER_REPOSITORY
            .get(&Transfer::key(transfer_id))
            .expect("Invalid transfer");

        ResourceSpecifier::Transfer(TransferActionSpecifier::Read(
            AccountSpecifier::Id([transfer.from_account].to_vec()),
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
        let user_id = *HelperMapper::to_uuid(input.user_id.to_owned())
            .expect("Invalid user id")
            .as_bytes();

        ResourceSpecifier::Common(
            ResourceType::User,
            AccountActionSpecifier::Read(CommonSpecifier::Id([user_id].to_vec())),
        )
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
            ProposalOperationInput::AddAccessPolicy(_) => ResourceSpecifier::Common(
                ResourceType::AccessPolicy,
                AccountActionSpecifier::Create,
            ),
            ProposalOperationInput::EditAccessPolicy(input) => {
                let access_policy_id = *HelperMapper::to_uuid(input.policy_id.to_owned())
                    .expect("Invalid access policy id")
                    .as_bytes();

                ResourceSpecifier::Common(
                    ResourceType::AccessPolicy,
                    AccountActionSpecifier::Update(CommonSpecifier::Id(
                        [access_policy_id].to_vec(),
                    )),
                )
            }
            ProposalOperationInput::RemoveAccessPolicy(input) => {
                let access_policy_id = *HelperMapper::to_uuid(input.policy_id.to_owned())
                    .expect("Invalid access policy id")
                    .as_bytes();

                ResourceSpecifier::Common(
                    ResourceType::AccessPolicy,
                    AccountActionSpecifier::Delete(CommonSpecifier::Id(
                        [access_policy_id].to_vec(),
                    )),
                )
            }
            ProposalOperationInput::AddProposalPolicy(_) => ResourceSpecifier::Common(
                ResourceType::ProposalPolicy,
                AccountActionSpecifier::Create,
            ),
            ProposalOperationInput::EditProposalPolicy(input) => {
                let proposal_policy_id = *HelperMapper::to_uuid(input.policy_id.to_owned())
                    .expect("Invalid proposal policy id")
                    .as_bytes();

                ResourceSpecifier::Common(
                    ResourceType::ProposalPolicy,
                    AccountActionSpecifier::Update(CommonSpecifier::Id(
                        [proposal_policy_id].to_vec(),
                    )),
                )
            }
            ProposalOperationInput::RemoveProposalPolicy(input) => {
                let proposal_policy_id = *HelperMapper::to_uuid(input.policy_id.to_owned())
                    .expect("Invalid proposal policy id")
                    .as_bytes();

                ResourceSpecifier::Common(
                    ResourceType::ProposalPolicy,
                    AccountActionSpecifier::Delete(CommonSpecifier::Id(
                        [proposal_policy_id].to_vec(),
                    )),
                )
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

impl From<&wallet_api::GetAccessPolicyInput> for ResourceSpecifier {
    fn from(input: &wallet_api::GetAccessPolicyInput) -> Self {
        let access_policy_id = *HelperMapper::to_uuid(input.id.to_owned())
            .expect("Invalid access policy id")
            .as_bytes();

        ResourceSpecifier::Common(
            ResourceType::AccessPolicy,
            AccountActionSpecifier::Read(CommonSpecifier::Id([access_policy_id].to_vec())),
        )
    }
}

impl From<&wallet_api::GetProposalPolicyInput> for ResourceSpecifier {
    fn from(input: &wallet_api::GetProposalPolicyInput) -> Self {
        let proposal_policy_id = *HelperMapper::to_uuid(input.id.to_owned())
            .expect("Invalid proposal policy id")
            .as_bytes();

        ResourceSpecifier::Common(
            ResourceType::ProposalPolicy,
            AccountActionSpecifier::Read(CommonSpecifier::Id([proposal_policy_id].to_vec())),
        )
    }
}

impl From<&wallet_api::GetUserGroupInput> for ResourceSpecifier {
    fn from(input: &wallet_api::GetUserGroupInput) -> Self {
        let user_group_id = *HelperMapper::to_uuid(input.user_group_id.to_owned())
            .expect("Invalid user group id")
            .as_bytes();

        ResourceSpecifier::Common(
            ResourceType::UserGroup,
            CommonActionSpecifier::Read(CommonSpecifier::Id([user_group_id].to_vec())),
        )
    }
}

impl From<&wallet_api::VoteOnProposalInput> for ResourceSpecifier {
    fn from(input: &wallet_api::VoteOnProposalInput) -> Self {
        let proposal_id = *HelperMapper::to_uuid(input.proposal_id.to_owned())
            .expect("Invalid proposal id")
            .as_bytes();

        ResourceSpecifier::Proposal(ProposalActionSpecifier::Read(CommonSpecifier::Id(
            [proposal_id].to_vec(),
        )))
    }
}
