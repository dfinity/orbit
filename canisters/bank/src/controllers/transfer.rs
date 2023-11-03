use crate::{
    core::{CallContext, WithCallContext, PERMISSION_READ_TRANSFER, PERMISSION_WRITE_TRANSFER},
    mappers::HelperMapper,
    services::TransferService,
    transport::{
        GetTransferInput, GetTransferResponse, GetTransfersInput, GetTransfersResponse,
        ListAccountTransfersInput, ListAccountTransfersResponse, TransferInput, TransferResponse,
    },
};
use ic_canister_core::api::{ApiError, ApiResult};
use ic_cdk_macros::{query, update};

#[update(name = "transfer")]
async fn transfer(input: TransferInput) -> ApiResult<TransferResponse> {
    CallContext::get().check_access(PERMISSION_WRITE_TRANSFER);

    let transfer = TransferService::with_call_context(CallContext::get())
        .create_transfer(input)
        .await?;

    Ok(TransferResponse {
        transfer: transfer.to_dto(),
    })
}

#[query(name = "get_transfer")]
async fn get_transfer(input: GetTransferInput) -> ApiResult<GetTransferResponse> {
    CallContext::get().check_access(PERMISSION_READ_TRANSFER);

    let transfer = TransferService::with_call_context(CallContext::get())
        .get_transfer(HelperMapper::to_uuid(input.transfer_id)?.as_bytes())?;

    Ok(GetTransferResponse {
        transfer: transfer.to_dto(),
    })
}

#[query(name = "get_transfers")]
async fn get_transfers(input: GetTransfersInput) -> ApiResult<GetTransfersResponse> {
    CallContext::get().check_access(PERMISSION_READ_TRANSFER);

    let ids: Vec<_> = input
        .transfer_ids
        .iter()
        .try_fold(Vec::new(), |mut acc, id| {
            let uuid = HelperMapper::to_uuid(id.clone())?;
            acc.push(*uuid.as_bytes());
            Ok::<Vec<_>, ApiError>(acc)
        })?;

    let transfers = TransferService::with_call_context(CallContext::get()).get_transfers(ids)?;

    Ok(GetTransfersResponse {
        transfers: transfers.into_iter().map(|t| t.to_dto()).collect(),
    })
}

#[query(name = "list_account_transfers")]
async fn list_account_transfers(
    input: ListAccountTransfersInput,
) -> ApiResult<ListAccountTransfersResponse> {
    CallContext::get().check_access(PERMISSION_READ_TRANSFER);

    let transfers =
        TransferService::with_call_context(CallContext::get()).list_account_transfers(input)?;

    Ok(ListAccountTransfersResponse {
        transfers: transfers
            .into_iter()
            .map(|t| t.to_list_item_dto())
            .collect(),
    })
}
