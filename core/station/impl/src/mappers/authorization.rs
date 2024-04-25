use super::HelperMapper;
use crate::{
    core::ic_cdk::api::trap,
    models::{
        resource::{
            AccessPolicyResourceAction, AccountResourceAction, ChangeCanisterResourceAction,
            ProposalResourceAction, Resource, ResourceAction, ResourceId, SystemResourceAction,
            UserResourceAction,
        },
        Transfer,
    },
    repositories::TRANSFER_REPOSITORY,
};
use ic_canister_core::repository::Repository;
use ic_canister_core::types::UUID;
use station_api::{ProposalOperationInput, UserPrivilege};

pub const USER_PRIVILEGES: [UserPrivilege; 15] = [
    UserPrivilege::Capabilities,
    UserPrivilege::SystemInfo,
    UserPrivilege::ListUsers,
    UserPrivilege::AddUser,
    UserPrivilege::ListAccounts,
    UserPrivilege::AddAccount,
    UserPrivilege::ListAccessPolicies,
    UserPrivilege::ListProposalPolicies,
    UserPrivilege::AddProposalPolicy,
    UserPrivilege::ListUserGroups,
    UserPrivilege::AddUserGroup,
    UserPrivilege::ListAddressBookEntries,
    UserPrivilege::AddAddressBookEntry,
    UserPrivilege::ChangeCanister,
    UserPrivilege::ListProposals,
];

impl From<UserPrivilege> for Resource {
    fn from(privilege: UserPrivilege) -> Self {
        match privilege {
            UserPrivilege::Capabilities => Resource::System(SystemResourceAction::Capabilities),
            UserPrivilege::SystemInfo => Resource::System(SystemResourceAction::SystemInfo),
            UserPrivilege::ListUsers => Resource::User(UserResourceAction::List),
            UserPrivilege::AddUser => Resource::User(UserResourceAction::Create),
            UserPrivilege::ListAccounts => Resource::Account(AccountResourceAction::List),
            UserPrivilege::AddAccount => Resource::Account(AccountResourceAction::Create),
            UserPrivilege::ListAccessPolicies => {
                Resource::AccessPolicy(AccessPolicyResourceAction::Read)
            }
            UserPrivilege::ListProposalPolicies => Resource::ProposalPolicy(ResourceAction::List),
            UserPrivilege::AddProposalPolicy => Resource::ProposalPolicy(ResourceAction::Create),
            UserPrivilege::ListUserGroups => Resource::UserGroup(ResourceAction::List),
            UserPrivilege::AddUserGroup => Resource::UserGroup(ResourceAction::Create),
            UserPrivilege::ListAddressBookEntries => Resource::AddressBook(ResourceAction::List),
            UserPrivilege::AddAddressBookEntry => Resource::AddressBook(ResourceAction::Create),
            UserPrivilege::ChangeCanister => {
                Resource::ChangeCanister(ChangeCanisterResourceAction::Create)
            }
            UserPrivilege::ListProposals => Resource::Proposal(ProposalResourceAction::List),
        }
    }
}

impl From<&station_api::GetAccountInput> for Resource {
    fn from(input: &station_api::GetAccountInput) -> Self {
        Resource::Account(AccountResourceAction::Read(ResourceId::Id(
            *HelperMapper::to_uuid(input.account_id.to_owned())
                .expect("Invalid account id")
                .as_bytes(),
        )))
    }
}

impl From<&station_api::ListAccountTransfersInput> for Resource {
    fn from(input: &station_api::ListAccountTransfersInput) -> Self {
        Resource::Account(AccountResourceAction::Read(ResourceId::Id(
            *HelperMapper::to_uuid(input.account_id.to_owned())
                .expect("Invalid account id")
                .as_bytes(),
        )))
    }
}

impl From<&station_api::GetUserInput> for Resource {
    fn from(input: &station_api::GetUserInput) -> Self {
        Resource::User(UserResourceAction::Read(ResourceId::Id(
            *HelperMapper::to_uuid(input.user_id.to_owned())
                .expect("Invalid user id")
                .as_bytes(),
        )))
    }
}

impl From<&station_api::GetProposalInput> for Resource {
    fn from(input: &station_api::GetProposalInput) -> Self {
        Resource::Proposal(ProposalResourceAction::Read(ResourceId::Id(
            *HelperMapper::to_uuid(input.proposal_id.to_owned())
                .expect("Invalid proposal id")
                .as_bytes(),
        )))
    }
}

impl From<&station_api::GetProposalPolicyInput> for Resource {
    fn from(input: &station_api::GetProposalPolicyInput) -> Self {
        Resource::ProposalPolicy(ResourceAction::Read(ResourceId::Id(
            *HelperMapper::to_uuid(input.id.to_owned())
                .expect("Invalid proposal policy id")
                .as_bytes(),
        )))
    }
}

impl From<&station_api::GetUserGroupInput> for Resource {
    fn from(input: &station_api::GetUserGroupInput) -> Self {
        Resource::UserGroup(ResourceAction::Read(ResourceId::Id(
            *HelperMapper::to_uuid(input.user_group_id.to_owned())
                .expect("Invalid user group id")
                .as_bytes(),
        )))
    }
}

impl From<&station_api::VoteOnProposalInput> for Resource {
    fn from(input: &station_api::VoteOnProposalInput) -> Self {
        Resource::Proposal(ProposalResourceAction::Read(ResourceId::Id(
            *HelperMapper::to_uuid(input.proposal_id.to_owned())
                .expect("Invalid proposal id")
                .as_bytes(),
        )))
    }
}

