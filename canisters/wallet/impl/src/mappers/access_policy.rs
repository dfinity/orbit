use super::HelperMapper;
use crate::{
    core::ic_cdk::api::trap,
    models::{
        access_policy::{
            AccessPolicy, AccessPolicyResourceAction, AccountResourceAction, Allow, AuthScope,
            ChangeCanisterResourceAction, ProposalResourceAction, Resource, ResourceAction,
            ResourceId, SettingsResourceAction, UserResourceAction,
        },
        Transfer,
    },
    repositories::TRANSFER_REPOSITORY,
};
use ic_canister_core::repository::Repository;
use ic_canister_core::types::UUID;
use uuid::Uuid;
use wallet_api::ProposalOperationInput;

impl From<&wallet_api::GetAccountInput> for Resource {
    fn from(input: &wallet_api::GetAccountInput) -> Self {
        Resource::Account(AccountResourceAction::Read(ResourceId::Id(
            *HelperMapper::to_uuid(input.account_id.to_owned())
                .expect("Invalid account id")
                .as_bytes(),
        )))
    }
}

impl From<&wallet_api::ListAccountTransfersInput> for Resource {
    fn from(input: &wallet_api::ListAccountTransfersInput) -> Self {
        Resource::Account(AccountResourceAction::Read(ResourceId::Id(
            *HelperMapper::to_uuid(input.account_id.to_owned())
                .expect("Invalid account id")
                .as_bytes(),
        )))
    }
}

impl From<&wallet_api::GetUserInput> for Resource {
    fn from(input: &wallet_api::GetUserInput) -> Self {
        Resource::User(UserResourceAction::Read(ResourceId::Id(
            *HelperMapper::to_uuid(input.user_id.to_owned())
                .expect("Invalid user id")
                .as_bytes(),
        )))
    }
}

impl From<&wallet_api::GetProposalInput> for Resource {
    fn from(input: &wallet_api::GetProposalInput) -> Self {
        Resource::Proposal(ProposalResourceAction::Read(ResourceId::Id(
            *HelperMapper::to_uuid(input.proposal_id.to_owned())
                .expect("Invalid proposal id")
                .as_bytes(),
        )))
    }
}

impl From<&wallet_api::GetProposalPolicyInput> for Resource {
    fn from(input: &wallet_api::GetProposalPolicyInput) -> Self {
        Resource::ProposalPolicy(ResourceAction::Read(ResourceId::Id(
            *HelperMapper::to_uuid(input.id.to_owned())
                .expect("Invalid proposal policy id")
                .as_bytes(),
        )))
    }
}

impl From<&wallet_api::GetUserGroupInput> for Resource {
    fn from(input: &wallet_api::GetUserGroupInput) -> Self {
        Resource::UserGroup(ResourceAction::Read(ResourceId::Id(
            *HelperMapper::to_uuid(input.user_group_id.to_owned())
                .expect("Invalid user group id")
                .as_bytes(),
        )))
    }
}

impl From<&wallet_api::VoteOnProposalInput> for Resource {
    fn from(input: &wallet_api::VoteOnProposalInput) -> Self {
        Resource::Proposal(ProposalResourceAction::Read(ResourceId::Id(
            *HelperMapper::to_uuid(input.proposal_id.to_owned())
                .expect("Invalid proposal id")
                .as_bytes(),
        )))
    }
}

impl From<&wallet_api::GetAddressBookEntryInputDTO> for Resource {
    fn from(input: &wallet_api::GetAddressBookEntryInputDTO) -> Self {
        Resource::AddressBook(ResourceAction::Read(ResourceId::Id(
            *HelperMapper::to_uuid(input.address_book_entry_id.to_owned())
                .expect("Invalid address book entry id")
                .as_bytes(),
        )))
    }
}

impl From<&wallet_api::CreateProposalInput> for Resource {
    fn from(input: &wallet_api::CreateProposalInput) -> Self {
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

pub(crate) struct FetchAccountBalancesInputRef<'a>(pub &'a wallet_api::FetchAccountBalancesInput);

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

pub(crate) struct GetTransfersInputRef<'a>(pub &'a wallet_api::GetTransfersInput);

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

