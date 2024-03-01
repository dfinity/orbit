use crate::{
    core::middlewares::{authorize, call_context},
    mappers::HelperMapper,
    models::access_control::{ResourceSpecifier, ResourceType, UserActionSpecifier},
    services::UserService,
};
use ic_canister_core::api::ApiResult;
use ic_canister_macros::with_middleware;
use ic_cdk_macros::query;
use lazy_static::lazy_static;
use wallet_api::{
    GetUserInput, GetUserResponse, ListUsersInput, ListUsersResponse, MeResponse,
    UserCallerPrivilegesDTO,
};

// Canister entrypoints for the controller.
#[query(name = "get_user")]
async fn get_user(input: GetUserInput) -> ApiResult<GetUserResponse> {
    CONTROLLER.get_user(input).await
}

#[query(name = "list_users")]
async fn list_users(input: ListUsersInput) -> ApiResult<ListUsersResponse> {
    CONTROLLER.list_users(input).await
}

#[query(name = "me")]
async fn me() -> ApiResult<MeResponse> {
    CONTROLLER.me().await
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: UserController = UserController::new(UserService::default());
}

#[derive(Debug)]
pub struct UserController {
    user_service: UserService,
}

impl UserController {
    fn new(user_service: UserService) -> Self {
        Self { user_service }
    }

    #[with_middleware(
        guard = "authorize",
        context = "call_context",
        args = [ResourceSpecifier::from(&input)],
        is_async = true
    )]
    async fn get_user(&self, input: GetUserInput) -> ApiResult<GetUserResponse> {
        let ctx = call_context();
        let user = self
            .user_service
            .get_user(HelperMapper::to_uuid(input.user_id)?.as_bytes())?;
        let privileges = self
            .user_service
            .get_caller_privileges_for_user(&user.id, &ctx)
            .await?;

        Ok(GetUserResponse {
            user: user.into(),
            privileges: privileges.into(),
        })
    }

    #[with_middleware(
        guard = "authorize",
        context = "call_context",
        args = [ResourceSpecifier::Common(ResourceType::User, UserActionSpecifier::List)],
        is_async = true
    )]
    async fn list_users(&self, input: ListUsersInput) -> ApiResult<ListUsersResponse> {
        let ctx = call_context();
        let list = self.user_service.list_users(input, Some(&ctx)).await?;
        let mut privileges = Vec::new();

        for user in &list.items {
            let user_privileges = self
                .user_service
                .get_caller_privileges_for_user(&user.id, &ctx)
                .await?;

            privileges.push(UserCallerPrivilegesDTO::from(user_privileges));
        }

        Ok(ListUsersResponse {
            users: list.items.into_iter().map(Into::into).collect(),
            next_offset: list.next_offset,
            total: list.total,
            privileges,
        })
    }

    /// Returns the user that is calling this endpoint.
    ///
    /// No authorization required since this endpoint only exposes the user associated with the caller identity.
    /// If the caller does not have a user associated with the identity, an error will be returned.
    async fn me(&self) -> ApiResult<MeResponse> {
        let ctx = call_context();
        let user = self.user_service.get_user_by_identity(&ctx.caller())?;

        let privileges = self
            .user_service
            .get_caller_privileges(&ctx)
            .await?;

        Ok(MeResponse {
            me: user.into(),
            privileges,
        })
    }
}
