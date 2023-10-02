use crate::{
    core::{CallContext, WithCallContext},
    services::TransferService,
    transport::{
        GetTransferInput, GetTransferResponse, ListWalletTransfersInput,
        ListWalletTransfersResponse, TransferInput, TransferResponse,
    },
};
use ic_canister_core::api::ApiResult;
use ic_cdk_macros::{query, update};

#[update(name = "transfer")]
async fn transfer(input: TransferInput) -> ApiResult<TransferResponse> {
    let transfer = TransferService::create()
        .with_call_context(CallContext::get())
        .create_transfer(input)
        .await?;

    Ok(TransferResponse { transfer })
}

#[query(name = "get_transfer")]
async fn get_transfer(input: GetTransferInput) -> ApiResult<GetTransferResponse> {
    let transfer_dto = TransferService::create()
        .with_call_context(CallContext::get())
        .get_transfer(input)
        .await?;

    Ok(GetTransferResponse {
        transfer: transfer_dto,
    })
}

#[query(name = "list_wallet_transfers")]
async fn list_wallet_transfers(
    input: ListWalletTransfersInput,
) -> ApiResult<ListWalletTransfersResponse> {
    let transfers = TransferService::create()
        .with_call_context(CallContext::get())
        .list_wallet_transfers(input)
        .await?;

    Ok(ListWalletTransfersResponse { transfers })
}
