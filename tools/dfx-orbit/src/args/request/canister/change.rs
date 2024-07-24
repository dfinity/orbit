//! Arguments for `dfx orbit canister change`.
pub mod wasm;

use crate::StationAgent;
use clap::Subcommand;
use orbit_station_api::RequestOperationInput;
use wasm::RequestCanisterChangeWasmArgs;

/// Request permission.
#[derive(Debug, Clone, Subcommand)]
#[command(version, about, long_about = None)]
pub enum RequestCanisterChangeArgs {
    /// Request changes to the canister Wasm.
    Wasm(RequestCanisterChangeWasmArgs),
}

impl RequestCanisterChangeArgs {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    pub(crate) fn into_create_request_input(
        self,
        station_agent: &StationAgent,
    ) -> anyhow::Result<RequestOperationInput> {
        match self {
            RequestCanisterChangeArgs::Wasm(wasm_args) => {
                wasm_args.into_create_request_input(station_agent)
            }
        }
    }
}
