use anyhow::bail;
use candid::Principal;
use station_api::{GetRequestResponse, RequestOperationDTO};

pub(super) fn verify_call(
    request: &GetRequestResponse,
    expected_canister_id: &Principal,
    expected_method: &str,
    expected_arg_checksum: &Option<String>,
) -> anyhow::Result<()> {
    let RequestOperationDTO::CallExternalCanister(operation) = &request.request.operation else {
        bail!("The request is not a call external canister request");
    };
    if &operation.execution_method.canister_id != expected_canister_id {
        bail!(
            "The request targets an unexpected canister. Expected: {}, actual: {}",
            expected_canister_id,
            operation.execution_method.canister_id
        );
    }
    if operation.execution_method.method_name != expected_method {
        bail!(
            "The method of this request is not \"{}\" but \"{}\" instead",
            expected_method,
            operation.execution_method.method_name
        );
    }
    if &operation.arg_checksum != expected_arg_checksum {
        bail!("Argument checksum does not match");
    }

    Ok(())
}
