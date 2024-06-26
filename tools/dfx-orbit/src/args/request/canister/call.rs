//! CLI arguments for `dfx-orbit canister call`.

use clap::Parser;

use crate::{args::request::CreateRequestArgs, orbit_station_agent::StationAgent};

/// Requests that a call be made to a canister.
#[derive(Debug, Parser)]
pub struct Args {}

impl CreateRequestArgs for Args {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    fn into_create_request_input(
        self,
        _station_agent: &StationAgent,
    ) -> anyhow::Result<orbit_station_api::CreateRequestInput> {
        todo!("Need to convert CLI args into Orbit API args for canister call.")
    }
}