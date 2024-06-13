//! Makes requests to Orbit.
pub mod canister;

use clap::Subcommand;

/// Request canister changes.
#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum Args {
    /// Request changes to a canister.
    #[command(subcommand)]
    Canister(canister::Args),
}
