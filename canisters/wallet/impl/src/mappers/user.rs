use crate::{
    core::ic_cdk::api::time,
    errors::UserError,
    models::{
        access_control::{CommonActionSpecifier, ResourceSpecifier, ResourceType},
        AddUserOperationInput, EditUserOperationInput, User,
    },
    repositories::USER_GROUP_REPOSITORY,
};
use candid::Principal;
use ic_canister_core::{
    repository::Repository,
    types::UUID,
    utils::{rfc3339_to_timestamp, timestamp_to_rfc3339},
};
use std::collections::HashSet;
use uuid::Uuid;
use wallet_api::{UserDTO, UserPrivilege};

use super::HelperMapper;

#[derive(Default, Clone, Debug)]
pub struct UserMapper {}

impl UserMapper {
    pub fn from_create_input(new_user_id: UUID, input: AddUserOperationInput) -> User {
        User {
            id: new_user_id,
            identities: input.identities,
            unconfirmed_identities: input.unconfirmed_identities,
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
            unconfirmed_identities: user.unconfirmed_identities,
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

impl From<UserDTO> for User {
    fn from(user: UserDTO) -> Self {
        Self {
            id: *Uuid::parse_str(&user.id).expect("Invalid UUID").as_bytes(),
            identities: user.identities,
            unconfirmed_identities: user.unconfirmed_identities,
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

pub const USER_PRIVILEGES: [UserPrivilege; 8] = [
    UserPrivilege::ListUsers,
    UserPrivilege::AddUser,
    UserPrivilege::ListAccounts,
    UserPrivilege::AddAccount,
    UserPrivilege::ListAccessPolicies,
    UserPrivilege::AddAccessPolicy,
    UserPrivilege::ListProposalPolicies,
    UserPrivilege::AddProposalPolicy,
];

impl From<UserPrivilege> for ResourceSpecifier {
    fn from(privilege: UserPrivilege) -> Self {
        match privilege {
            UserPrivilege::ListUsers => {
                ResourceSpecifier::Common(ResourceType::User, CommonActionSpecifier::List)
            }
            UserPrivilege::AddUser => {
                ResourceSpecifier::Common(ResourceType::User, CommonActionSpecifier::Create)
            }
            UserPrivilege::ListAccounts => {
                ResourceSpecifier::Common(ResourceType::Account, CommonActionSpecifier::List)
            }
            UserPrivilege::AddAccount => {
                ResourceSpecifier::Common(ResourceType::Account, CommonActionSpecifier::Create)
            }
            UserPrivilege::ListAccessPolicies => {
                ResourceSpecifier::Common(ResourceType::AccessPolicy, CommonActionSpecifier::List)
            }
            UserPrivilege::AddAccessPolicy => {
                ResourceSpecifier::Common(ResourceType::AccessPolicy, CommonActionSpecifier::Create)
            }
            UserPrivilege::ListProposalPolicies => {
                ResourceSpecifier::Common(ResourceType::ProposalPolicy, CommonActionSpecifier::List)
            }
            UserPrivilege::AddProposalPolicy => ResourceSpecifier::Common(
                ResourceType::ProposalPolicy,
                CommonActionSpecifier::Create,
            ),
            UserPrivilege::ListUserGroups => {
                ResourceSpecifier::Common(ResourceType::UserGroup, CommonActionSpecifier::List)
            }
            UserPrivilege::AddUserGroup => {
                ResourceSpecifier::Common(ResourceType::UserGroup, CommonActionSpecifier::Create)
            }
        }
    }
}

impl User {
    pub fn update_with(&mut self, input: EditUserOperationInput) -> Result<(), UserError> {
        if let Some(new_identities) = &input.identities {
            let mut unconfirmed_identities: HashSet<Principal> = self
                .unconfirmed_identities
                .iter()
                .filter(|i| new_identities.contains(i))
                .copied()
                .collect();

            for identity in new_identities {
                unconfirmed_identities.retain(|i| *i != *identity);
            }

            self.identities = new_identities.to_owned();
            self.unconfirmed_identities = unconfirmed_identities.into_iter().collect();
        }

        if let Some(new_groups) = input.groups {
            self.groups = new_groups;
        }

        if let Some(new_name) = input.name {
            self.name = Some(new_name);
        }

        Ok(())
    }
}
