use ic_canister_core::api::ApiResult;
use ic_canister_macros::with_middleware;
use ic_cdk_macros::query;
use lazy_static::lazy_static;
use wallet_api::{
    GetUserGroupInput, GetUserGroupResponse, ListUserGroupsInput, ListUserGroupsResponse,
};

use crate::{
    core::middlewares::{authorize, call_context},
    mappers::HelperMapper,
    models::access_control::{ResourceSpecifier, ResourceType, UserGroupActionSpecifier},
    services::UserGroupService,
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

    #[with_middleware(
        guard = "authorize",
        context = "call_context",
        args = [ResourceSpecifier::from(&input)],
        is_async = true
    )]
    async fn get_user_group(&self, input: GetUserGroupInput) -> ApiResult<GetUserGroupResponse> {
        let user_group = self
            .user_group_service
            .get(HelperMapper::to_uuid(input.user_group_id)?.as_bytes())?
            .into();

        Ok(GetUserGroupResponse { user_group })
    }

    #[with_middleware(
        guard = "authorize",
        context = "call_context",
        args = [ResourceSpecifier::Common(ResourceType::UserGroup, UserGroupActionSpecifier::List)],
        is_async = true
    )]
    async fn list_user_groups(
        &self,
        input: ListUserGroupsInput,
    ) -> ApiResult<ListUserGroupsResponse> {
        let ctx = call_context();
        let result = self.user_group_service.list(input, Some(&ctx)).await?;

        Ok(ListUserGroupsResponse {
            user_groups: result.items.into_iter().map(Into::into).collect(),
            next_offset: result.next_offset,
            total: result.total,
        })
    }
}
