use crate::{
    core::{
        CallContext, WithCallContext, PERMISSION_READ_ACCOUNT, PERMISSION_REGISTER_ACCOUNT,
        PERMISSION_WRITE_ACCOUNT,
    },
    mappers::HelperMapper,
    services::AccountService,
    transport::{
        ConfirmAccountInput, ConfirmAccountResponse, EditAccountInput, EditAccountResponse,
        GetAccountInput, GetAccountResponse, RegisterAccountInput, RegisterAccountResponse,
    },
};
use ic_canister_core::api::ApiResult;
use ic_cdk_macros::{query, update};

#[update(name = "register_account")]
async fn register_account(input: RegisterAccountInput) -> ApiResult<RegisterAccountResponse> {
    CallContext::get().check_access(PERMISSION_REGISTER_ACCOUNT);

    let account = AccountService::with_call_context(CallContext::get())
        .register_account(input, Vec::new())
        .await?
        .to_dto();

    Ok(RegisterAccountResponse { account })
}

#[update(name = "confirm_account")]
async fn confirm_account(input: ConfirmAccountInput) -> ApiResult<ConfirmAccountResponse> {
    CallContext::get().check_access(PERMISSION_REGISTER_ACCOUNT);

    let account = AccountService::with_call_context(CallContext::get())
        .confirm_account(input)
        .await?
        .to_dto();

    Ok(ConfirmAccountResponse { account })
}

#[update(name = "edit_account")]
async fn edit_account(input: EditAccountInput) -> ApiResult<EditAccountResponse> {
    CallContext::get().check_access(PERMISSION_WRITE_ACCOUNT);

    let account = AccountService::with_call_context(CallContext::get())
        .edit_account(input)
        .await?
        .to_dto();

    Ok(EditAccountResponse { account })
}

#[query(name = "get_account")]
async fn get_account(input: GetAccountInput) -> ApiResult<GetAccountResponse> {
    CallContext::get().check_access(PERMISSION_READ_ACCOUNT);

    let account = match input.account_id {
        Some(account_id) => AccountService::with_call_context(CallContext::get())
            .get_account(HelperMapper::to_uuid(account_id)?.as_bytes())?
            .to_dto(),
        _ => AccountService::with_call_context(CallContext::get())
            .get_account_by_identity(&CallContext::get().caller())?
            .to_dto(),
    };

    Ok(GetAccountResponse { account })
}
