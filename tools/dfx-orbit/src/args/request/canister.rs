//! CLI arguments for `dfx-orbit canister`.

pub mod call;
pub mod change;

use super::CreateRequestArgs;
use crate::orbit_station_agent::StationAgent;
use clap::Subcommand;

/// Request canister changes.
#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum Args {
    /// Request to update the canister.
    Change(change::Args),
    /// Request to call a canister method.
    Call(call::Args),
}

impl CreateRequestArgs for Args {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    fn into_create_request_input(
        self,
        station_agent: &StationAgent,
    ) -> anyhow::Result<orbit_station_api::CreateRequestInput> {
        match self {
            Args::Change(change_args) => change_args.into_create_request_input(station_agent),
            Args::Call(call_args) => call_args.into_create_request_input(station_agent),
        }
    }
}
