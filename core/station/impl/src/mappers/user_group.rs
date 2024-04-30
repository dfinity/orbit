use super::HelperMapper;
use crate::models::{
    AddUserGroupOperation, AddUserGroupOperationInput, EditUserGroupOperation,
    EditUserGroupOperationInput, RemoveUserGroupOperation, RemoveUserGroupOperationInput,
    UserGroup, UserGroupCallerPrivileges,
};
use uuid::Uuid;

impl From<station_api::AddUserGroupOperationInput> for AddUserGroupOperationInput {
    fn from(input: station_api::AddUserGroupOperationInput) -> Self {
        Self { name: input.name }
    }
}

impl From<AddUserGroupOperationInput> for station_api::AddUserGroupOperationInput {
    fn from(input: AddUserGroupOperationInput) -> Self {
        Self { name: input.name }
    }
}

impl From<station_api::EditUserGroupOperationInput> for EditUserGroupOperationInput {
    fn from(input: station_api::EditUserGroupOperationInput) -> Self {
        Self {
            user_group_id: *HelperMapper::to_uuid(input.user_group_id)
                .expect("Invalid UUID")
                .as_bytes(),
            name: input.name,
        }
    }
}

impl From<EditUserGroupOperationInput> for station_api::EditUserGroupOperationInput {
    fn from(input: EditUserGroupOperationInput) -> Self {
        Self {
            user_group_id: Uuid::from_bytes(input.user_group_id)
                .hyphenated()
                .to_string(),
            name: input.name,
        }
    }
}

impl From<station_api::RemoveUserGroupOperationInput> for RemoveUserGroupOperationInput {
    fn from(input: station_api::RemoveUserGroupOperationInput) -> Self {
        Self {
            user_group_id: *HelperMapper::to_uuid(input.user_group_id)
                .expect("Invalid UUID")
                .as_bytes(),
        }
    }
}

impl From<RemoveUserGroupOperationInput> for station_api::RemoveUserGroupOperationInput {
    fn from(input: RemoveUserGroupOperationInput) -> Self {
        Self {
            user_group_id: Uuid::from_bytes(input.user_group_id)
                .hyphenated()
                .to_string(),
        }
    }
}

impl From<UserGroup> for station_api::UserGroupDTO {
    fn from(user_group: UserGroup) -> Self {
        Self {
            id: Uuid::from_bytes(user_group.id).hyphenated().to_string(),
            name: user_group.name,
        }
    }
}

impl AddUserGroupOperation {
    pub fn to_dto(self, user_group: Option<UserGroup>) -> station_api::AddUserGroupOperationDTO {
        station_api::AddUserGroupOperationDTO {
            user_group: user_group.map(|user_group| user_group.into()),
            input: self.input.into(),
        }
    }
}

impl From<EditUserGroupOperation> for station_api::EditUserGroupOperationDTO {
    fn from(operation: EditUserGroupOperation) -> Self {
        Self {
            input: operation.input.into(),
        }
    }
}

impl From<RemoveUserGroupOperation> for station_api::RemoveUserGroupOperationDTO {
    fn from(operation: RemoveUserGroupOperation) -> Self {
        Self {
            input: operation.input.into(),
        }
    }
}

impl From<station_api::AddUserGroupOperationInput> for AddUserGroupOperation {
    fn from(input: station_api::AddUserGroupOperationInput) -> Self {
        Self {
            user_group_id: None,
            input: input.into(),
        }
    }
}

impl From<station_api::EditUserGroupOperationInput> for EditUserGroupOperation {
    fn from(input: station_api::EditUserGroupOperationInput) -> Self {
        Self {
            input: input.into(),
        }
    }
}

impl From<station_api::RemoveUserGroupOperationInput> for RemoveUserGroupOperation {
    fn from(input: station_api::RemoveUserGroupOperationInput) -> Self {
        Self {
            input: input.into(),
        }
    }
}

impl From<UserGroupCallerPrivileges> for station_api::UserGroupCallerPrivilegesDTO {
    fn from(privileges: UserGroupCallerPrivileges) -> Self {
        Self {
            id: Uuid::from_bytes(privileges.id).hyphenated().to_string(),
            can_edit: privileges.can_edit,
            can_delete: privileges.can_delete,
        }
    }
}
