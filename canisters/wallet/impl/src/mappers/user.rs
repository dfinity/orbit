use super::HelperMapper;
use crate::{
    core::ic_cdk::api::time,
    errors::UserError,
    models::{
        access_policy::{
            AccessPolicyResourceAction, AccountResourceAction, ChangeCanisterResourceAction,
            ProposalResourceAction, Resource, ResourceAction, SettingsResourceAction,
            UserResourceAction,
        },
        AddUserOperationInput, DisplayUser, EditUserOperationInput, User, UserCallerPrivileges,
    },
    repositories::USER_GROUP_REPOSITORY,
};
use ic_canister_core::{
    repository::Repository,
    types::UUID,
    utils::{rfc3339_to_timestamp, timestamp_to_rfc3339},
};
use uuid::Uuid;
use wallet_api::{BasicUserDTO, DisplayUserDTO, UserDTO, UserPrivilege};

#[derive(Default, Clone, Debug)]
pub struct UserMapper {}

impl UserMapper {
    pub fn from_create_input(new_user_id: UUID, input: AddUserOperationInput) -> User {
        User {
            id: new_user_id,
            identities: input.identities,
            groups: input.groups,
            name: input.name,
            status: input.status,
            last_modification_timestamp: time(),
        }
    }
}

impl From<User> for UserDTO {
    fn from(user: User) -> Self {
        UserDTO {
            id: Uuid::from_bytes(user.id).hyphenated().to_string(),
            identities: user.identities,
            name: user.name,
            status: user.status.into(),
            groups: user
                .groups
                .iter()
                .map(|group| {
                    USER_GROUP_REPOSITORY
                        .get(group)
                        .expect("Invalid group")
                        .into()
                })
                .collect(),
            last_modification_timestamp: timestamp_to_rfc3339(&user.last_modification_timestamp),
        }
    }
}

impl From<User> for BasicUserDTO {
    fn from(user: User) -> Self {
        BasicUserDTO {
            id: Uuid::from_bytes(user.id).hyphenated().to_string(),
            name: user.name.unwrap_or("".to_string()),
            status: user.status.into(),
        }
    }
}

impl From<DisplayUser> for DisplayUserDTO {
    fn from(user: DisplayUser) -> Self {
        DisplayUserDTO {
            id: Uuid::from_bytes(user.id).hyphenated().to_string(),
            name: user.name,
        }
    }
}

impl From<UserDTO> for User {
    fn from(user: UserDTO) -> Self {
        Self {
            id: *Uuid::parse_str(&user.id).expect("Invalid UUID").as_bytes(),
            identities: user.identities,
            name: user.name,
            status: user.status.into(),
            groups: user
                .groups
                .iter()
                .map(|group| {
                    *HelperMapper::to_uuid(group.id.to_owned())
                        .expect("Invalid UUID")
                        .as_bytes()
                })
                .collect(),
            last_modification_timestamp: rfc3339_to_timestamp(
                user.last_modification_timestamp.as_str(),
            ),
        }
    }
}

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
            UserPrivilege::Capabilities => Resource::Settings(SettingsResourceAction::Capabilities),
            UserPrivilege::SystemInfo => Resource::Settings(SettingsResourceAction::SystemInfo),
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

impl User {
    pub fn update_with(&mut self, input: EditUserOperationInput) -> Result<(), UserError> {
        if let Some(new_identities) = &input.identities {
            self.identities = new_identities.to_owned();
        }

        if let Some(new_groups) = input.groups {
            self.groups = new_groups;
        }

        if let Some(new_name) = input.name {
            self.name = Some(new_name);
        }

        if let Some(new_status) = input.status {
            self.status = new_status;
        }

        Ok(())
    }
}

impl From<UserCallerPrivileges> for wallet_api::UserCallerPrivilegesDTO {
    fn from(privileges: UserCallerPrivileges) -> Self {
        wallet_api::UserCallerPrivilegesDTO {
            id: Uuid::from_bytes(privileges.id).hyphenated().to_string(),
            can_edit: privileges.can_edit,
        }
    }
}
