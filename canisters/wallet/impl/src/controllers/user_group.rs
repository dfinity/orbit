use ic_canister_core::api::ApiResult;
use ic_canister_macros::with_middleware;
use ic_cdk_macros::query;
use lazy_static::lazy_static;
use wallet_api::{GetUserGroupInput, GetUserGroupResponse, ListUserGroupResponse};

use crate::{
    core::{
        middlewares::{authorize, call_context},
        PERMISSION_READ_USER_GROUP,
    },
    mappers::HelperMapper,
    services::UserGroupService,
};

#[query(name = "get_user_group")]
async fn get_user_group(input: GetUserGroupInput) -> ApiResult<GetUserGroupResponse> {
    CONTROLLER.get_user_group(input).await
}

#[query(name = "list_user_groups")]
async fn list_user_groups() -> ApiResult<ListUserGroupResponse> {
    CONTROLLER.list_user_groups().await
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

    #[with_middleware(guard = "authorize", context = "call_context", args = [PERMISSION_READ_USER_GROUP])]
    async fn get_user_group(&self, input: GetUserGroupInput) -> ApiResult<GetUserGroupResponse> {
        let user_group = self
            .user_group_service
            .get(HelperMapper::to_uuid(input.user_group_id)?.as_bytes())?
            .to_dto();

        Ok(GetUserGroupResponse { user_group })
    }

    #[with_middleware(guard = "authorize", context = "call_context", args = [PERMISSION_READ_USER_GROUP])]
    async fn list_user_groups(&self) -> ApiResult<ListUserGroupResponse> {
        let user_groups = self
            .user_group_service
            .list()?
            .iter()
            .map(|g| g.to_dto())
            .collect();

        Ok(ListUserGroupResponse { user_groups })
    }
}
