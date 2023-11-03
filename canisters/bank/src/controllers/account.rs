use crate::{
    core::{CallContext, WithCallContext, PERMISSION_READ_ACCOUNT, PERMISSION_WRITE_ACCOUNT},
    mappers::HelperMapper,
    services::AccountService,
    transport::{
        CreateAccountInput, CreateAccountResponse, FetchAccountBalancesInput,
        FetchAccountBalancesResponse, GetAccountInput, GetAccountResponse, ListAccountResponse,
    },
};
use ic_canister_core::api::ApiResult;
use ic_cdk_macros::{query, update};

#[update(name = "create_account")]
async fn create_account(input: CreateAccountInput) -> ApiResult<CreateAccountResponse> {
    CallContext::get().check_access(PERMISSION_WRITE_ACCOUNT);

    let created_account = AccountService::with_call_context(CallContext::get())
        .create_account(input)
        .await?
        .to_dto();

    Ok(CreateAccountResponse {
        account: created_account,
    })
}

#[query(name = "get_account")]
async fn get_account(input: GetAccountInput) -> ApiResult<GetAccountResponse> {
    CallContext::get().check_access(PERMISSION_READ_ACCOUNT);

    let account = AccountService::with_call_context(CallContext::get())
        .get_account(HelperMapper::to_uuid(input.account_id)?.as_bytes())?
        .to_dto();

    Ok(GetAccountResponse { account })
}

#[query(name = "list_accounts")]
async fn list_accounts() -> ApiResult<ListAccountResponse> {
    let ctx = CallContext::get();
    let owner_identity = ctx.caller();
    ctx.check_access(PERMISSION_READ_ACCOUNT);

    let accounts = AccountService::with_call_context(ctx)
        .list_accounts(owner_identity)?
        .iter()
        .map(|account| account.to_dto())
        .collect();

    Ok(ListAccountResponse { accounts })
}

#[update(name = "fetch_account_balances")]
async fn fetch_account_balances(
    input: FetchAccountBalancesInput,
) -> ApiResult<FetchAccountBalancesResponse> {
    CallContext::get().check_access(PERMISSION_READ_ACCOUNT);

    let balances = AccountService::with_call_context(CallContext::get())
        .fetch_account_balances(input)
        .await?;

    Ok(FetchAccountBalancesResponse { balances })
}
