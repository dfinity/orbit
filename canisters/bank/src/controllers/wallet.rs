use crate::{
    core::{CallContext, WithCallContext},
    services::WalletService,
    transport::{CreateWalletInput, CreateWalletResponse, GetWalletInput, GetWalletResponse},
};
use ic_canister_core::api::ApiResult;
use ic_cdk_macros::{query, update};

#[update(name = "create_wallet")]
async fn create_wallet(input: CreateWalletInput) -> ApiResult<CreateWalletResponse> {
    let created_wallet = WalletService::new()
        .with_call_context(CallContext::get())
        .create_wallet(input)
        .await?;

    Ok(CreateWalletResponse {
        wallet: created_wallet,
    })
}

#[query(name = "get_wallet")]
async fn get_wallet(input: GetWalletInput) -> ApiResult<GetWalletResponse> {
    let wallet = WalletService::new()
        .with_call_context(CallContext::get())
        .get_wallet(input)
        .await?;

    Ok(GetWalletResponse { wallet })
}
