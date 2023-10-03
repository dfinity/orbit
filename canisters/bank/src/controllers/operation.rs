use crate::{
    core::{CallContext, WithCallContext, PERMISSION_READ_OPERATION, PERMISSION_WRITE_OPERATION},
    services::OperationService,
    transport::{
        EditOperationInput, EditOperationResponse, GetOperationInput, GetOperationResponse,
        ListOperationsInput, ListOperationsResponse, ListWalletOperationsInput,
        ListWalletOperationsResponse,
    },
};
use ic_canister_core::api::ApiResult;
use ic_cdk_macros::{query, update};

#[query(name = "list_operations")]
async fn list_operations(input: ListOperationsInput) -> ApiResult<ListOperationsResponse> {
    CallContext::get().check_access(PERMISSION_READ_OPERATION);

    let operations = OperationService::create()
        .with_call_context(CallContext::get())
        .list_operations(input)
        .await?;

    Ok(ListOperationsResponse { operations })
}

#[query(name = "list_wallet_operations")]
async fn list_wallet_operations(
    input: ListWalletOperationsInput,
) -> ApiResult<ListWalletOperationsResponse> {
    CallContext::get().check_access(PERMISSION_READ_OPERATION);

    let operations = OperationService::create()
        .with_call_context(CallContext::get())
        .list_wallet_operations(input)
        .await?;

    Ok(ListWalletOperationsResponse { operations })
}

#[query(name = "get_operation")]
async fn get_operation(input: GetOperationInput) -> ApiResult<GetOperationResponse> {
    CallContext::get().check_access(PERMISSION_READ_OPERATION);

    let operation = OperationService::create()
        .with_call_context(CallContext::get())
        .get_operation(input)
        .await?;

    Ok(GetOperationResponse { operation })
}

#[update(name = "edit_operation")]
async fn edit_operation(input: EditOperationInput) -> ApiResult<EditOperationResponse> {
    CallContext::get().check_access(PERMISSION_WRITE_OPERATION);

    let operation = OperationService::create()
        .with_call_context(CallContext::get())
        .edit_operation(input)
        .await?;

    Ok(EditOperationResponse { operation })
}
