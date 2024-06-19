//! Implements the `dfx-orbit canister *` CLI commands.
mod claim;

use crate::args::canister::Args;

/// The main entry point for the `dfx orbit` CLI.
pub fn exec(args: Args) -> anyhow::Result<()> {
    match args {
        Args::Claim(claim_args) => claim::exec(claim_args),
    }
}
