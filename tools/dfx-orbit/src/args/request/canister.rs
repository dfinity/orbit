//! CLI arguments for `dfx-orbit canister`.

pub mod call;
pub mod change;

use crate::StationAgent;
use call::RequestCanisterCallArgs;
use change::RequestCanisterChangeArgs;
use clap::Subcommand;
use orbit_station_api::RequestOperationInput;

/// Request canister changes.
#[derive(Debug, Clone, Subcommand)]
#[command(version, about, long_about = None)]
pub enum RequestCanisterArgs {
    /// Request to update the canister.
    #[command(subcommand)]
    Change(RequestCanisterChangeArgs),
    /// Request to call a canister method.
    Call(RequestCanisterCallArgs),
}

impl RequestCanisterArgs {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    pub(crate) fn into_create_request_input(
        self,
        station_agent: &StationAgent,
    ) -> anyhow::Result<RequestOperationInput> {
        match self {
            RequestCanisterArgs::Change(change_args) => {
                change_args.into_create_request_input(station_agent)
            }
            RequestCanisterArgs::Call(call_args) => {
                call_args.into_create_request_input(station_agent)
            }
        }
    }
}
