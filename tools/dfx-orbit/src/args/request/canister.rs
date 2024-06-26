//! Makes requests to do things to canisters.  Such as update the Wasm, deploy frontend assets or make API calls to them.

pub mod change;

use super::CreateRequestArgs;
use crate::orbit_station_agent::StationAgent;
use clap::Subcommand;

/// Request canister changes.
#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum Args {
    /// Request to update the canister.
    Change(change::ChangeExternalCanister),
}

impl CreateRequestArgs for Args {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    fn into_create_request_input(
        self,
        station_agent: &StationAgent,
    ) -> anyhow::Result<orbit_station_api::CreateRequestInput> {
        match self {
            Args::Change(change_args) => change_args.into_create_request_input(station_agent),
        }
    }
}
