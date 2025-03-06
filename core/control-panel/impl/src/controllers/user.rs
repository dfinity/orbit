//! User services.
use crate::controllers::USER_REGISTRATION_RATE;
use crate::core::middlewares::{call_context, logger, use_canister_call_metric};
use crate::services::USER_SERVICE;
use crate::{core::CallContext, services::UserService};
use control_panel_api::{
    DeleteUserResponse, GetUserResponse, RegisterUserInput, RegisterUserResponse, UserDTO,
};
use ic_cdk_macros::{query, update};
use lazy_static::lazy_static;
use orbit_essentials::api::{ApiError, ApiResult};
use orbit_essentials::with_middleware;
use std::cell::RefCell;
use std::sync::Arc;

thread_local! {
    pub static AVAILABLE_TOKENS_USER_REGISTRATION: RefCell<u32> = const { RefCell::new(USER_REGISTRATION_RATE) };
}

// Canister entrypoints for the controller.

#[query(name = "get_user")]
async fn get_user() -> ApiResult<GetUserResponse> {
    CONTROLLER.get_user().await
}

#[update(name = "set_user_active")]
async fn set_user_active() -> ApiResult<()> {
    CONTROLLER.set_user_active().await
}

#[update(name = "register_user")]
async fn register_user(input: RegisterUserInput) -> ApiResult<RegisterUserResponse> {
    AVAILABLE_TOKENS_USER_REGISTRATION.with(|ts| {
        let mut ts = ts.borrow_mut();

        if *ts < 1 {
            return Err(ApiError::new("rate limited".into(), None, None));
        }

        *ts -= 1;

        Ok(())
    })?;

    CONTROLLER.register_user(input).await
}

#[update(name = "delete_user")]
async fn delete_user() -> ApiResult<DeleteUserResponse> {
    CONTROLLER.delete_user().await
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: UserController = UserController::new(Arc::clone(&USER_SERVICE));
}

#[derive(Debug)]
pub struct UserController {
    user_service: Arc<UserService>,
}

impl UserController {
    pub fn new(user_service: Arc<UserService>) -> Self {
        Self { user_service }
    }

    async fn get_user(&self) -> ApiResult<GetUserResponse> {
        let ctx: CallContext = CallContext::get();
        let user = self
            .user_service
            .get_user_by_identity(&ctx.caller(), &ctx)?;

        Ok(GetUserResponse {
            user: UserDTO::from(user),
        })
    }

    #[with_middleware(
        guard = logger::<()>(__target_fn, context, None),
        tail = logger(__target_fn, context, Some(&result)),
        context = &call_context()
    )]
    #[with_middleware(tail = use_canister_call_metric("register_user", &result))]
    async fn register_user(&self, input: RegisterUserInput) -> ApiResult<RegisterUserResponse> {
        let ctx: CallContext = CallContext::get();
        let user = self.user_service.register_user(input, &ctx).await?;

        Ok(RegisterUserResponse {
            user: UserDTO::from(user),
        })
    }

    #[with_middleware(
        guard = logger::<()>(__target_fn, context, None),
        tail = logger(__target_fn, context, Some(&result)),
        context = &call_context()
    )]
    #[with_middleware(tail = use_canister_call_metric("delete_user", &result))]
    async fn delete_user(&self) -> ApiResult<DeleteUserResponse> {
        let ctx: CallContext = CallContext::get();
        let user = self
            .user_service
            .get_user_by_identity(&ctx.caller(), &ctx)?;

        let deleted_user = UserService::default()
            .remove_user(&user.identity, &ctx)
            .await?;

        Ok(DeleteUserResponse {
            user: UserDTO::from(deleted_user),
        })
    }

    #[with_middleware(
        guard = logger::<()>(__target_fn, context, None),
        tail = logger(__target_fn, context, Some(&result)),
        context = &call_context()
    )]
    #[with_middleware(tail = use_canister_call_metric("set_user_active", &result))]
    async fn set_user_active(&self) -> ApiResult<()> {
        let ctx = CallContext::get();
        self.user_service
            .set_last_active(&ctx.caller(), &ctx)
            .await?;

        Ok(())
    }
}
