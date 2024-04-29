use super::HelperMapper;
use crate::{
    core::ic_cdk::next_time,
    errors::UserError,
    models::{
        AddUserOperationInput, DisplayUser, EditUserOperationInput, User, UserCallerPrivileges,
    },
    repositories::USER_GROUP_REPOSITORY,
};
use orbit_essentials::{
    repository::Repository,
    types::UUID,
    utils::{rfc3339_to_timestamp, timestamp_to_rfc3339},
};
use station_api::{BasicUserDTO, DisplayUserDTO, UserDTO};
use uuid::Uuid;

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
            last_modification_timestamp: next_time(),
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
                .filter_map(|group| USER_GROUP_REPOSITORY.get(group))
                .map(Into::into)
                .collect(),
            last_modification_timestamp: timestamp_to_rfc3339(&user.last_modification_timestamp),
        }
    }
}

impl From<User> for BasicUserDTO {
    fn from(user: User) -> Self {
        BasicUserDTO {
            id: Uuid::from_bytes(user.id).hyphenated().to_string(),
            name: user.name,
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

impl User {
    pub fn update_with(&mut self, input: EditUserOperationInput) -> Result<(), UserError> {
        if let Some(new_identities) = &input.identities {
            self.identities = new_identities.to_owned();
        }

        if let Some(new_groups) = input.groups {
            self.groups = new_groups;
        }

        if let Some(new_name) = input.name {
            self.name = new_name;
        }

        if let Some(new_status) = input.status {
            self.status = new_status;
        }

        Ok(())
    }
}

impl From<UserCallerPrivileges> for station_api::UserCallerPrivilegesDTO {
    fn from(privileges: UserCallerPrivileges) -> Self {
        station_api::UserCallerPrivilegesDTO {
            id: Uuid::from_bytes(privileges.id).hyphenated().to_string(),
            can_edit: privileges.can_edit,
        }
    }
}