impl From<wallet_api::AuthScopeDTO> for AuthScope {
    fn from(dto: wallet_api::AuthScopeDTO) -> Self {
        match dto {
            wallet_api::AuthScopeDTO::Public => AuthScope::Public,
            wallet_api::AuthScopeDTO::Authenticated => AuthScope::Authenticated,
            wallet_api::AuthScopeDTO::Restricted => AuthScope::Restricted,
        }
    }
}

impl From<AuthScope> for wallet_api::AuthScopeDTO {
    fn from(auth: AuthScope) -> Self {
        match auth {
            AuthScope::Public => wallet_api::AuthScopeDTO::Public,
            AuthScope::Authenticated => wallet_api::AuthScopeDTO::Authenticated,
            AuthScope::Restricted => wallet_api::AuthScopeDTO::Restricted,
        }
    }
}

impl From<wallet_api::AllowDTO> for Allow {
    fn from(dto: wallet_api::AllowDTO) -> Self {
        Allow {
            auth_scope: dto.auth_scope.into(),
            users: dto
                .users
                .iter()
                .map(|id| {
                    *HelperMapper::to_uuid(id.to_owned())
                        .expect("Invalid user id")
                        .as_bytes()
                })
                .collect(),
            user_groups: dto
                .user_groups
                .iter()
                .map(|id| {
                    *HelperMapper::to_uuid(id.to_owned())
                        .expect("Invalid user group id")
                        .as_bytes()
                })
                .collect(),
        }
    }
}

impl From<Allow> for wallet_api::AllowDTO {
    fn from(allow: Allow) -> Self {
        wallet_api::AllowDTO {
            auth_scope: allow.auth_scope.into(),
            users: allow
                .users
                .iter()
                .map(|id| Uuid::from_bytes(*id).hyphenated().to_string())
                .collect(),
            user_groups: allow
                .user_groups
                .iter()
                .map(|id| Uuid::from_bytes(*id).hyphenated().to_string())
                .collect(),
        }
    }
}

impl From<wallet_api::ResourceActionDTO> for ResourceAction {
    fn from(dto: wallet_api::ResourceActionDTO) -> Self {
        match dto {
            wallet_api::ResourceActionDTO::List => ResourceAction::List,
            wallet_api::ResourceActionDTO::Create => ResourceAction::Create,
            wallet_api::ResourceActionDTO::Read(id) => ResourceAction::Read(id.into()),
            wallet_api::ResourceActionDTO::Update(id) => ResourceAction::Update(id.into()),
            wallet_api::ResourceActionDTO::Delete(id) => ResourceAction::Delete(id.into()),
        }
    }
}

impl From<ResourceAction> for wallet_api::ResourceActionDTO {
    fn from(action: ResourceAction) -> Self {
        match action {
            ResourceAction::List => wallet_api::ResourceActionDTO::List,
            ResourceAction::Create => wallet_api::ResourceActionDTO::Create,
            ResourceAction::Read(id) => wallet_api::ResourceActionDTO::Read(id.into()),
            ResourceAction::Update(id) => wallet_api::ResourceActionDTO::Update(id.into()),
            ResourceAction::Delete(id) => wallet_api::ResourceActionDTO::Delete(id.into()),
        }
    }
}

impl From<wallet_api::ResourceIdDTO> for ResourceId {
    fn from(dto: wallet_api::ResourceIdDTO) -> Self {
        match dto {
            wallet_api::ResourceIdDTO::Any => ResourceId::Any,
            wallet_api::ResourceIdDTO::Id(id) => ResourceId::Id(
                *HelperMapper::to_uuid(id)
                    .expect("Invalid resource id")
                    .as_bytes(),
            ),
        }
    }
}

impl From<ResourceId> for wallet_api::ResourceIdDTO {
    fn from(id: ResourceId) -> Self {
        match id {
            ResourceId::Any => wallet_api::ResourceIdDTO::Any,
            ResourceId::Id(id) => {
                wallet_api::ResourceIdDTO::Id(Uuid::from_bytes(id).hyphenated().to_string())
            }
        }
    }
}

