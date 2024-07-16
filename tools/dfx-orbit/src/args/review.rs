//! Defines the command line arguments for `dfx-orbit review`.  These correspond to Orbit station `get_request`, `submit_request_approval` and related API calls.
pub mod id;
pub mod list;
pub mod next;

use clap::Subcommand;

/// Station management commands.
#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum Args {
    /// List requests
    List(list::ReviewListArgs),
    /// Review the next request.
    Next(next::ReviewNextArgs),
    /// Review a specific request.
    Id(id::ReviewIdArgs),
}
