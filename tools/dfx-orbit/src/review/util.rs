use serde::Serialize;
use station_api::ListRequestsOperationTypeDTO;

pub(super) fn print_as_json<D>(data: D) -> anyhow::Result<()>
where
    D: Serialize,
{
    println!("{}", serde_json::to_string_pretty(&data)?);
    Ok(())
}

pub(super) fn external_canister_operations() -> Vec<ListRequestsOperationTypeDTO> {
    vec![
        ListRequestsOperationTypeDTO::ChangeExternalCanister(None),
        ListRequestsOperationTypeDTO::CreateExternalCanister,
        ListRequestsOperationTypeDTO::CallExternalCanister(None),
        ListRequestsOperationTypeDTO::ConfigureExternalCanister(None),
    ]
}
