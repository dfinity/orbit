//! Account services.
use crate::{
    core::{CallContext, WithCallContext},
    mappers::HelperMapper,
    services::AccountService,
    transport::{
        AccountDTO, AssociateIdentityWithAccountInput, AssociateIdentityWithAccountResponse,
        DeleteAccountResponse, GetAccountResponse, ManageAccountInput, ManageAccountResponse,
        RegisterAccountInput, RegisterAccountResponse,
    },
};
use ic_canister_core::api::ApiResult;
use ic_cdk_macros::{query, update};

#[query(name = "get_account")]
async fn get_account() -> ApiResult<GetAccountResponse> {
    let account = AccountService::with_call_context(CallContext::get())
        .get_account_by_identity(&CallContext::get().caller())?;

    Ok(GetAccountResponse {
        account: AccountDTO::from(account),
    })
}

#[update(name = "register_account")]
async fn register_account(input: RegisterAccountInput) -> ApiResult<RegisterAccountResponse> {
    let account = AccountService::with_call_context(CallContext::get())
        .register_account(input)
        .await?;

    Ok(RegisterAccountResponse {
        account: AccountDTO::from(account),
    })
}

#[update(name = "manage_account")]
async fn manage_account(input: ManageAccountInput) -> ApiResult<ManageAccountResponse> {
    let account = AccountService::with_call_context(CallContext::get())
        .manage_account(input)
        .await?;

    Ok(ManageAccountResponse {
        account: AccountDTO::from(account),
    })
}

#[update(name = "associate_identity_with_account")]
async fn associate_identity_with_account(
    input: AssociateIdentityWithAccountInput,
) -> ApiResult<AssociateIdentityWithAccountResponse> {
    let account = AccountService::with_call_context(CallContext::get())
        .associate_identity_with_account(*HelperMapper::to_uuid(input.account_id)?.as_bytes())
        .await?;

    Ok(AssociateIdentityWithAccountResponse {
        account: AccountDTO::from(account),
    })
}

#[update(name = "delete_account")]
async fn delete_account() -> ApiResult<DeleteAccountResponse> {
    let service = AccountService::with_call_context(CallContext::get());
    let account = service.get_account_by_identity(&CallContext::get().caller())?;

    let deleted_account = AccountService::default()
        .remove_account(&account.id)
        .await?;

    Ok(DeleteAccountResponse {
        account: AccountDTO::from(deleted_account),
    })
}
