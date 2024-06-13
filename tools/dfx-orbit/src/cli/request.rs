//! Implements the dfx extension CLI commands for making requests.

pub mod canister;

use crate::args::request::Args;

/// The main entry point for the `dfx orbit` CLI.
pub fn main(args: Args) -> anyhow::Result<()> {
    match args {
        Args::Canister(canister_args) => canister::main(canister_args),
    }
}
