use super::HelperMapper;
use crate::{
    core::ic_cdk::api::trap,
    models::{
        access_policy::{
            AccessPolicy, AccessPolicyCallerPrivileges, AccessPolicyResourceAction,
            AccountResourceAction, Allow, AllowLevel, ChangeCanisterResourceAction,
            ProposalResourceAction, Resource, ResourceAction, ResourceActionType, ResourceId,
            ResourceType, ResourceTypeId, SettingsResourceAction, UserResourceAction,
        },
        Transfer,
    },
    repositories::TRANSFER_REPOSITORY,
};
use ic_canister_core::repository::Repository;
use ic_canister_core::types::UUID;
use uuid::Uuid;
use wallet_api::ProposalOperationInput;

impl From<AccessPolicyCallerPrivileges> for wallet_api::AccessPolicyCallerPrivilegesDTO {
    fn from(privileges: AccessPolicyCallerPrivileges) -> Self {
        wallet_api::AccessPolicyCallerPrivilegesDTO {
            can_edit: privileges.can_edit,
        }
    }
}

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

impl From<&wallet_api::GetAccessPolicyInput> for Resource {
    fn from(input: &wallet_api::GetAccessPolicyInput) -> Self {
        Resource::AccessPolicy(AccessPolicyResourceAction::Read(ResourceTypeId::Resource(
            Resource::from(input.resource.clone()).to_type(),
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
            ProposalOperationInput::EditAccessPolicy(input) => {
                Resource::AccessPolicy(AccessPolicyResourceAction::Edit(ResourceTypeId::Resource(
                    Resource::from(input.resource.clone()).to_type(),
                )))
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

impl From<FetchAccountBalancesInputRef<'_>> for Vec<Resource> {
    fn from(input: FetchAccountBalancesInputRef) -> Self {
        let account_ids = input
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

impl From<GetTransfersInputRef<'_>> for Vec<Resource> {
    fn from(input: GetTransfersInputRef) -> Self {
        let transfer_ids = input
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

impl From<wallet_api::AllowDTO> for Allow {
    fn from(dto: wallet_api::AllowDTO) -> Self {
        match dto {
            wallet_api::AllowDTO::Any => Allow::Any,
            wallet_api::AllowDTO::Authenticated => Allow::Authenticated,
            wallet_api::AllowDTO::Users(ids) => Allow::Users(
                ids.into_iter()
                    .map(|id| {
                        *HelperMapper::to_uuid(id.to_owned())
                            .expect("Invalid user id")
                            .as_bytes()
                    })
                    .collect(),
            ),
            wallet_api::AllowDTO::UserGroups(ids) => Allow::UserGroups(
                ids.into_iter()
                    .map(|id| {
                        *HelperMapper::to_uuid(id.to_owned())
                            .expect("Invalid user group id")
                            .as_bytes()
                    })
                    .collect(),
            ),
        }
    }
}

impl From<Allow> for wallet_api::AllowDTO {
    fn from(allow: Allow) -> Self {
        match allow {
            Allow::Any => wallet_api::AllowDTO::Any,
            Allow::Authenticated => wallet_api::AllowDTO::Authenticated,
            Allow::Users(ids) => wallet_api::AllowDTO::Users(
                ids.iter()
                    .map(|id| Uuid::from_bytes(*id).hyphenated().to_string())
                    .collect(),
            ),
            Allow::UserGroups(ids) => wallet_api::AllowDTO::UserGroups(
                ids.iter()
                    .map(|id| Uuid::from_bytes(*id).hyphenated().to_string())
                    .collect(),
            ),
        }
    }
}

impl From<wallet_api::AllowLevelDTO> for AllowLevel {
    fn from(dto: wallet_api::AllowLevelDTO) -> Self {
        match dto {
            wallet_api::AllowLevelDTO::Any => AllowLevel::Any,
            wallet_api::AllowLevelDTO::Authenticated => AllowLevel::Authenticated,
            wallet_api::AllowLevelDTO::Users => AllowLevel::Users,
            wallet_api::AllowLevelDTO::UserGroups => AllowLevel::UserGroups,
        }
    }
}

impl From<AllowLevel> for wallet_api::AllowLevelDTO {
    fn from(key: AllowLevel) -> Self {
        match key {
            AllowLevel::Any => wallet_api::AllowLevelDTO::Any,
            AllowLevel::Authenticated => wallet_api::AllowLevelDTO::Authenticated,
            AllowLevel::Users => wallet_api::AllowLevelDTO::Users,
            AllowLevel::UserGroups => wallet_api::AllowLevelDTO::UserGroups,
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
            wallet_api::AccessPolicyResourceActionDTO::List => AccessPolicyResourceAction::List,
            wallet_api::AccessPolicyResourceActionDTO::Read(rtype) => {
                AccessPolicyResourceAction::Read(rtype.into())
            }
            wallet_api::AccessPolicyResourceActionDTO::Edit(rtype) => {
                AccessPolicyResourceAction::Edit(rtype.into())
            }
        }
    }
}

impl From<AccessPolicyResourceAction> for wallet_api::AccessPolicyResourceActionDTO {
    fn from(action: AccessPolicyResourceAction) -> Self {
        match action {
            AccessPolicyResourceAction::List => wallet_api::AccessPolicyResourceActionDTO::List,
            AccessPolicyResourceAction::Read(id) => {
                wallet_api::AccessPolicyResourceActionDTO::Read(id.into())
            }
            AccessPolicyResourceAction::Edit(id) => {
                wallet_api::AccessPolicyResourceActionDTO::Edit(id.into())
            }
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
            wallet_api::SettingsResourceActionDTO::Read => SettingsResourceAction::Read,
            wallet_api::SettingsResourceActionDTO::ReadConfig => SettingsResourceAction::ReadConfig,
        }
    }
}

impl From<SettingsResourceAction> for wallet_api::SettingsResourceActionDTO {
    fn from(action: SettingsResourceAction) -> Self {
        match action {
            SettingsResourceAction::Read => wallet_api::SettingsResourceActionDTO::Read,
            SettingsResourceAction::ReadConfig => wallet_api::SettingsResourceActionDTO::ReadConfig,
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

impl From<ResourceType> for wallet_api::ResourceTypeDTO {
    fn from(resource_type: ResourceType) -> Self {
        match resource_type {
            ResourceType::AddressBook(action) => {
                wallet_api::ResourceTypeDTO::AddressBook(action.into())
            }
            ResourceType::ProposalPolicy(action) => {
                wallet_api::ResourceTypeDTO::ProposalPolicy(action.into())
            }
            ResourceType::UserGroup(action) => {
                wallet_api::ResourceTypeDTO::UserGroup(action.into())
            }
        }
    }
}

impl From<wallet_api::ResourceTypeDTO> for ResourceType {
    fn from(dto: wallet_api::ResourceTypeDTO) -> Self {
        match dto {
            wallet_api::ResourceTypeDTO::AddressBook(action) => {
                ResourceType::AddressBook(action.into())
            }
            wallet_api::ResourceTypeDTO::ProposalPolicy(action) => {
                ResourceType::ProposalPolicy(action.into())
            }
            wallet_api::ResourceTypeDTO::UserGroup(action) => {
                ResourceType::UserGroup(action.into())
            }
        }
    }
}

impl From<ResourceActionType> for wallet_api::ResourceActionTypeDTO {
    fn from(action_type: ResourceActionType) -> Self {
        match action_type {
            ResourceActionType::List => wallet_api::ResourceActionTypeDTO::List,
            ResourceActionType::Create => wallet_api::ResourceActionTypeDTO::Create,
            ResourceActionType::Read => wallet_api::ResourceActionTypeDTO::Read,
            ResourceActionType::Update => wallet_api::ResourceActionTypeDTO::Update,
            ResourceActionType::Delete => wallet_api::ResourceActionTypeDTO::Delete,
        }
    }
}

impl From<wallet_api::ResourceActionTypeDTO> for ResourceActionType {
    fn from(dto: wallet_api::ResourceActionTypeDTO) -> Self {
        match dto {
            wallet_api::ResourceActionTypeDTO::List => ResourceActionType::List,
            wallet_api::ResourceActionTypeDTO::Create => ResourceActionType::Create,
            wallet_api::ResourceActionTypeDTO::Read => ResourceActionType::Read,
            wallet_api::ResourceActionTypeDTO::Update => ResourceActionType::Update,
            wallet_api::ResourceActionTypeDTO::Delete => ResourceActionType::Delete,
        }
    }
}

impl From<ResourceAction> for ResourceActionType {
    fn from(action: ResourceAction) -> Self {
        match action {
            ResourceAction::List => ResourceActionType::List,
            ResourceAction::Create => ResourceActionType::Create,
            ResourceAction::Read(_) => ResourceActionType::Read,
            ResourceAction::Update(_) => ResourceActionType::Update,
            ResourceAction::Delete(_) => ResourceActionType::Delete,
        }
    }
}

impl From<ResourceTypeId> for wallet_api::ResourceTypeIdDTO {
    fn from(resource_type_id: ResourceTypeId) -> Self {
        match resource_type_id {
            ResourceTypeId::Any => wallet_api::ResourceTypeIdDTO::Any,
            ResourceTypeId::Resource(resource_type) => {
                wallet_api::ResourceTypeIdDTO::Resource(resource_type.into())
            }
        }
    }
}

impl From<wallet_api::ResourceTypeIdDTO> for ResourceTypeId {
    fn from(dto: wallet_api::ResourceTypeIdDTO) -> Self {
        match dto {
            wallet_api::ResourceTypeIdDTO::Any => ResourceTypeId::Any,
            wallet_api::ResourceTypeIdDTO::Resource(resource_type) => {
                ResourceTypeId::Resource(resource_type.into())
            }
        }
    }
}
