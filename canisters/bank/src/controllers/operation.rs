use crate::{
    core::{CallContext, WithCallContext},
    services::OperationService,
    transport::{
        EditOperationInput, EditOperationResponse, GetOperationInput, GetOperationResponse,
        ListOperationsInput, ListOperationsResponse,
    },
};
use ic_canister_core::api::ApiResult;
use ic_cdk_macros::{query, update};

#[query(name = "list_operations")]
async fn list_operations(input: ListOperationsInput) -> ApiResult<ListOperationsResponse> {
    let operations = OperationService::create()
        .with_call_context(CallContext::get())
        .list_operations(input)
        .await?;

    Ok(ListOperationsResponse { operations })
}

#[query(name = "get_operation")]
async fn get_operation(input: GetOperationInput) -> ApiResult<GetOperationResponse> {
    let operation = OperationService::create()
        .with_call_context(CallContext::get())
        .get_operation(input)
        .await?;

    Ok(GetOperationResponse { operation })
}

#[update(name = "edit_operation")]
async fn edit_operation(input: EditOperationInput) -> ApiResult<EditOperationResponse> {
    let operation = OperationService::create()
        .with_call_context(CallContext::get())
        .edit_operation(input)
        .await?;

    Ok(EditOperationResponse { operation })
}
