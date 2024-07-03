//! Arguments for `dfx orbit canister change`.
pub mod wasm;

use clap::Subcommand;

use crate::{args::request::CreateRequestArgs, orbit_station_agent::StationAgent};

/// Request permission.
#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum Args {
    /// Request changes to the canister Wasm.
    Wasm(wasm::Args),
}

impl CreateRequestArgs for Args {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    fn into_create_request_input(
        self,
        station_agent: &StationAgent,
    ) -> anyhow::Result<orbit_station_api::CreateRequestInput> {
        match self {
            Args::Wasm(wasm_args) => wasm_args.into_create_request_input(station_agent),
        }
    }
}