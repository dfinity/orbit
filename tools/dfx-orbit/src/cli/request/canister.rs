//! Implements the dfx extension CLI commands for making requests about canisters.

use crate::args::request::canister::Args;

/// The main entry point for the `dfx orbit` CLI.
pub fn main(args: Args) -> anyhow::Result<()> {
    match args {
        Args::Change(_change_args) => todo!(), //change(change_args),
    }
}

/*
fn change(args: ChangeExternalCanister) -> anyhow::Result<()> {
    Change

    let Args::Change(change_args) = args;
    println!("Changing request: {:?}", change_args);
    Ok(())
}
    */
