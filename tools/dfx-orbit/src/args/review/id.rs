//! CLI arguments for `dfx-orbit review next`.
use clap::Parser;

/// Reviews the next request.
#[derive(Debug, Parser)]
pub struct ReviewIdArgs {
    /// The ID of the request to review.
    pub(crate) request_id: String,
}
