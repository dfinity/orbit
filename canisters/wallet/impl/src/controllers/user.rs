use crate::core::{PERMISSION_READ_USER, PERMISSION_REGISTER_USER, PERMISSION_WRITE_USER};
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
    ConfirmUserIdentityInput, ConfirmUserIdentityResponse, EditUserInput, EditUserResponse,
    GetUserInput, GetUserResponse, RegisterUserInput, RegisterUserResponse,
};

// Canister entrypoints for the controller.
#[update(name = "register_user")]
async fn register_user(input: RegisterUserInput) -> ApiResult<RegisterUserResponse> {
    CONTROLLER.register_user(input).await
}

#[update(name = "confirm_user_identity")]
async fn confirm_user_identity(
    input: ConfirmUserIdentityInput,
) -> ApiResult<ConfirmUserIdentityResponse> {
    CONTROLLER.confirm_user_identity(input).await
}

#[update(name = "edit_user")]
async fn edit_user(input: EditUserInput) -> ApiResult<EditUserResponse> {
    CONTROLLER.edit_user(input).await
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

    #[with_middleware(guard = "authorize", context = "call_context", args = [PERMISSION_REGISTER_USER])]
    async fn register_user(&self, input: RegisterUserInput) -> ApiResult<RegisterUserResponse> {
        let user = self
            .user_service
            .register_user(input, Vec::new(), &call_context())
            .await?
            .to_dto();

        Ok(RegisterUserResponse { user })
    }

    #[with_middleware(guard = "authorize", context = "call_context", args = [PERMISSION_REGISTER_USER])]
    async fn confirm_user_identity(
        &self,
        input: ConfirmUserIdentityInput,
    ) -> ApiResult<ConfirmUserIdentityResponse> {
        let user = self
            .user_service
            .confirm_user_identity(input, &call_context())
            .await?
            .to_dto();

        Ok(ConfirmUserIdentityResponse { user })
    }

    #[with_middleware(guard = "authorize", context = "call_context", args = [PERMISSION_WRITE_USER])]
    async fn edit_user(&self, input: EditUserInput) -> ApiResult<EditUserResponse> {
        let user = self
            .user_service
            .edit_user(input, &call_context())
            .await?
            .to_dto();

        Ok(EditUserResponse { user })
    }

    #[with_middleware(guard = "authorize", context = "call_context", args = [PERMISSION_READ_USER])]
    async fn get_user(&self, input: GetUserInput) -> ApiResult<GetUserResponse> {
        let ctx = call_context();
        let user = match input.user_id {
            Some(user_id) => self
                .user_service
                .get_user(HelperMapper::to_uuid(user_id)?.as_bytes(), &ctx)?
                .to_dto(),
            _ => self
                .user_service
                .get_user_by_identity(&CallContext::get().caller(), &ctx)?
                .to_dto(),
        };

        Ok(GetUserResponse { user })
    }
}
