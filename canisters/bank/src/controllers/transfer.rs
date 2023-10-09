use crate::{
    core::{CallContext, WithCallContext, PERMISSION_READ_TRANSFER, PERMISSION_WRITE_TRANSFER},
    services::TransferService,
    transport::{
        GetTransferInput, GetTransferResponse, GetTransfersInput, GetTransfersResponse,
        ListWalletTransfersInput, ListWalletTransfersResponse, TransferInput, TransferResponse,
    },
};
use ic_canister_core::api::ApiResult;
use ic_cdk_macros::{query, update};

#[update(name = "transfer")]
async fn transfer(input: TransferInput) -> ApiResult<TransferResponse> {
    CallContext::get().check_access(PERMISSION_WRITE_TRANSFER);

    let transfer = TransferService::create()
        .with_call_context(CallContext::get())
        .create_transfer(input)
        .await?;

    Ok(TransferResponse { transfer })
}

#[query(name = "get_transfer")]
async fn get_transfer(input: GetTransferInput) -> ApiResult<GetTransferResponse> {
    CallContext::get().check_access(PERMISSION_READ_TRANSFER);

    let transfer_dto = TransferService::create()
        .with_call_context(CallContext::get())
        .get_transfer(input)
        .await?;

    Ok(GetTransferResponse {
        transfer: transfer_dto,
    })
}

#[query(name = "get_transfers")]
async fn get_transfers(input: GetTransfersInput) -> ApiResult<GetTransfersResponse> {
    CallContext::get().check_access(PERMISSION_READ_TRANSFER);

    let dtos = TransferService::create()
        .with_call_context(CallContext::get())
        .get_transfers(input)
        .await?;

    Ok(GetTransfersResponse { transfers: dtos })
}

#[query(name = "list_wallet_transfers")]
async fn list_wallet_transfers(
    input: ListWalletTransfersInput,
) -> ApiResult<ListWalletTransfersResponse> {
    CallContext::get().check_access(PERMISSION_READ_TRANSFER);

    let transfers = TransferService::create()
        .with_call_context(CallContext::get())
        .list_wallet_transfers(input)
        .await?;

    Ok(ListWalletTransfersResponse { transfers })
}
