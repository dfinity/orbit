//! Makes `EditPermission` requests to Orbit.
pub mod canister;
#[allow(clippy::module_inception)]
pub mod permission;

use crate::DfxOrbit;
use canister::RequestPermissionUpgradeCanisterArgs;
use clap::Subcommand;
use orbit_station_api::RequestOperationInput;
use permission::{RequestPermissionReadPermissionsArgs, RequestPermissionUpdatePermissionsArgs};

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
