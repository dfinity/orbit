use orbit_station_api::{ListRequestsResponse, RequestOperationDTO, RequestStatusDTO};
use std::collections::HashMap;
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
            match request.operation {
                RequestOperationDTO::Transfer(_) => String::from("Transfer"),
                RequestOperationDTO::AddAccount(_) => String::from("AddAccount"),
                RequestOperationDTO::EditAccount(_) => String::from("EditAccount"),
                RequestOperationDTO::AddAddressBookEntry(_) => String::from("AddAddressBookEntry"),
                RequestOperationDTO::EditAddressBookEntry(_) => {
                    String::from("EditAddressBookEntry")
                }
                RequestOperationDTO::RemoveAddressBookEntry(_) => {
                    String::from("RemoveAddressBookEntry")
                }
                RequestOperationDTO::AddUser(_) => String::from("AddUser"),
                RequestOperationDTO::EditUser(_) => String::from("EditUser"),
                RequestOperationDTO::AddUserGroup(_) => String::from("AddUserGroup"),
                RequestOperationDTO::EditUserGroup(_) => String::from("EditUserGroup"),
                RequestOperationDTO::RemoveUserGroup(_) => String::from("RemoveUserGroup"),
                RequestOperationDTO::ChangeCanister(_) => String::from("ChangeCanister"),
                RequestOperationDTO::SetDisasterRecovery(_) => String::from("SetDisasterRecovery"),
                RequestOperationDTO::ChangeExternalCanister(_) => {
                    String::from("ChangeExternalCanister")
                }
                RequestOperationDTO::CreateExternalCanister(_) => {
                    String::from("CreateExternalCanister")
                }
                RequestOperationDTO::ConfigureExternalCanister(_) => {
                    String::from("ConfigureExternalCanister")
                }
                RequestOperationDTO::CallExternalCanister(_) => {
                    String::from("CallExternalCanister")
                }
                RequestOperationDTO::EditPermission(_) => String::from("EditPermission"),
                RequestOperationDTO::AddRequestPolicy(_) => String::from("AddRequestPolicy"),
                RequestOperationDTO::EditRequestPolicy(_) => String::from("EditRequestPolicy"),
                RequestOperationDTO::RemoveRequestPolicy(_) => String::from("RemoveRequestPolicy"),
                RequestOperationDTO::ManageSystemInfo(_) => String::from("ManageSystemInfo"),
            },
            match request.status {
                RequestStatusDTO::Created => String::from("Created"),
                RequestStatusDTO::Approved => String::from("Approved"),
                RequestStatusDTO::Rejected => String::from("Rejected"),
                RequestStatusDTO::Cancelled { .. } => String::from("Cancelled"),
                RequestStatusDTO::Scheduled { .. } => String::from("Scheduled"),
                RequestStatusDTO::Processing { .. } => String::from("Processing"),
                RequestStatusDTO::Completed { .. } => String::from("Completed"),
                RequestStatusDTO::Failed { .. } => String::from("Failed"),
            },
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

// TODO: Display request for review id and review next
// TODO: ^ This needs canister id to name reverse backup
