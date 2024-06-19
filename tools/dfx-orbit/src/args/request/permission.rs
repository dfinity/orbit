//! Makes `EditPermission` requests to Orbit.
pub mod canister;

use clap::Subcommand;

use crate::orbit_station_agent::StationAgent;

use super::CreateRequestArgs;

/// Request permission.
#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum Args {
    /// Request changes to canister permissions.
    #[command(subcommand)]
    Canister(canister::Args),
}

impl CreateRequestArgs for Args {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    fn into_create_request_input(
        self,
        station_agent: &StationAgent,
    ) -> anyhow::Result<orbit_station_api::CreateRequestInput> {
        match self {
            Args::Canister(canister_args) => canister_args.into_create_request_input(station_agent),
        }
    }
}
