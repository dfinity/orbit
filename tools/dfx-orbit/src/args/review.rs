//! Defines the command line arguments for `dfx-orbit review`.  These correspond to Orbit station `get_request`, `submit_request_approval` and related API calls.
pub mod id;
pub mod list;
pub mod next;

use clap::{Parser, Subcommand};
use id::ReviewIdArgs;
use list::ReviewListArgs;
use next::ReviewNextArgs;

/// Station management commands.
#[derive(Debug, Parser)]
pub struct ReviewArgs {
    /// Return output as JSON
    #[clap(short, long)]
    pub(crate) json: bool,

    #[clap(subcommand)]
    pub(crate) action: ReviewActionArgs,
}

#[derive(Debug, Subcommand)]
pub enum ReviewActionArgs {
    /// List requests
    List(ReviewListArgs),
    /// Review the next request.
    Next(ReviewNextArgs),
    /// Review a specific request.
    Id(ReviewIdArgs),
}
