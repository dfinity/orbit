use crate::DfxOrbit;
use clap::Parser;
use itertools::Itertools;
use station_api::{MeResponse, UserPrivilege, UserStatusDTO};
use std::fmt::Write;

#[derive(Debug, Clone, Parser)]
pub struct MeArgs {
    /// Return output as JSON
    #[clap(short, long)]
    pub(crate) json: bool,
}

impl DfxOrbit {
    pub(crate) fn display_me(&self, response: MeResponse) -> anyhow::Result<String> {
        let mut output = String::new();

        writeln!(output, "Name: {}", response.me.name)?;
        writeln!(output, "Id: {}", response.me.id)?;
        writeln!(
            output,
            "Status: {}",
            display_user_status_dto(&response.me.status)
        )?;
        writeln!(
            output,
            "Identities: {}",
            response
                .me
                .identities
                .iter()
                .map(|p| format!("\n\t{}", p))
                .join("")
        )?;
        writeln!(
            output,
            "Groups: {}",
            response
                .me
                .groups
                .iter()
                .map(|group| format!("\n\t{} ({})", group.name, group.id))
                .join("")
        )?;
        writeln!(
            output,
            "Privileges: {}",
            response
                .privileges
                .iter()
                .map(|p| format!("\n\t{}", display_privilege(p)))
                .join("")
        )?;

        Ok(output)
    }
}

fn display_user_status_dto(status: &UserStatusDTO) -> &'static str {
    match status {
        UserStatusDTO::Active => "Active",
        UserStatusDTO::Inactive => "Inactive",
    }
}

fn display_privilege(privilege: &UserPrivilege) -> &'static str {
    match privilege {
        UserPrivilege::Capabilities => "Capabilities",
        UserPrivilege::SystemInfo => "SystemInfo",
        UserPrivilege::ManageSystemInfo => "ManageSystemInfo",
        UserPrivilege::ListAccounts => "ListAccounts",
        UserPrivilege::AddAccount => "AddAccount",
        UserPrivilege::ListUsers => "ListUsers",
        UserPrivilege::AddUser => "AddUser",
        UserPrivilege::ListUserGroups => "ListUserGroups",
        UserPrivilege::AddUserGroup => "AddUserGroup",
        UserPrivilege::ListPermissions => "ListPermissions",
        UserPrivilege::ListRequestPolicies => "ListRequestPolicies",
        UserPrivilege::AddRequestPolicy => "AddRequestPolicy",
        UserPrivilege::ListAddressBookEntries => "ListAddressBookEntries",
        UserPrivilege::AddAddressBookEntry => "AddAddressBookEntry",
        UserPrivilege::SystemUpgrade => "SystemUpgrade",
        UserPrivilege::ListRequests => "ListRequests",
        UserPrivilege::CreateExternalCanister => "CreateExternalCanister",
        UserPrivilege::ListExternalCanisters => "ListExternalCanisters",
    }
}
