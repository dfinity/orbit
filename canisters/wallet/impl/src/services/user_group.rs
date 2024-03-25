use crate::core::authorization::Authorization;
use crate::core::ic_cdk::api::time;
use crate::core::utils::{
    paginated_items, retain_accessible_resources, PaginatedData, PaginatedItemsArgs,
};
use crate::core::{generate_uuid_v4, CallContext};
use crate::errors::UserGroupError;
use crate::models::resource::{Resource, ResourceAction, ResourceId};
use crate::models::{
    AddUserGroupOperationInput, EditUserGroupOperationInput, UserGroup, UserGroupCallerPrivileges,
};
use crate::repositories::{UseGroupWhereClause, UserGroupRepository};
use ic_canister_core::api::ServiceResult;
use ic_canister_core::model::ModelValidator;
use ic_canister_core::repository::Repository;
use ic_canister_core::types::UUID;
use lazy_static::lazy_static;
use std::sync::Arc;
use uuid::Uuid;
use wallet_api::ListUserGroupsInput;

lazy_static! {
    pub static ref USER_GROUP_SERVICE: Arc<UserGroupService> =
        Arc::new(UserGroupService::default());
}

#[derive(Default, Debug)]
pub struct UserGroupService {
    user_group_repository: UserGroupRepository,
}

impl UserGroupService {
    const DEFAULT_USER_GROUP_LIST_LIMIT: u16 = 100;
    const MAX_USER_GROUP_LIST_LIMIT: u16 = 1000;

    pub fn get(&self, user_group_id: &UUID) -> ServiceResult<UserGroup> {
        let user_group =
            self.user_group_repository
                .get(user_group_id)
                .ok_or(UserGroupError::NotFound {
                    id: Uuid::from_bytes(*user_group_id).hyphenated().to_string(),
                })?;

        Ok(user_group)
    }

    pub async fn get_caller_privileges_for_user_group(
        &self,
        user_group_id: &UUID,
        ctx: &CallContext,
    ) -> ServiceResult<UserGroupCallerPrivileges> {
        Ok(UserGroupCallerPrivileges {
            id: *user_group_id,
            can_edit: Authorization::is_allowed(
                ctx,
                &Resource::UserGroup(ResourceAction::Update(ResourceId::Id(*user_group_id))),
            ),
            can_delete: Authorization::is_allowed(
                ctx,
                &Resource::UserGroup(ResourceAction::Delete(ResourceId::Id(*user_group_id))),
            ),
        })
    }

    pub async fn list(
        &self,
        input: ListUserGroupsInput,
        ctx: Option<&CallContext>,
    ) -> ServiceResult<PaginatedData<UserGroup>> {
        let mut user_groups = self.user_group_repository.find_where(UseGroupWhereClause {
            search_term: input.search_term.to_owned(),
        });

        // filter out user groups that the caller does not have access to read
        if let Some(ctx) = ctx {
            retain_accessible_resources(ctx, &mut user_groups, |user_group| {
                Resource::UserGroup(ResourceAction::Read(ResourceId::Id(user_group.id)))
            });
        }

        let result = paginated_items(PaginatedItemsArgs {
            offset: input.paginate.to_owned().and_then(|p| p.offset),
            limit: input.paginate.and_then(|p| p.limit),
            default_limit: Some(Self::DEFAULT_USER_GROUP_LIST_LIMIT),
            max_limit: Some(Self::MAX_USER_GROUP_LIST_LIMIT),
            items: &user_groups,
        })?;

        Ok(result)
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
            .insert(user_group.id, user_group.clone());

        Ok(user_group)
    }

    pub async fn edit(&self, input: EditUserGroupOperationInput) -> ServiceResult<UserGroup> {
        let mut user_group = self.get(&input.user_group_id)?;

        user_group.name = input.name.to_string();
        user_group.last_modification_timestamp = time();

        user_group.validate()?;

        self.user_group_repository
            .insert(user_group.id, user_group.clone());

        Ok(user_group)
    }

    pub async fn remove(&self, id: &UUID) -> ServiceResult<()> {
        let user_group = self.get(id)?;

        self.user_group_repository.remove(&user_group.id);

        Ok(())
    }
}
