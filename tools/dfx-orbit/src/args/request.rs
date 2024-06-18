//! Makes requests to Orbit.
pub mod canister;
pub mod permission;

use clap::Subcommand;

/// Request canister changes.
#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum Args {
    /// Request changes to a canister.
    #[command(subcommand)]
    Canister(canister::Args),
    /// Request changes to a canister.
    #[command(subcommand)]
    Permission(permission::Args),
}
