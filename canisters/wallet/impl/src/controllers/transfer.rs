use crate::{
    core::middlewares::{authorize, call_context},
    mappers::HelperMapper,
    models::access_control::ResourceSpecifier,
    services::TransferService,
};
use ic_canister_core::api::{ApiError, ApiResult};
use ic_canister_macros::with_middleware;
use ic_cdk_macros::query;
use lazy_static::lazy_static;
use wallet_api::{
    GetTransferResponse, GetTransfersInput, GetTransfersResponse, ListAccountTransfersInput,
    ListAccountTransfersResponse,
};

// Canister entrypoints for the controller.
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

    #[with_middleware(
        guard = "authorize",
        context = "call_context",
        args = [ResourceSpecifier::from(&input)],
        is_async = true
    )]
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

    #[with_middleware(
        guard = "authorize",
        context = "call_context",
        args = [ResourceSpecifier::from(&input)],
        is_async = true
    )]
    async fn list_account_transfers(
        &self,
        input: ListAccountTransfersInput,
    ) -> ApiResult<ListAccountTransfersResponse> {
        let transfers = self.transfer_service.list_account_transfers(input)?;

        Ok(ListAccountTransfersResponse {
            transfers: transfers
                .into_iter()
                .map(|t| t.to_list_item_dto())
                .collect(),
        })
    }
}
