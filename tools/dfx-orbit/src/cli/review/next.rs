//! Implements `dfx review next` command.  These correspond to Orbit station `get_next_approvable_request` API call.

use crate::args::review::next::Args;

/// The main entry point for the `dfx orbit review next` CLI.
pub async fn exec(_args: Args) -> anyhow::Result<()> {
    unimplemented!("Review next request")
}
