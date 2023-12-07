use crate::core::middlewares::ResourceAccess;
use crate::models::access_control::{AccessModifier, Resource};
use crate::{
    core::{
        middlewares::{authorize, call_context},
        CallContext,
    },
    mappers::HelperMapper,
    services::UserService,
};
use ic_canister_core::api::ApiResult;
use ic_canister_macros::with_middleware;
use ic_cdk_macros::{query, update};
use lazy_static::lazy_static;
use wallet_api::{
    ConfirmUserIdentityInput, ConfirmUserIdentityResponse, GetUserInput, GetUserResponse,
};

// Canister entrypoints for the controller.
#[update(name = "confirm_user_identity")]
async fn confirm_user_identity(
    input: ConfirmUserIdentityInput,
) -> ApiResult<ConfirmUserIdentityResponse> {
    CONTROLLER.confirm_user_identity(input).await
}

#[query(name = "get_user")]
async fn get_user(input: GetUserInput) -> ApiResult<GetUserResponse> {
    CONTROLLER.get_user(input).await
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

    /// No authorization required since the user will be calling this with a new identity that is not
    /// yet confirmed with a user.
    async fn confirm_user_identity(
        &self,
        input: ConfirmUserIdentityInput,
    ) -> ApiResult<ConfirmUserIdentityResponse> {
        let user = self
            .user_service
            .confirm_user_identity(input, &call_context())
            .await?
            .into();

        Ok(ConfirmUserIdentityResponse { user })
    }

    #[with_middleware(
        guard = "authorize",
        context = "call_context",
        args = [
            ResourceAccess(Resource::User, AccessModifier::Default)
        ],
        is_async = true
    )]
    async fn get_user(&self, input: GetUserInput) -> ApiResult<GetUserResponse> {
        let ctx = call_context();
        let user = match input.user_id {
            Some(user_id) => self
                .user_service
                .get_user(HelperMapper::to_uuid(user_id)?.as_bytes(), &ctx)?
                .into(),
            _ => self
                .user_service
                .get_user_by_identity(&CallContext::get().caller(), &ctx)?
                .into(),
        };

        Ok(GetUserResponse { user })
    }
}
