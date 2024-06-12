//! Implements the dfx extension CLI commands for managing external canisters.
mod claim;

use crate::args::canister::Args;

/// The main entry point for the `dfx orbit` CLI.
pub fn main(args: Args) -> anyhow::Result<()> {
    match args {
        Args::Claim(claim_args) => claim::main(claim_args),
    }
}
