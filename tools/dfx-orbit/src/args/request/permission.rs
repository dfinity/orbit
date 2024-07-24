//! Makes `EditPermission` requests to Orbit.
pub mod canister;
#[allow(clippy::module_inception)]
pub mod permission;

use super::CreateRequestArgs;
use crate::StationAgent;
use canister::RequestPermissionUpdateCanisterArgs;
use clap::Subcommand;
use orbit_station_api::CreateRequestInput;
use permission::{RequestPermissionReadPermissionsArgs, RequestPermissionUpdatePermissionsArgs};

/// Request permission.
#[derive(Debug, Clone, Subcommand)]
#[command(version, about, long_about = None)]
pub enum RequestPermissionArgs {
    /// Request permission to read permission(s)
    ReadPermissions(RequestPermissionReadPermissionsArgs),
    /// Request permission to update permission(s)
    UpdatePermissions(RequestPermissionUpdatePermissionsArgs),
    /// Request permission to update canister(s)
    UpdateCanister(RequestPermissionUpdateCanisterArgs),
}

impl CreateRequestArgs for RequestPermissionArgs {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    fn into_create_request_input(
        self,
        station_agent: &StationAgent,
    ) -> anyhow::Result<orbit_station_api::CreateRequestInput> {
        let operation = match self {
            RequestPermissionArgs::UpdateCanister(canister_args) => {
                canister_args.into_create_request_input(station_agent)?
            }
            RequestPermissionArgs::ReadPermissions(permission_args) => permission_args.into(),
            RequestPermissionArgs::UpdatePermissions(permission_args) => permission_args.into(),
        };

        Ok(CreateRequestInput {
            operation,
            title: None,
            summary: None,
            execution_plan: None,
        })
    }
}
