//! Implements the dfx extension CLI commands for making requests.

use crate::args::request::permission::canister::Args;

/// The main entry point for the `dfx orbit` CLI.
pub async fn main(
    args: Args,
) -> anyhow::Result<Result<orbit_station_api::CreateRequestResponse, orbit_station_api::ApiErrorDTO>>
{
    match args {
        Args::Change(_change_args) => main_change().await,
    }
}

/// Change permissions for a user to interact with a canister.
async fn main_change(
) -> anyhow::Result<Result<orbit_station_api::CreateRequestResponse, orbit_station_api::ApiErrorDTO>>
{
    // TODO: Add title, summary and execution_plan to the CLI.
    //    let args = CreateRequestInput {
    //        operation: args,
    //      title: None,
    //    summary: None,
    //  execution_plan: None,
    //};
    todo!()
}
