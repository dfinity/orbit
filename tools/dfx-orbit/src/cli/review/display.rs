use itertools::Itertools;
use orbit_station_api::{
    CanisterInstallMode, ChangeExternalCanisterOperationDTO, GetRequestResponse,
    ListRequestsResponse, RequestOperationDTO, RequestStatusDTO,
};
use std::{collections::HashMap, fmt::Write};
use tabled::{
    settings::{Settings, Style},
    Table,
};

use crate::DfxOrbit;

impl DfxOrbit {
    pub(crate) fn display_list(&self, data: ListRequestsResponse) -> String {
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
                self.display_request_operation(&request.operation)
                    .to_string(),
                self.display_request_status(&request.status).to_string(),
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

    pub(crate) fn display_get_request_response(&self, request: GetRequestResponse) -> String {
        let base_info = request.request;
        let add_info = request.additional_info;

        let mut output = String::new();

        // General request information
        writeln!(output, "===REQUEST===").unwrap();
        writeln!(output, "ID: {}", base_info.id).unwrap();
        writeln!(
            output,
            "Operation: {}",
            self.display_request_operation(&base_info.operation)
        )
        .unwrap();
        writeln!(
            output,
            "Status: {}",
            self.display_request_status(&base_info.status)
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

        match base_info.operation {
            RequestOperationDTO::ChangeExternalCanister(op) => {
                self.display_change_canister_operation(&mut output, op.as_ref())
            }
            _ => (),
        };
        // write!(output, "ID: {}\n", base_info.id).unwrap();
        //  approved by (comma sepatated)

        // TODO: Display operation
        // TODO: Per operation additional information

        output
    }

    fn display_change_canister_operation(
        &self,
        output: &mut String,
        op: &ChangeExternalCanisterOperationDTO,
    ) {
        writeln!(output, "=== Change External Canister ===").unwrap();
        match self.canister_name(&op.canister_id).ok() {
            Some(canister_name) => {
                writeln!(output, "Target: {} ({})", canister_name, &op.canister_id)
            }
            None => writeln!(output, "Target: {}", &op.canister_id),
        }
        .unwrap();

        let mode = match op.mode {
            CanisterInstallMode::Install => "Install",
            CanisterInstallMode::Reinstall => "Reinstall",
            CanisterInstallMode::Upgrade => "Upgrade",
        };
        writeln!(output, "Mode {}", mode).unwrap();

        writeln!(output, "Module checksum: 0x{}", &op.module_checksum).unwrap();
        op.arg_checksum
            .as_ref()
            .map(|arg_checksum| writeln!(output, "Arg checksum: 0x{}", arg_checksum).unwrap());
    }

    fn display_request_operation(&self, op: &RequestOperationDTO) -> &'static str {
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

    fn display_request_status(&self, status: &RequestStatusDTO) -> &'static str {
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
}
