use super::HelperMapper;
use crate::models::{
    AddUserGroupOperation, AddUserGroupOperationInput, EditUserGroupOperation,
    EditUserGroupOperationInput, RemoveUserGroupOperation, RemoveUserGroupOperationInput,
    UserGroup,
};
use uuid::Uuid;
use wallet_api::UserGroupDTO;

#[derive(Default, Clone, Debug)]
pub struct UserGroupMapper;

impl From<wallet_api::AddUserGroupOperationInput> for AddUserGroupOperationInput {
    fn from(input: wallet_api::AddUserGroupOperationInput) -> Self {
        Self { name: input.name }
    }
}

impl From<AddUserGroupOperationInput> for wallet_api::AddUserGroupOperationInput {
    fn from(input: AddUserGroupOperationInput) -> Self {
        Self { name: input.name }
    }
}

impl From<wallet_api::EditUserGroupOperationInput> for EditUserGroupOperationInput {
    fn from(input: wallet_api::EditUserGroupOperationInput) -> Self {
        Self {
            user_group_id: *HelperMapper::to_uuid(input.user_group_id)
                .expect("Invalid UUID")
                .as_bytes(),
            name: input.name,
        }
    }
}

impl From<EditUserGroupOperationInput> for wallet_api::EditUserGroupOperationInput {
    fn from(input: EditUserGroupOperationInput) -> Self {
        Self {
            user_group_id: Uuid::from_bytes(input.user_group_id)
                .hyphenated()
                .to_string(),
            name: input.name,
        }
    }
}

impl From<wallet_api::RemoveUserGroupOperationInput> for RemoveUserGroupOperationInput {
    fn from(input: wallet_api::RemoveUserGroupOperationInput) -> Self {
        Self {
            user_group_id: *HelperMapper::to_uuid(input.user_group_id)
                .expect("Invalid UUID")
                .as_bytes(),
        }
    }
}

impl From<RemoveUserGroupOperationInput> for wallet_api::RemoveUserGroupOperationInput {
    fn from(input: RemoveUserGroupOperationInput) -> Self {
        Self {
            user_group_id: Uuid::from_bytes(input.user_group_id)
                .hyphenated()
                .to_string(),
        }
    }
}

impl From<UserGroup> for wallet_api::UserGroupDTO {
    fn from(user_group: UserGroup) -> Self {
        Self {
            id: Uuid::from_bytes(user_group.id).hyphenated().to_string(),
            name: user_group.name,
        }
    }
}

impl AddUserGroupOperation {
    pub fn to_dto(self, user_group: Option<UserGroup>) -> wallet_api::AddUserGroupOperationDTO {
        wallet_api::AddUserGroupOperationDTO {
            user_group: user_group.map(|user_group| user_group.into()),
            input: self.input.into(),
        }
    }
}

impl From<EditUserGroupOperation> for wallet_api::EditUserGroupOperationDTO {
    fn from(operation: EditUserGroupOperation) -> Self {
        Self {
            input: operation.input.into(),
        }
    }
}

impl From<RemoveUserGroupOperation> for wallet_api::RemoveUserGroupOperationDTO {
    fn from(operation: RemoveUserGroupOperation) -> Self {
        Self {
            input: operation.input.into(),
        }
    }
}

impl From<wallet_api::AddUserGroupOperationInput> for AddUserGroupOperation {
    fn from(input: wallet_api::AddUserGroupOperationInput) -> Self {
        Self {
            user_group_id: None,
            input: input.into(),
        }
    }
}

impl From<wallet_api::EditUserGroupOperationInput> for EditUserGroupOperation {
    fn from(input: wallet_api::EditUserGroupOperationInput) -> Self {
        Self {
            input: input.into(),
        }
    }
}

impl From<wallet_api::RemoveUserGroupOperationInput> for RemoveUserGroupOperation {
    fn from(input: wallet_api::RemoveUserGroupOperationInput) -> Self {
        Self {
            input: input.into(),
        }
    }
}

impl UserGroupMapper {
    pub fn to_dto(user_group: UserGroup) -> UserGroupDTO {
        let id = Uuid::from_slice(&user_group.id)
            .unwrap()
            .hyphenated()
            .to_string();

        let name = user_group.name;

        UserGroupDTO { id, name }
    }
}

impl UserGroup {
    pub fn to_dto(&self) -> UserGroupDTO {
        UserGroupMapper::to_dto(self.clone())
    }
}
