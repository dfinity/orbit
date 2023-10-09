use crate::{
    core::{CallContext, WithCallContext, PERMISSION_READ_WALLET, PERMISSION_WRITE_WALLET},
    services::WalletService,
    transport::{
        CreateWalletInput, CreateWalletResponse, FetchWalletBalancesInput,
        FetchWalletBalancesResponse, GetWalletInput, GetWalletResponse, ListWalletResponse,
    },
};
use ic_canister_core::api::ApiResult;
use ic_cdk_macros::{query, update};

#[update(name = "create_wallet")]
async fn create_wallet(input: CreateWalletInput) -> ApiResult<CreateWalletResponse> {
    CallContext::get().check_access(PERMISSION_WRITE_WALLET);

    let created_wallet = WalletService::create()
        .with_call_context(CallContext::get())
        .create_wallet(input)
        .await?;

    Ok(CreateWalletResponse {
        wallet: created_wallet,
    })
}

#[query(name = "get_wallet")]
async fn get_wallet(input: GetWalletInput) -> ApiResult<GetWalletResponse> {
    CallContext::get().check_access(PERMISSION_READ_WALLET);

    let wallet = WalletService::create()
        .with_call_context(CallContext::get())
        .get_wallet(input)
        .await?;

    Ok(GetWalletResponse { wallet })
}

#[query(name = "list_wallets")]
async fn list_wallets() -> ApiResult<ListWalletResponse> {
    CallContext::get().check_access(PERMISSION_READ_WALLET);

    let wallets = WalletService::create()
        .with_call_context(CallContext::get())
        .list_wallets(None)
        .await?;

    Ok(ListWalletResponse { wallets })
}

#[update(name = "fetch_wallet_balances")]
async fn fetch_wallet_balances(input: FetchWalletBalancesInput) -> ApiResult<FetchWalletBalancesResponse> {
    CallContext::get().check_access(PERMISSION_READ_WALLET);

    let balances = WalletService::create()
        .with_call_context(CallContext::get())
        .fetch_wallet_balances(input)
        .await?;

    Ok(FetchWalletBalancesResponse { balances })
}
