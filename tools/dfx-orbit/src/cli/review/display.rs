use itertools::Itertools;
use orbit_station_api::{
    GetRequestResponse, ListRequestsResponse, RequestOperationDTO, RequestStatusDTO,
};
use std::{collections::HashMap, fmt::Write};
use tabled::{
    settings::{Settings, Style},
    Table,
};

pub(crate) fn display_list(data: ListRequestsResponse) -> String {
    let add_info = data
        .additional_info
        .into_iter()
        .map(|info| (info.id.clone(), info))
        .collect::<HashMap<String, _>>();

    let data_iter = data.requests.iter().map(|request| {
        let add_info = add_info.get(&request.id);

        [
            request.id.clone(),
            add_info
                .map(|add_info| add_info.requester_name.clone())
                .unwrap_or(String::from("-")),
            request.title.clone(),
            display_request_operation(&request.operation).to_string(),
            display_request_status(&request.status).to_string(),
        ]
    });
    let titled_iter = std::iter::once([
        String::from("ID"),
        String::from("Requested by"),
        String::from("Title"),
        String::from("Operation"),
        String::from("Execution Status"),
    ])
    .chain(data_iter);

    let table_config = Settings::default().with(Style::psql());
    let table = Table::from_iter(titled_iter).with(table_config).to_string();

    table
}

pub(crate) fn display_get_request_response(request: GetRequestResponse) -> String {
    let base_info = request.request;
    let add_info = request.additional_info;

    let mut output = String::new();

    // General request information
    writeln!(output, "===REQUEST===").unwrap();
    writeln!(output, "ID: {}", base_info.id).unwrap();
    writeln!(
        output,
        "Operation: {}",
        display_request_operation(&base_info.operation)
    )
    .unwrap();
    writeln!(
        output,
        "Status: {}",
        display_request_status(&base_info.status)
    )
    .unwrap();
    writeln!(output, "Title: {}", base_info.title).unwrap();
    if let Some(summary) = base_info.summary {
        writeln!(output, "Summary: {}", summary).unwrap()
    }
    writeln!(output, "Requested by: {}", add_info.requester_name).unwrap();
    writeln!(
        output,
        "Approved by: {}",
        add_info
            .approvers
            .into_iter()
            .map(|approver| approver.name)
            .join(", ")
    )
    .unwrap();
    // write!(output, "ID: {}\n", base_info.id).unwrap();
    //  approved by (comma sepatated)

    // TODO: Display operation
    // TODO: Per operation additional information

    output
}

fn display_request_operation(op: &RequestOperationDTO) -> &'static str {
    match op {
        RequestOperationDTO::Transfer(_) => "Transfer",
        RequestOperationDTO::AddAccount(_) => "AddAccount",
        RequestOperationDTO::EditAccount(_) => "EditAccount",
        RequestOperationDTO::AddAddressBookEntry(_) => "AddAddressBookEntry",
        RequestOperationDTO::EditAddressBookEntry(_) => "EditAddressBookEntry",
        RequestOperationDTO::RemoveAddressBookEntry(_) => "RemoveAddressBookEntry",
        RequestOperationDTO::AddUser(_) => "AddUser",
        RequestOperationDTO::EditUser(_) => "EditUser",
        RequestOperationDTO::AddUserGroup(_) => "AddUserGroup",
        RequestOperationDTO::EditUserGroup(_) => "EditUserGroup",
        RequestOperationDTO::RemoveUserGroup(_) => "RemoveUserGroup",
        RequestOperationDTO::ChangeCanister(_) => "ChangeCanister",
        RequestOperationDTO::SetDisasterRecovery(_) => "SetDisasterRecovery",
        RequestOperationDTO::ChangeExternalCanister(_) => "ChangeExternalCanister",
        RequestOperationDTO::CreateExternalCanister(_) => "CreateExternalCanister",
        RequestOperationDTO::ConfigureExternalCanister(_) => "ConfigureExternalCanister",
        RequestOperationDTO::CallExternalCanister(_) => "CallExternalCanister",
        RequestOperationDTO::EditPermission(_) => "EditPermission",
        RequestOperationDTO::AddRequestPolicy(_) => "AddRequestPolicy",
        RequestOperationDTO::EditRequestPolicy(_) => "EditRequestPolicy",
        RequestOperationDTO::RemoveRequestPolicy(_) => "RemoveRequestPolicy",
        RequestOperationDTO::ManageSystemInfo(_) => "ManageSystemInfo",
    }
}

fn display_request_status(status: &RequestStatusDTO) -> &'static str {
    match status {
        RequestStatusDTO::Created => "Created",
        RequestStatusDTO::Approved => "Approved",
        RequestStatusDTO::Rejected => "Rejected",
        RequestStatusDTO::Cancelled { .. } => "Cancelled",
        RequestStatusDTO::Scheduled { .. } => "Scheduled",
        RequestStatusDTO::Processing { .. } => "Processing",
        RequestStatusDTO::Completed { .. } => "Completed",
        RequestStatusDTO::Failed { .. } => "Failed",
    }
}
