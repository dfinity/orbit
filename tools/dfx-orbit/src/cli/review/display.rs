use crate::DfxOrbit;
use candid::Principal;
use station_api::{
    CallExternalCanisterOperationDTO, CanisterInstallMode, ChangeExternalCanisterOperationDTO,
    GetRequestResponse, ListRequestsResponse, RequestAdditionalInfoDTO, RequestApprovalDTO,
    RequestApprovalStatusDTO, RequestDTO, RequestOperationDTO, RequestStatusDTO,
};
use std::{
    collections::{BTreeMap, HashMap},
    fmt::Write,
};
use tabled::{
    settings::{Settings, Style},
    Table,
};

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

    pub(crate) fn display_get_request_response(
        &self,
        request: GetRequestResponse,
    ) -> anyhow::Result<String> {
        let base_info = request.request;
        let add_info = request.additional_info;

        let mut output = String::new();

        // General request information
        writeln!(output, "=== REQUEST ===")?;
        writeln!(output, "ID: {}", base_info.id)?;
        writeln!(
            output,
            "Operation: {}",
            self.display_request_operation(&base_info.operation)
        )?;
        writeln!(output, "Title: {}", base_info.title)?;
        if let Some(ref summary) = base_info.summary {
            writeln!(output, "Summary: {}", summary)?
        }
        writeln!(output, "Requested by: {}", add_info.requester_name)?;

        self.display_approvers_and_rejectors(&mut output, &base_info, &add_info)?;

        writeln!(
            output,
            "Execution Status: {}",
            self.display_request_status(&base_info.status)
        )?;
        if let Some(additional_status) = self.display_additional_stats_info(&base_info.status) {
            writeln!(output, "{}", additional_status)?;
        }

        match base_info.operation {
            RequestOperationDTO::ChangeExternalCanister(op) => {
                self.display_change_canister_operation(&mut output, op.as_ref())?;
            }
            RequestOperationDTO::CallExternalCanister(op) => {
                self.display_call_canister_operation(&mut output, op.as_ref())?;
            }
            // TODO: CreateCanister Additional information
            // TODO: ConfigureCanister Additional information
            _ => (),
        };

        Ok(output)
    }

    fn display_change_canister_operation(
        &self,
        output: &mut String,
        op: &ChangeExternalCanisterOperationDTO,
    ) -> anyhow::Result<()> {
        writeln!(output, "=== Change External Canister ===")?;
        writeln!(
            output,
            "Target: {}",
            self.try_reverse_lookup(&op.canister_id)
        )?;

        let mode = match op.mode {
            CanisterInstallMode::Install => "Install",
            CanisterInstallMode::Reinstall => "Reinstall",
            CanisterInstallMode::Upgrade => "Upgrade",
        };
        writeln!(output, "Mode: {}", mode)?;

        writeln!(output, "Module checksum: {}", &op.module_checksum)?;
        if let Some(arg_checksum) = &op.arg_checksum {
            writeln!(output, "Argument checksum: {}", arg_checksum)?;
        }
        Ok(())
    }

    fn display_call_canister_operation(
        &self,
        output: &mut String,
        op: &CallExternalCanisterOperationDTO,
    ) -> anyhow::Result<()> {
        writeln!(output, "=== Call External Canister ===")?;
        writeln!(
            output,
            "Execution method: \"{}\" of {}",
            op.execution_method.method_name,
            self.try_reverse_lookup(&op.execution_method.canister_id)
        )?;
        if let Some(validation_method) = &op.validation_method {
            writeln!(
                output,
                "Validation method: \"{}\" of {}",
                validation_method.method_name,
                self.try_reverse_lookup(&validation_method.canister_id)
            )?
        }
        if let Some(checksum) = &op.arg_checksum {
            writeln!(output, "Argument checksum: {}", checksum)?
        }
        if let Some(args) = &op.arg_rendering {
            writeln!(output, "Argument: {}", args)?
        }
        if let Some(cycles) = &op.execution_method_cycles {
            writeln!(output, "Execution method cycles: {}", cycles)?
        }
        if let Some(reply) = &op.execution_method_reply {
            match candid_parser::IDLArgs::from_bytes(reply) {
                // TODO: Check if we can get the type information from somewhere to annotate this with types
                Ok(response) => writeln!(output, "Execution response: {}", response),
                Err(_) => writeln!(output, "FAILED TO PARSE EXECUTION RESPONSE"),
            }?;
        }

        Ok(())
    }

    fn display_approvers_and_rejectors<W: Write>(
        &self,
        writer: &mut W,
        base_info: &RequestDTO,
        add_info: &RequestAdditionalInfoDTO,
    ) -> anyhow::Result<()> {
        let usernames: BTreeMap<String, String> = add_info
            .approvers
            .iter()
            .map(|user| (user.id.clone(), user.name.clone()))
            .collect();

        let (approvers, rejectors): (Vec<_>, Vec<_>) = base_info
            .approvals
            .iter()
            .partition(|approval| approval.status == RequestApprovalStatusDTO::Approved);

        if !approvers.is_empty() {
            write!(writer, "Approved by: ")?;
            self.display_request_approvals(writer, approvers, &usernames)?;
        }
        if !rejectors.is_empty() {
            write!(writer, "Rejected by: ")?;
            self.display_request_approvals(writer, rejectors, &usernames)?;
        }

        Ok(())
    }

    fn display_request_approvals<W: Write>(
        &self,
        writer: &mut W,
        list: Vec<&RequestApprovalDTO>,
        usernames: &BTreeMap<String, String>,
    ) -> anyhow::Result<()> {
        for user in list {
            let name = usernames
                .get(&user.approver_id)
                .unwrap_or(&user.approver_id);
            write!(writer, "\n\t{}", name)?;
            if let Some(reason) = &user.status_reason {
                write!(writer, " ({})", reason)?;
            }
        }
        writeln!(writer)?;
        Ok(())
    }

    fn try_reverse_lookup(&self, canister_id: &Principal) -> String {
        match self.canister_name(canister_id).ok() {
            Some(canister_name) => {
                format!("{} ({})", canister_name, canister_id)
            }
            None => format!("{}", canister_id),
        }
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
            RequestOperationDTO::SystemUpgrade(_) => "SystemUpgrade",
            RequestOperationDTO::SetDisasterRecovery(_) => "SetDisasterRecovery",
            RequestOperationDTO::ChangeExternalCanister(_) => "ChangeExternalCanister",
            RequestOperationDTO::CreateExternalCanister(_) => "CreateExternalCanister",
            RequestOperationDTO::ConfigureExternalCanister(_) => "ConfigureExternalCanister",
            RequestOperationDTO::CallExternalCanister(_) => "CallExternalCanister",
            RequestOperationDTO::FundExternalCanister(_) => "FundExternalCanister",
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

    fn display_additional_stats_info(&self, status: &RequestStatusDTO) -> Option<String> {
        match status {
            RequestStatusDTO::Cancelled { reason } => {
                reason.clone().map(|reason| format!("Reason: {}", reason))
            }
            RequestStatusDTO::Failed { reason } => {
                reason.clone().map(|reason| format!("Reason: {}", reason))
            }
            _ => None,
        }
    }
}
