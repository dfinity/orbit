//! Implements the dfx extension CLI commands for making requests about canisters.

use crate::args::request::canister::{Args, ChangeExternalCanister};

/// The main entry point for the `dfx orbit` CLI.
pub fn main(args: Args) -> anyhow::Result<()> {
    match args {
        Args::Change(change_args) => change(change_args),
    }
}

/// Makes an API call to chnage an external canister.
fn change(args: ChangeExternalCanister) -> anyhow::Result<()> {
    let _args = orbit_station_api::ChangeExternalCanisterOperationInput::from(args);

    todo!()
}
