//! Implements the dfx extension CLI commands for making requests.

use crate::args::request::permission::canister::Args;

/// The main entry point for the `dfx orbit` CLI.
pub async fn main(args: Args) -> anyhow::Result<()> {
    match args {
        Args::Change(_change_args) => todo!(),
    }
}
