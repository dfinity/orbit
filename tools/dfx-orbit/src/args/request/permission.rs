//! Makes `EditPermission` requests to Orbit.
pub mod canister;
#[allow(clippy::module_inception)]
pub mod permission;

use super::CreateRequestArgs;
use crate::StationAgent;
use canister::RequestPermissionUpdateCanisterArgs;
use clap::Subcommand;

/// Request permission.
#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum RequestPermissionArgs {
    // TODO: Move these up a level to reduce nesting
    /// Request permission to create requests.
    #[command(subcommand)]
    Permission(permission::Args),
    /// Request permission to update canister(s)
    UpdateCanister(RequestPermissionUpdateCanisterArgs),
}

impl CreateRequestArgs for RequestPermissionArgs {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    fn into_create_request_input(
        self,
        station_agent: &StationAgent,
    ) -> anyhow::Result<orbit_station_api::CreateRequestInput> {
        match self {
            RequestPermissionArgs::UpdateCanister(canister_args) => {
                canister_args.into_create_request_input(station_agent)
            }
            RequestPermissionArgs::Permission(permission_args) => {
                permission_args.into_create_request_input(station_agent)
            }
        }
    }
}
