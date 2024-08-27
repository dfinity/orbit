//! Makes `EditPermission` requests to Orbit.

use crate::DfxOrbit;
use clap::{Parser, Subcommand};
use station_api::{
    EditPermissionOperationInput, ExternalCanisterIdDTO, ExternalCanisterResourceActionDTO,
    PermissionResourceActionDTO, RequestOperationInput, ResourceDTO,
};

/// Request permission.
#[derive(Debug, Clone, Subcommand)]
#[clap(version, about, long_about = None)]
pub enum RequestPermissionArgs {
    /// Request permission to read permission(s)
    ReadPermissions(RequestPermissionReadPermissionsArgs),
    /// Request permission to update permission(s)
    UpdatePermissions(RequestPermissionUpdatePermissionsArgs),
    /// Requests the permisson to propose canister upgrades
    UpgradeCanister(RequestPermissionUpgradeCanisterArgs),
}

impl RequestPermissionArgs {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    pub(crate) fn into_create_request_input(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        let operation = match self {
            RequestPermissionArgs::UpgradeCanister(canister_args) => {
                canister_args.into_create_request_input(dfx_orbit)?
            }
            RequestPermissionArgs::ReadPermissions(permission_args) => permission_args.into(),
            RequestPermissionArgs::UpdatePermissions(permission_args) => permission_args.into(),
        };

        Ok(operation)
    }
}

/// Requests the permisson to propose canister upgrades.
#[derive(Debug, Clone, Parser)]
pub struct RequestPermissionUpgradeCanisterArgs {
    /// A users that should be permitted to change permissions.  WARNING: Any user that is not listed will lose the ability to change permissions.
    #[structopt(long)]
    pub user: Vec<String>,
    /// A groups that should be permitted to change permissions.  WARNING: Any group that is not listed will lose the ability to change permissions.
    #[structopt(long)]
    pub group: Vec<String>,
    /// Canister name or ID. If none specified, this will request all
    // TODO: If a canister is not specified, require --all.
    pub canister: Option<String>,
}

impl RequestPermissionUpgradeCanisterArgs {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    pub(crate) fn into_create_request_input(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        let canisters: ExternalCanisterIdDTO = if let Some(canister_name_or_id) = self.canister {
            dfx_orbit
                .canister_id(&canister_name_or_id)
                .map(ExternalCanisterIdDTO::Canister)?
        } else {
            ExternalCanisterIdDTO::Any
        };

        let resource =
            ResourceDTO::ExternalCanister(ExternalCanisterResourceActionDTO::Change(canisters));

        Ok(RequestOperationInput::EditPermission(
            EditPermissionOperationInput {
                resource,
                auth_scope: None,
                users: Some(self.user),
                user_groups: Some(self.group),
            },
        ))
    }
}

/// Requests the privilige of proposing canister upgrades.
#[derive(Debug, Clone, Parser)]
pub struct RequestPermissionUpdatePermissionsArgs {
    /// A users that should be permitted to change permissions.  WARNING: Any user that is not listed will lose the ability to change permissions.
    #[structopt(long)]
    pub user: Vec<String>,
    /// A groups that should be permitted to change permissions.  WARNING: Any group that is not listed will lose the ability to change permissions.
    #[structopt(long)]
    pub group: Vec<String>,
}

impl From<RequestPermissionUpdatePermissionsArgs> for RequestOperationInput {
    fn from(value: RequestPermissionUpdatePermissionsArgs) -> Self {
        RequestOperationInput::EditPermission(EditPermissionOperationInput {
            resource: ResourceDTO::Permission(PermissionResourceActionDTO::Update),
            auth_scope: None,
            users: Some(value.user),
            user_groups: Some(value.group),
        })
    }
}

/// Requests the privilige of proposing canister upgrades.
#[derive(Debug, Clone, Parser)]
pub struct RequestPermissionReadPermissionsArgs {
    /// A users that should be permitted to change permissions.  WARNING: Any user that is not listed will lose the ability to change permissions.
    #[structopt(long)]
    pub user: Vec<String>,
    /// A groups that should be permitted to change permissions.  WARNING: Any group that is not listed will lose the ability to change permissions.
    #[structopt(long)]
    pub group: Vec<String>,
}

impl From<RequestPermissionReadPermissionsArgs> for RequestOperationInput {
    fn from(value: RequestPermissionReadPermissionsArgs) -> Self {
        RequestOperationInput::EditPermission(EditPermissionOperationInput {
            resource: ResourceDTO::Permission(PermissionResourceActionDTO::Read),
            auth_scope: None,
            users: Some(value.user),
            user_groups: Some(value.group),
        })
    }
}
