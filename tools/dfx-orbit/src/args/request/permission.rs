//! Makes `EditPermission` requests to Orbit.
pub mod canister;
#[allow(clippy::module_inception)]
pub mod permission;

use super::CreateRequestArgs;
use crate::StationAgent;
use clap::Subcommand;

/// Request permission.
#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum RequestPermissionArgs {
    /// Request permission to create requests.
    #[command(subcommand)]
    Permission(permission::Args),
    /// Request changes to canister permissions.
    #[command(subcommand)]
    Canister(canister::Args),
}

impl CreateRequestArgs for RequestPermissionArgs {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    fn into_create_request_input(
        self,
        station_agent: &StationAgent,
    ) -> anyhow::Result<orbit_station_api::CreateRequestInput> {
        match self {
            RequestPermissionArgs::Canister(canister_args) => {
                canister_args.into_create_request_input(station_agent)
            }
            RequestPermissionArgs::Permission(permission_args) => {
                permission_args.into_create_request_input(station_agent)
            }
        }
    }
}
