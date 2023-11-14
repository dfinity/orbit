//! User services.
use crate::core::middlewares::{call_context, log_call, log_call_result};
use crate::{
    core::CallContext,
    mappers::HelperMapper,
    services::UserService,
    transport::{
        AssociateIdentityWithUserInput, AssociateIdentityWithUserResponse, DeleteUserResponse,
        GetUserResponse, ManageUserInput, ManageUserResponse, RegisterUserInput,
        RegisterUserResponse, UserDTO,
    },
};
use ic_canister_core::api::ApiResult;
use ic_canister_macros::with_middleware;
use ic_cdk_macros::{query, update};
use lazy_static::lazy_static;

// Canister entrypoints for the controller.

#[query(name = "get_user")]
async fn get_user() -> ApiResult<GetUserResponse> {
    CONTROLLER.get_user().await
}

#[update(name = "register_user")]
async fn register_user(input: RegisterUserInput) -> ApiResult<RegisterUserResponse> {
    CONTROLLER.register_user(input).await
}

#[update(name = "manage_user")]
async fn manage_user(input: ManageUserInput) -> ApiResult<ManageUserResponse> {
    CONTROLLER.manage_user(input).await
}

#[update(name = "associate_identity_with_user")]
async fn associate_identity_with_user(
    input: AssociateIdentityWithUserInput,
) -> ApiResult<AssociateIdentityWithUserResponse> {
    CONTROLLER.associate_identity_with_user(input).await
}

#[update(name = "delete_user")]
async fn delete_user() -> ApiResult<DeleteUserResponse> {
    CONTROLLER.delete_user().await
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: UserController = UserController::new(UserService::new());
}

#[derive(Debug)]
pub struct UserController {
    user_service: UserService,
}

impl UserController {
    pub fn new(user_service: UserService) -> Self {
        Self { user_service }
    }

    #[with_middleware(guard = "log_call", when = "before", context = "call_context")]
    #[with_middleware(guard = "log_call_result", when = "after", context = "call_context")]
    async fn get_user(&self) -> ApiResult<GetUserResponse> {
        let ctx: CallContext = CallContext::get();
        let user = self
            .user_service
            .get_user_by_identity(&ctx.caller(), &ctx)?;

        Ok(GetUserResponse {
            user: UserDTO::from(user),
        })
    }

    #[with_middleware(guard = "log_call", when = "before", context = "call_context")]
    #[with_middleware(guard = "log_call_result", when = "after", context = "call_context")]
    async fn register_user(&self, input: RegisterUserInput) -> ApiResult<RegisterUserResponse> {
        let ctx: CallContext = CallContext::get();
        let user = self.user_service.register_user(input, &ctx).await?;

        Ok(RegisterUserResponse {
            user: UserDTO::from(user),
        })
    }

    #[with_middleware(guard = "log_call", when = "before", context = "call_context")]
    #[with_middleware(guard = "log_call_result", when = "after", context = "call_context")]
    async fn manage_user(&self, input: ManageUserInput) -> ApiResult<ManageUserResponse> {
        let ctx: CallContext = CallContext::get();
        let user = self.user_service.manage_user(input, &ctx).await?;

        Ok(ManageUserResponse {
            user: UserDTO::from(user),
        })
    }

    #[with_middleware(guard = "log_call", when = "before", context = "call_context")]
    #[with_middleware(guard = "log_call_result", when = "after", context = "call_context")]
    async fn delete_user(&self) -> ApiResult<DeleteUserResponse> {
        let ctx: CallContext = CallContext::get();
        let user = self
            .user_service
            .get_user_by_identity(&ctx.caller(), &ctx)?;

        let deleted_user = UserService::default().remove_user(&user.id, &ctx).await?;

        Ok(DeleteUserResponse {
            user: UserDTO::from(deleted_user),
        })
    }

    #[with_middleware(guard = "log_call", when = "before", context = "call_context")]
    #[with_middleware(guard = "log_call_result", when = "after", context = "call_context")]
    async fn associate_identity_with_user(
        &self,
        input: AssociateIdentityWithUserInput,
    ) -> ApiResult<AssociateIdentityWithUserResponse> {
        let ctx: CallContext = CallContext::get();
        let user = self
            .user_service
            .associate_identity_with_user(*HelperMapper::to_uuid(input.user_id)?.as_bytes(), &ctx)
            .await?;

        Ok(AssociateIdentityWithUserResponse {
            user: UserDTO::from(user),
        })
    }
}
