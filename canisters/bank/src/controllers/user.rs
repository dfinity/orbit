use crate::{
    core::{
        CallContext, WithCallContext, PERMISSION_READ_USER, PERMISSION_REGISTER_USER,
        PERMISSION_WRITE_USER,
    },
    mappers::HelperMapper,
    services::UserService,
    transport::{
        ConfirmUserIdentityInput, ConfirmUserIdentityResponse, EditUserInput, EditUserResponse,
        GetUserInput, GetUserResponse, RegisterUserInput, RegisterUserResponse,
    },
};
use ic_canister_core::api::ApiResult;
use ic_cdk_macros::{query, update};

#[update(name = "register_user")]
async fn register_user(input: RegisterUserInput) -> ApiResult<RegisterUserResponse> {
    CallContext::get().check_access(PERMISSION_REGISTER_USER);

    let user = UserService::with_call_context(CallContext::get())
        .register_user(input, Vec::new())
        .await?
        .to_dto();

    Ok(RegisterUserResponse { user })
}

#[update(name = "confirm_user_identity")]
async fn confirm_user_identity(
    input: ConfirmUserIdentityInput,
) -> ApiResult<ConfirmUserIdentityResponse> {
    CallContext::get().check_access(PERMISSION_REGISTER_USER);

    let user = UserService::with_call_context(CallContext::get())
        .confirm_user_identity(input)
        .await?
        .to_dto();

    Ok(ConfirmUserIdentityResponse { user })
}

#[update(name = "edit_user")]
async fn edit_user(input: EditUserInput) -> ApiResult<EditUserResponse> {
    CallContext::get().check_access(PERMISSION_WRITE_USER);

    let user = UserService::with_call_context(CallContext::get())
        .edit_user(input)
        .await?
        .to_dto();

    Ok(EditUserResponse { user })
}

#[query(name = "get_user")]
async fn get_user(input: GetUserInput) -> ApiResult<GetUserResponse> {
    CallContext::get().check_access(PERMISSION_READ_USER);

    let user = match input.user_id {
        Some(user_id) => UserService::with_call_context(CallContext::get())
            .get_user(HelperMapper::to_uuid(user_id)?.as_bytes())?
            .to_dto(),
        _ => UserService::with_call_context(CallContext::get())
            .get_user_by_identity(&CallContext::get().caller())?
            .to_dto(),
    };

    Ok(GetUserResponse { user })
}
