//! Makes `EditPermission` requests to Orbit.
pub mod canister;

use clap::Subcommand;

/// Request permission.
#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum Args {
    /// Request changes to canister permissions.
    #[command(subcommand)]
    Canister(canister::Args),
}
