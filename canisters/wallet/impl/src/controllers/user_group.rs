use crate::{
    core::middlewares::{authorize, call_context},
    mappers::HelperMapper,
    models::access_policy::{Resource, ResourceAction},
    services::UserGroupService,
};
use ic_canister_core::api::ApiResult;
use ic_canister_macros::with_middleware;
use ic_cdk_macros::query;
use lazy_static::lazy_static;
use wallet_api::{
    GetUserGroupInput, GetUserGroupResponse, ListUserGroupsInput, ListUserGroupsResponse,
    UserGroupCallerPrivilegesDTO,
};

#[query(name = "get_user_group")]
async fn get_user_group(input: GetUserGroupInput) -> ApiResult<GetUserGroupResponse> {
    CONTROLLER.get_user_group(input).await
}

#[query(name = "list_user_groups")]
async fn list_user_groups(input: ListUserGroupsInput) -> ApiResult<ListUserGroupsResponse> {
    CONTROLLER.list_user_groups(input).await
}

lazy_static! {
    static ref CONTROLLER: UserGroupController =
        UserGroupController::new(UserGroupService::default());
}

#[derive(Debug)]
pub struct UserGroupController {
    user_group_service: UserGroupService,
}

impl UserGroupController {
    pub fn new(user_group_service: UserGroupService) -> Self {
        Self { user_group_service }
    }

    #[with_middleware(guard = "authorize", context = "call_context", args = [Resource::from(&input)])]
    async fn get_user_group(&self, input: GetUserGroupInput) -> ApiResult<GetUserGroupResponse> {
        let ctx = call_context();
        let user_group = self
            .user_group_service
            .get(HelperMapper::to_uuid(input.user_group_id)?.as_bytes())?;
        let privileges = self
            .user_group_service
            .get_caller_privileges_for_user_group(&user_group.id, &ctx)
            .await?;

        Ok(GetUserGroupResponse {
            user_group: user_group.into(),
            privileges: privileges.into(),
        })
    }

    #[with_middleware(
        guard = "authorize",
        context = "call_context",
        args = [Resource::UserGroup(ResourceAction::List)]
    )]
    async fn list_user_groups(
        &self,
        input: ListUserGroupsInput,
    ) -> ApiResult<ListUserGroupsResponse> {
        let ctx = call_context();
        let result = self.user_group_service.list(input, Some(&ctx)).await?;
        let mut privileges = Vec::new();

        for user_group in &result.items {
            let user_group_privileges = self
                .user_group_service
                .get_caller_privileges_for_user_group(&user_group.id, &ctx)
                .await?;

            privileges.push(UserGroupCallerPrivilegesDTO::from(user_group_privileges));
        }

        Ok(ListUserGroupsResponse {
            user_groups: result.items.into_iter().map(Into::into).collect(),
            next_offset: result.next_offset,
            total: result.total,
            privileges,
        })
    }
}
