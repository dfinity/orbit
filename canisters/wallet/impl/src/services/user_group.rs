use crate::core::generate_uuid_v4;
use crate::core::ic_cdk::api::time;
use crate::errors::UserGroupError;
use crate::models::{
    AddUserGroupOperationInput, EditUserGroupOperationInput, RemoveUserGroupOperationInput,
    UserGroup,
};
use crate::repositories::UserGroupRepository;
use ic_canister_core::api::ServiceResult;
use ic_canister_core::model::ModelValidator;
use ic_canister_core::repository::Repository;
use ic_canister_core::types::UUID;
use uuid::Uuid;

#[derive(Default, Debug)]
pub struct UserGroupService {
    user_group_repository: UserGroupRepository,
}

impl UserGroupService {
    pub fn get(&self, user_group_id: &UUID) -> ServiceResult<UserGroup> {
        let user_group = self
            .user_group_repository
            .get(&UserGroup::key(*user_group_id))
            .ok_or(UserGroupError::NotFound {
                id: Uuid::from_bytes(*user_group_id).hyphenated().to_string(),
            })?;

        Ok(user_group)
    }

    pub async fn create(&self, input: AddUserGroupOperationInput) -> ServiceResult<UserGroup> {
        let user_group_id = generate_uuid_v4().await;
        let user_group = UserGroup {
            id: *user_group_id.as_bytes(),
            name: input.name.to_string(),
            last_modification_timestamp: time(),
        };

        user_group.validate()?;

        self.user_group_repository
            .insert(user_group.to_key(), user_group.clone());

        Ok(user_group)
    }

    pub async fn edit(&self, input: EditUserGroupOperationInput) -> ServiceResult<UserGroup> {
        let mut user_group = self.get(&input.user_group_id)?;

        user_group.name = input.name.to_string();
        user_group.last_modification_timestamp = time();

        user_group.validate()?;

        self.user_group_repository
            .insert(user_group.to_key(), user_group.clone());

        Ok(user_group)
    }

    pub async fn remove(&self, input: RemoveUserGroupOperationInput) -> ServiceResult<()> {
        let user_group = self.get(&input.user_group_id)?;

        self.user_group_repository.remove(&user_group.to_key());

        Ok(())
    }
}
