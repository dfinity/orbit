//! Implements `dfx review` commands.  These correspond to Orbit station `get_request`, approve and related API calls.
pub mod next;

use crate::args::review::Args;

/// The main entry point for the `dfx orbit review` CLI.
pub async fn exec(args: Args) -> anyhow::Result<()> {
    match args {
        Args::Next(next_args) => next::exec(next_args).await,
    }
}
