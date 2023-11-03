use crate::{
    core::{CallContext, WithCallContext, PERMISSION_READ_OPERATION, PERMISSION_WRITE_OPERATION},
    mappers::HelperMapper,
    services::OperationService,
    transport::{
        EditOperationInput, EditOperationResponse, GetOperationInput, GetOperationResponse,
        ListAccountOperationsInput, ListAccountOperationsResponse, ListOperationsInput,
        ListOperationsResponse,
    },
};
use ic_canister_core::api::{ApiError, ApiResult};
use ic_cdk_macros::{query, update};

#[query(name = "list_operations")]
async fn list_operations(input: ListOperationsInput) -> ApiResult<ListOperationsResponse> {
    CallContext::get().check_access(PERMISSION_READ_OPERATION);
    let service = OperationService::with_call_context(CallContext::get());

    let operations =
        service
            .list_operations(input)?
            .iter()
            .try_fold(Vec::new(), |mut acc, operation| {
                let operation_context = service.get_operation_context(&operation.id)?;
                acc.push(operation.to_dto(operation_context));
                Ok::<Vec<_>, ApiError>(acc)
            })?;

    Ok(ListOperationsResponse { operations })
}

#[query(name = "list_account_operations")]
async fn list_account_operations(
    input: ListAccountOperationsInput,
) -> ApiResult<ListAccountOperationsResponse> {
    CallContext::get().check_access(PERMISSION_READ_OPERATION);
    let service = OperationService::with_call_context(CallContext::get());

    let operations = service.list_account_operations(input)?.iter().try_fold(
        Vec::new(),
        |mut acc, operation| {
            let operation_context = service.get_operation_context(&operation.id)?;
            acc.push(operation.to_dto(operation_context));
            Ok::<Vec<_>, ApiError>(acc)
        },
    )?;

    Ok(ListAccountOperationsResponse { operations })
}

#[query(name = "get_operation")]
async fn get_operation(input: GetOperationInput) -> ApiResult<GetOperationResponse> {
    CallContext::get().check_access(PERMISSION_READ_OPERATION);
    let service = OperationService::with_call_context(CallContext::get());

    let operation = service.get_operation(HelperMapper::to_uuid(input.operation_id)?.as_bytes())?;
    let operation_context = service.get_operation_context(&operation.id)?;

    Ok(GetOperationResponse {
        operation: operation.to_dto(operation_context),
    })
}

#[update(name = "edit_operation")]
async fn edit_operation(input: EditOperationInput) -> ApiResult<EditOperationResponse> {
    CallContext::get().check_access(PERMISSION_WRITE_OPERATION);
    let service = OperationService::with_call_context(CallContext::get());

    let operation = service.edit_operation(input).await?;
    let operation_context = service.get_operation_context(&operation.id)?;

    Ok(EditOperationResponse {
        operation: operation.to_dto(operation_context),
    })
}
