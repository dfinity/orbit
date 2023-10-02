use crate::{
    core::{CallContext, WithCallContext},
    services::TransferService,
    transport::{
        GetTransferInput, GetTransferResponse, ListWalletTransfersInput,
        ListWalletTransfersResponse, TransferInput, TransferResponse,
    },
};
use ic_canister_core::api::ApiResult;
use ic_cdk_macros::update;

#[update(name = "transfer")]
async fn transfer(input: TransferInput) -> ApiResult<TransferResponse> {
    let transfer = TransferService::create()
        .with_call_context(CallContext::get())
        .create_transfer(input)
        .await?;

    Ok(TransferResponse { transfer })
}
