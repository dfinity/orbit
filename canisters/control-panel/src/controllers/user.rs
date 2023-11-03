//! User services.
use crate::{
    core::{CallContext, WithCallContext},
    mappers::HelperMapper,
    services::UserService,
    transport::{
        AssociateIdentityWithUserInput, AssociateIdentityWithUserResponse, DeleteUserResponse,
        GetUserResponse, ManageUserInput, ManageUserResponse, RegisterUserInput,
        RegisterUserResponse, UserDTO,
    },
};
use ic_canister_core::api::ApiResult;
use ic_cdk_macros::{query, update};

#[query(name = "get_user")]
async fn get_user() -> ApiResult<GetUserResponse> {
    let user = UserService::with_call_context(CallContext::get())
        .get_user_by_identity(&CallContext::get().caller())?;

    Ok(GetUserResponse {
        user: UserDTO::from(user),
    })
}

#[update(name = "register_user")]
async fn register_user(input: RegisterUserInput) -> ApiResult<RegisterUserResponse> {
    let user = UserService::with_call_context(CallContext::get())
        .register_user(input)
        .await?;

    Ok(RegisterUserResponse {
        user: UserDTO::from(user),
    })
}

#[update(name = "manage_user")]
async fn manage_user(input: ManageUserInput) -> ApiResult<ManageUserResponse> {
    let user = UserService::with_call_context(CallContext::get())
        .manage_user(input)
        .await?;

    Ok(ManageUserResponse {
        user: UserDTO::from(user),
    })
}

#[update(name = "associate_identity_with_user")]
async fn associate_identity_with_user(
    input: AssociateIdentityWithUserInput,
) -> ApiResult<AssociateIdentityWithUserResponse> {
    let user = UserService::with_call_context(CallContext::get())
        .associate_identity_with_user(*HelperMapper::to_uuid(input.user_id)?.as_bytes())
        .await?;

    Ok(AssociateIdentityWithUserResponse {
        user: UserDTO::from(user),
    })
}

#[update(name = "delete_user")]
async fn delete_user() -> ApiResult<DeleteUserResponse> {
    let service = UserService::with_call_context(CallContext::get());
    let user = service.get_user_by_identity(&CallContext::get().caller())?;

    let deleted_user = UserService::default().remove_user(&user.id).await?;

    Ok(DeleteUserResponse {
        user: UserDTO::from(deleted_user),
    })
}
