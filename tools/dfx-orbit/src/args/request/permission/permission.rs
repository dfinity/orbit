//! Arguments for `dfx-orbit request permission permission`.
pub mod read;
pub mod update;

use crate::{args::request::CreateRequestArgs, StationAgent};
use clap::Subcommand;
use orbit_station_api::CreateRequestInput;

/// Request canister changes.
///
// TODO: Add flags for --title, --summary, and --execution-plan.
// Note: I have looked at the docs and the anwer for how to do this really doesn't jump out at me.  Google foo failed as well.  Maybe the sdk repo has some examples.
#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum Args {
    /// Request permission to update permissions.
    Update(update::Args),
    /// Request permission to read permissions.
    Read(read::Args),
}

impl CreateRequestArgs for Args {
    fn into_create_request_input(
        self,
        station_agent: &StationAgent,
    ) -> anyhow::Result<CreateRequestInput> {
        match self {
            Args::Read(read_args) => read_args.into_create_request_input(station_agent),
            Args::Update(update_args) => update_args.into_create_request_input(station_agent),
        }
    }
}
