//! Implements the dfx extension CLI commands for making requests.

//use orbit_station_api::RequestOperationInput;

use crate::args::request::permission::canister::{Args, ChangeCanister};

/// The main entry point for the `dfx orbit` CLI.
pub async fn main(
    args: Args,
) -> anyhow::Result<Result<orbit_station_api::CreateRequestResponse, orbit_station_api::ApiErrorDTO>>
{
    match args {
        Args::Change(change_args) => main_change(change_args).await,
    }
}

/// Change permissions for a user to interact with a canister.
async fn main_change(
    _args: ChangeCanister,
) -> anyhow::Result<Result<orbit_station_api::CreateRequestResponse, orbit_station_api::ApiErrorDTO>>
{
    //let _args = RequestOperationInput::ChangeExternalCanister(args);
    // TODO: Add title, summary and execution_plan to the CLI.
    //    let args = CreateRequestInput {
    //        operation: args,
    //      title: None,
    //    summary: None,
    //  execution_plan: None,
    //};
    todo!()
}
