use crate::core::PERMISSION_READ_TRANSFER;
use crate::{
    core::middlewares::{authorize, call_context},
    mappers::HelperMapper,
    services::TransferService,
    transport::{
        GetTransferInput, GetTransferResponse, GetTransfersInput, GetTransfersResponse,
        ListAccountTransfersInput, ListAccountTransfersResponse,
    },
};
use ic_canister_core::api::{ApiError, ApiResult};
use ic_canister_macros::with_middleware;
use ic_cdk_macros::query;
use lazy_static::lazy_static;

// Canister entrypoints for the controller.
#[query(name = "get_transfer")]
async fn get_transfer(input: GetTransferInput) -> ApiResult<GetTransferResponse> {
    CONTROLLER.get_transfer(input).await
}

#[query(name = "get_transfers")]
async fn get_transfers(input: GetTransfersInput) -> ApiResult<GetTransfersResponse> {
    CONTROLLER.get_transfers(input).await
}

#[query(name = "list_account_transfers")]
async fn list_account_transfers(
    input: ListAccountTransfersInput,
) -> ApiResult<ListAccountTransfersResponse> {
    CONTROLLER.list_account_transfers(input).await
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: TransferController = TransferController::new(TransferService::default());
}

#[derive(Debug)]
pub struct TransferController {
    transfer_service: TransferService,
}

impl TransferController {
    fn new(transfer_service: TransferService) -> Self {
        Self { transfer_service }
    }

    #[with_middleware(guard = "authorize", context = "call_context", args = [PERMISSION_READ_TRANSFER])]
    async fn get_transfer(&self, input: GetTransferInput) -> ApiResult<GetTransferResponse> {
        let transfer = self.transfer_service.get_transfer(
            HelperMapper::to_uuid(input.transfer_id)?.as_bytes(),
            &call_context(),
        )?;

        Ok(GetTransferResponse {
            transfer: transfer.to_dto(),
        })
    }

    #[with_middleware(guard = "authorize", context = "call_context", args = [PERMISSION_READ_TRANSFER])]
    async fn get_transfers(&self, input: GetTransfersInput) -> ApiResult<GetTransfersResponse> {
        let ids: Vec<_> = input
            .transfer_ids
            .iter()
            .try_fold(Vec::new(), |mut acc, id| {
                let uuid = HelperMapper::to_uuid(id.clone())?;
                acc.push(*uuid.as_bytes());
                Ok::<Vec<_>, ApiError>(acc)
            })?;

        let transfers = self.transfer_service.get_transfers(ids, &call_context())?;

        Ok(GetTransfersResponse {
            transfers: transfers.into_iter().map(|t| t.to_dto()).collect(),
        })
    }

    #[with_middleware(guard = "authorize", context = "call_context", args = [PERMISSION_READ_TRANSFER])]
    async fn list_account_transfers(
        &self,
        input: ListAccountTransfersInput,
    ) -> ApiResult<ListAccountTransfersResponse> {
        let transfers = self
            .transfer_service
            .list_account_transfers(input, &call_context())?;

        Ok(ListAccountTransfersResponse {
            transfers: transfers
                .into_iter()
                .map(|t| t.to_list_item_dto())
                .collect(),
        })
    }
}
