//! CLI arguments for `dfx-orbit canister`.

pub mod call;
pub mod change;

use crate::StationAgent;
use call::RequestCanisterCallArgs;
use change::RequestCanisterInstallArgs;
use clap::{Parser, Subcommand};
use orbit_station_api::RequestOperationInput;

/// Request canister operations through Orbit
#[derive(Debug, Clone, Parser)]
pub struct RequestCanisterArgs {
    /// The operation to request
    #[command(subcommand)]
    action: RequestCanisterActionArgs,
}

#[derive(Debug, Clone, Subcommand)]
#[command(version, about, long_about = None)]
pub enum RequestCanisterActionArgs {
    /// Request to upgrade the canister wasm
    Install(RequestCanisterInstallArgs),
    /// Request to call a canister method
    Call(RequestCanisterCallArgs),
}

impl RequestCanisterArgs {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    pub(crate) fn into_create_request_input(
        self,
        station_agent: &StationAgent,
    ) -> anyhow::Result<RequestOperationInput> {
        self.action.into_create_request_input(station_agent)
    }
}

impl RequestCanisterActionArgs {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    pub(crate) fn into_create_request_input(
        self,
        station_agent: &StationAgent,
    ) -> anyhow::Result<RequestOperationInput> {
        match self {
            RequestCanisterActionArgs::Install(change_args) => {
                change_args.into_create_request_input(station_agent)
            }
            RequestCanisterActionArgs::Call(call_args) => {
                call_args.into_create_request_input(station_agent)
            }
        }
    }
}