impl From<wallet_api::AccessPolicyResourceActionDTO> for AccessPolicyResourceAction {
    fn from(dto: wallet_api::AccessPolicyResourceActionDTO) -> Self {
        match dto {
            wallet_api::AccessPolicyResourceActionDTO::Read => AccessPolicyResourceAction::Read,
            wallet_api::AccessPolicyResourceActionDTO::Update => AccessPolicyResourceAction::Update,
        }
    }
}

impl From<AccessPolicyResourceAction> for wallet_api::AccessPolicyResourceActionDTO {
    fn from(action: AccessPolicyResourceAction) -> Self {
        match action {
            AccessPolicyResourceAction::Read => wallet_api::AccessPolicyResourceActionDTO::Read,
            AccessPolicyResourceAction::Update => wallet_api::AccessPolicyResourceActionDTO::Update,
        }
    }
}

impl From<wallet_api::UserResourceActionDTO> for UserResourceAction {
    fn from(dto: wallet_api::UserResourceActionDTO) -> Self {
        match dto {
            wallet_api::UserResourceActionDTO::List => UserResourceAction::List,
            wallet_api::UserResourceActionDTO::Create => UserResourceAction::Create,
            wallet_api::UserResourceActionDTO::Read(id) => UserResourceAction::Read(id.into()),
            wallet_api::UserResourceActionDTO::Update(id) => UserResourceAction::Update(id.into()),
        }
    }
}

impl From<UserResourceAction> for wallet_api::UserResourceActionDTO {
    fn from(action: UserResourceAction) -> Self {
        match action {
            UserResourceAction::List => wallet_api::UserResourceActionDTO::List,
            UserResourceAction::Create => wallet_api::UserResourceActionDTO::Create,
            UserResourceAction::Read(id) => wallet_api::UserResourceActionDTO::Read(id.into()),
            UserResourceAction::Update(id) => wallet_api::UserResourceActionDTO::Update(id.into()),
        }
    }
}

impl From<wallet_api::AccountResourceActionDTO> for AccountResourceAction {
    fn from(dto: wallet_api::AccountResourceActionDTO) -> Self {
        match dto {
            wallet_api::AccountResourceActionDTO::List => AccountResourceAction::List,
            wallet_api::AccountResourceActionDTO::Create => AccountResourceAction::Create,
            wallet_api::AccountResourceActionDTO::Transfer(id) => {
                AccountResourceAction::Transfer(id.into())
            }
            wallet_api::AccountResourceActionDTO::Read(id) => {
                AccountResourceAction::Read(id.into())
            }
            wallet_api::AccountResourceActionDTO::Update(id) => {
                AccountResourceAction::Update(id.into())
            }
        }
    }
}

impl From<AccountResourceAction> for wallet_api::AccountResourceActionDTO {
    fn from(action: AccountResourceAction) -> Self {
        match action {
            AccountResourceAction::List => wallet_api::AccountResourceActionDTO::List,
            AccountResourceAction::Create => wallet_api::AccountResourceActionDTO::Create,
            AccountResourceAction::Transfer(id) => {
                wallet_api::AccountResourceActionDTO::Transfer(id.into())
            }
            AccountResourceAction::Read(id) => {
                wallet_api::AccountResourceActionDTO::Read(id.into())
            }
            AccountResourceAction::Update(id) => {
                wallet_api::AccountResourceActionDTO::Update(id.into())
            }
        }
    }
}

impl From<wallet_api::SettingsResourceActionDTO> for SettingsResourceAction {
    fn from(dto: wallet_api::SettingsResourceActionDTO) -> Self {
        match dto {
            wallet_api::SettingsResourceActionDTO::SystemInfo => SettingsResourceAction::SystemInfo,
            wallet_api::SettingsResourceActionDTO::Capabilities => {
                SettingsResourceAction::Capabilities
            }
        }
    }
}

