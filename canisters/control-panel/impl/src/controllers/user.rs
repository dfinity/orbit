//! User services.
use crate::controllers::USER_REGISTRATION_RATE;
use crate::core::metrics::{
    COUNTER_DELETE_USER_TOTAL, COUNTER_MANAGE_USER_TOTAL, COUNTER_REGISTER_USER_TOTAL,
    COUNTER_SUBSCRIBE_TO_WAITING_LIST_TOTAL, COUNTER_UPDATE_WAITING_LIST_TOTAL,
};
use crate::core::middlewares::{call_context, logger};
use crate::services::USER_SERVICE;
use crate::{core::CallContext, services::UserService};
use control_panel_api::{
    DeleteUserResponse, GetUserResponse, ManageUserInput, ManageUserResponse, RegisterUserInput,
    RegisterUserResponse, UpdateWaitingListInput, UserDTO,
};
use ic_canister_core::api::{ApiError, ApiResult};
use ic_canister_macros::with_middleware;
use ic_cdk_macros::{query, update};
use lazy_static::lazy_static;
use prometheus::labels;
use std::cell::RefCell;
use std::sync::Arc;

thread_local! {
    pub static AVAILABLE_TOKENS_USER_REGISTRATION: RefCell<u32> = RefCell::new(USER_REGISTRATION_RATE);
}

// Canister entrypoints for the controller.

#[query(name = "get_user")]
async fn get_user() -> ApiResult<GetUserResponse> {
    CONTROLLER.get_user().await
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

    let out = CONTROLLER.register_user(input).await;

    COUNTER_REGISTER_USER_TOTAL.with(|c| {
        c.borrow()
            .with(&labels! {
                "status" => match &out {
                    Ok(_) => "ok",
                    Err(_) => "fail",
                }
            })
            .inc()
    });

    out
}

#[update(name = "manage_user")]
async fn manage_user(input: ManageUserInput) -> ApiResult<ManageUserResponse> {
    let out = CONTROLLER.manage_user(input).await;

    COUNTER_MANAGE_USER_TOTAL.with(|c| {
        c.borrow()
            .with(&labels! {
                "status" => match &out {
                    Ok(_) => "ok",
                    Err(_) => "fail",
                }
            })
            .inc()
    });

    out
}

#[update(name = "subscribe_to_waiting_list")]
async fn subscribe_to_waiting_list(email: String) -> ApiResult<()> {
    let out = CONTROLLER.subscribe_to_waiting_list(email).await;

    COUNTER_SUBSCRIBE_TO_WAITING_LIST_TOTAL.with(|c| {
        c.borrow()
            .with(&labels! {
                "status" => match &out {
                    Ok(_) => "ok",
                    Err(_) => "fail",
                }
            })
            .inc()
    });

    out
}

#[update(name = "update_waiting_list")]
async fn update_waiting_list(input: UpdateWaitingListInput) -> ApiResult<()> {
    let out = CONTROLLER.update_waiting_list(input).await;

    COUNTER_UPDATE_WAITING_LIST_TOTAL.with(|c| {
        c.borrow()
            .with(&labels! {
                "status" => match &out {
                    Ok(_) => "ok",
                    Err(_) => "fail",
                }
            })
            .inc()
    });

    out
}

#[update(name = "delete_user")]
async fn delete_user() -> ApiResult<DeleteUserResponse> {
    let out = CONTROLLER.delete_user().await;

    COUNTER_DELETE_USER_TOTAL.with(|c| {
        c.borrow()
            .with(&labels! {
                "status" => match &out {
                    Ok(_) => "ok",
                    Err(_) => "fail",
                }
            })
            .inc()
    });

    out
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

    #[with_middleware(
        guard = logger::<()>(__target_fn, context, None),
        tail = logger(__target_fn, context, Some(&result)),
        context = &call_context()
    )]
    async fn get_user(&self) -> ApiResult<GetUserResponse> {
        let ctx: CallContext = CallContext::get();
        let user = self.user_service.get_user(&ctx.caller(), &ctx)?;

        Ok(GetUserResponse {
            user: UserDTO::from(user),
        })
    }

    #[with_middleware(
        guard = logger::<()>(__target_fn, context, None),
        tail = logger(__target_fn, context, Some(&result)),
        context = &call_context()
    )]
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
    async fn manage_user(&self, input: ManageUserInput) -> ApiResult<ManageUserResponse> {
        let ctx: CallContext = CallContext::get();
        let user = self.user_service.manage_user(input, &ctx).await?;

        Ok(ManageUserResponse {
            user: UserDTO::from(user),
        })
    }

    #[with_middleware(
        guard = logger::<()>(__target_fn, context, None),
        tail = logger(__target_fn, context, Some(&result)),
        context = &call_context()
    )]
    async fn subscribe_to_waiting_list(&self, email: String) -> ApiResult<()> {
        let ctx: CallContext = CallContext::get();
        self.user_service
            .subscribe_to_waiting_list(email, &ctx)
            .await?;

        Ok(())
    }

    #[with_middleware(
        guard = logger::<()>(__target_fn, context, None),
        tail = logger(__target_fn, context, Some(&result)),
        context = &call_context()
    )]
    async fn update_waiting_list(&self, input: UpdateWaitingListInput) -> ApiResult<()> {
        let ctx: CallContext = CallContext::get();
        self.user_service.update_waiting_list(input, &ctx)?;

        Ok(())
    }

    #[with_middleware(
        guard = logger::<()>(__target_fn, context, None),
        tail = logger(__target_fn, context, Some(&result)),
        context = &call_context()
    )]
    async fn delete_user(&self) -> ApiResult<DeleteUserResponse> {
        let ctx: CallContext = CallContext::get();
        let user = self.user_service.get_user(&ctx.caller(), &ctx)?;

        let deleted_user = UserService::default().remove_user(&user.id, &ctx).await?;

        Ok(DeleteUserResponse {
            user: UserDTO::from(deleted_user),
        })
    }
}