impl From<&station_api::GetAddressBookEntryInputDTO> for Resource {
    fn from(input: &station_api::GetAddressBookEntryInputDTO) -> Self {
        Resource::AddressBook(ResourceAction::Read(ResourceId::Id(
            *HelperMapper::to_uuid(input.address_book_entry_id.to_owned())
                .expect("Invalid address book entry id")
                .as_bytes(),
        )))
    }
}

impl From<&station_api::CreateProposalInput> for Resource {
    fn from(input: &station_api::CreateProposalInput) -> Self {
        match &input.operation {
            ProposalOperationInput::AddAccount(_) => {
                Resource::Account(AccountResourceAction::Create)
            }
            ProposalOperationInput::EditAccount(input) => {
                Resource::Account(AccountResourceAction::Update(ResourceId::Id(
                    *HelperMapper::to_uuid(input.account_id.to_owned())
                        .expect("Invalid account id")
                        .as_bytes(),
                )))
            }
            ProposalOperationInput::AddAddressBookEntry(_) => {
                Resource::AddressBook(ResourceAction::Create)
            }
            ProposalOperationInput::EditAddressBookEntry(input) => {
                Resource::AddressBook(ResourceAction::Update(ResourceId::Id(
                    *HelperMapper::to_uuid(input.address_book_entry_id.to_owned())
                        .expect("Invalid address book entry id")
                        .as_bytes(),
                )))
            }
            ProposalOperationInput::RemoveAddressBookEntry(input) => {
                Resource::AddressBook(ResourceAction::Delete(ResourceId::Id(
                    *HelperMapper::to_uuid(input.address_book_entry_id.to_owned())
                        .expect("Invalid address book entry id")
                        .as_bytes(),
                )))
            }
            ProposalOperationInput::Transfer(input) => {
                Resource::Account(AccountResourceAction::Transfer(ResourceId::Id(
                    *HelperMapper::to_uuid(input.from_account_id.to_owned())
                        .expect("Invalid account id")
                        .as_bytes(),
                )))
            }
            ProposalOperationInput::AddUser(_) => Resource::User(UserResourceAction::Create),
            ProposalOperationInput::EditUser(input) => {
                Resource::User(UserResourceAction::Update(ResourceId::Id(
                    *HelperMapper::to_uuid(input.id.to_owned())
                        .expect("Invalid user id")
                        .as_bytes(),
                )))
            }
            ProposalOperationInput::AddUserGroup(_) => Resource::UserGroup(ResourceAction::Create),
            ProposalOperationInput::EditUserGroup(input) => {
                Resource::UserGroup(ResourceAction::Update(ResourceId::Id(
                    *HelperMapper::to_uuid(input.user_group_id.to_owned())
                        .expect("Invalid user group id")
                        .as_bytes(),
                )))
            }
            ProposalOperationInput::RemoveUserGroup(input) => {
                Resource::UserGroup(ResourceAction::Delete(ResourceId::Id(
                    *HelperMapper::to_uuid(input.user_group_id.to_owned())
                        .expect("Invalid user group id")
                        .as_bytes(),
                )))
            }
            ProposalOperationInput::ChangeCanister(_) => {
                Resource::ChangeCanister(ChangeCanisterResourceAction::Create)
            }
            ProposalOperationInput::EditAccessPolicy(_) => {
                Resource::AccessPolicy(AccessPolicyResourceAction::Update)
            }
            ProposalOperationInput::AddProposalPolicy(_) => {
                Resource::ProposalPolicy(ResourceAction::Create)
            }
            ProposalOperationInput::EditProposalPolicy(input) => {
                Resource::ProposalPolicy(ResourceAction::Update(ResourceId::Id(
                    *HelperMapper::to_uuid(input.policy_id.to_owned())
                        .expect("Invalid proposal policy id")
                        .as_bytes(),
                )))
            }
            ProposalOperationInput::RemoveProposalPolicy(input) => {
                Resource::ProposalPolicy(ResourceAction::Delete(ResourceId::Id(
                    *HelperMapper::to_uuid(input.policy_id.to_owned())
                        .expect("Invalid proposal policy id")
                        .as_bytes(),
                )))
            }
        }
    }
}

pub(crate) struct FetchAccountBalancesInputRef<'a>(pub &'a station_api::FetchAccountBalancesInput);

impl FetchAccountBalancesInputRef<'_> {
    pub fn to_resources(&self) -> Vec<Resource> {
        let account_ids = self
            .0
            .account_ids
            .iter()
            .map(|account_id| {
                let account_id = *HelperMapper::to_uuid(account_id.to_owned())
                    .expect("Invalid account id")
                    .as_bytes();

                account_id
            })
            .collect::<Vec<UUID>>();

        account_ids
            .iter()
            .map(|account_id| {
                Resource::Account(AccountResourceAction::Read(ResourceId::Id(*account_id)))
            })
            .collect()
    }
}

pub(crate) struct GetTransfersInputRef<'a>(pub &'a station_api::GetTransfersInput);

impl GetTransfersInputRef<'_> {
    pub fn to_resources(&self) -> Vec<Resource> {
        let transfer_ids = self
            .0
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
            .collect::<Vec<_>>();

        account_ids
            .iter()
            .map(|account_id| {
                Resource::Account(AccountResourceAction::Read(ResourceId::Id(*account_id)))
            })
            .collect()
    }
}
