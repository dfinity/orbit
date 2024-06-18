//! Makes `EditPermission` requests regarding `ExternalCanister` to Orbit.
use clap::{Parser, Subcommand};

/// Request canister changes.
#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum Args {
    /// Request changes to canister permissions.
    Change(ChangeCanister),
}

/// Requests permissions to change a canister.
#[derive(Debug, Parser)]
pub struct ChangeCanister {
    /// Station name.
    #[structopt(long)]
    pub name: Option<String>,
}