impl From<SettingsResourceAction> for wallet_api::SettingsResourceActionDTO {
    fn from(action: SettingsResourceAction) -> Self {
        match action {
            SettingsResourceAction::SystemInfo => wallet_api::SettingsResourceActionDTO::SystemInfo,
            SettingsResourceAction::Capabilities => {
                wallet_api::SettingsResourceActionDTO::Capabilities
            }
        }
    }
}

impl From<wallet_api::ChangeCanisterResourceActionDTO> for ChangeCanisterResourceAction {
    fn from(_: wallet_api::ChangeCanisterResourceActionDTO) -> Self {
        ChangeCanisterResourceAction::Create
    }
}

impl From<ChangeCanisterResourceAction> for wallet_api::ChangeCanisterResourceActionDTO {
    fn from(_: ChangeCanisterResourceAction) -> Self {
        wallet_api::ChangeCanisterResourceActionDTO::Create
    }
}

impl From<wallet_api::ProposalResourceActionDTO> for ProposalResourceAction {
    fn from(dto: wallet_api::ProposalResourceActionDTO) -> Self {
        match dto {
            wallet_api::ProposalResourceActionDTO::List => ProposalResourceAction::List,
            wallet_api::ProposalResourceActionDTO::Read(id) => {
                ProposalResourceAction::Read(id.into())
            }
        }
    }
}

impl From<ProposalResourceAction> for wallet_api::ProposalResourceActionDTO {
    fn from(action: ProposalResourceAction) -> Self {
        match action {
            ProposalResourceAction::List => wallet_api::ProposalResourceActionDTO::List,
            ProposalResourceAction::Read(id) => {
                wallet_api::ProposalResourceActionDTO::Read(id.into())
            }
        }
    }
}

impl From<wallet_api::ResourceDTO> for Resource {
    fn from(dto: wallet_api::ResourceDTO) -> Self {
        match dto {
            wallet_api::ResourceDTO::User(action) => Resource::User(action.into()),
            wallet_api::ResourceDTO::Account(action) => Resource::Account(action.into()),
            wallet_api::ResourceDTO::AccessPolicy(action) => Resource::AccessPolicy(action.into()),
            wallet_api::ResourceDTO::ProposalPolicy(action) => {
                Resource::ProposalPolicy(action.into())
            }
            wallet_api::ResourceDTO::UserGroup(action) => Resource::UserGroup(action.into()),
            wallet_api::ResourceDTO::AddressBook(action) => Resource::AddressBook(action.into()),
            wallet_api::ResourceDTO::ChangeCanister(action) => {
                Resource::ChangeCanister(action.into())
            }
            wallet_api::ResourceDTO::Proposal(action) => Resource::Proposal(action.into()),
            wallet_api::ResourceDTO::Settings(action) => Resource::Settings(action.into()),
        }
    }
}

impl From<Resource> for wallet_api::ResourceDTO {
    fn from(resource: Resource) -> Self {
        match resource {
            Resource::User(action) => wallet_api::ResourceDTO::User(action.into()),
            Resource::Account(action) => wallet_api::ResourceDTO::Account(action.into()),
            Resource::AccessPolicy(action) => wallet_api::ResourceDTO::AccessPolicy(action.into()),
            Resource::ProposalPolicy(action) => {
                wallet_api::ResourceDTO::ProposalPolicy(action.into())
            }
            Resource::UserGroup(action) => wallet_api::ResourceDTO::UserGroup(action.into()),
            Resource::AddressBook(action) => wallet_api::ResourceDTO::AddressBook(action.into()),
            Resource::ChangeCanister(action) => {
                wallet_api::ResourceDTO::ChangeCanister(action.into())
            }
            Resource::Proposal(action) => wallet_api::ResourceDTO::Proposal(action.into()),
            Resource::Settings(action) => wallet_api::ResourceDTO::Settings(action.into()),
        }
    }
}

impl From<AccessPolicy> for wallet_api::AccessPolicyDTO {
    fn from(policy: AccessPolicy) -> Self {
        wallet_api::AccessPolicyDTO {
            resource: policy.resource.into(),
            allow: policy.allow.into(),
        }
    }
}
