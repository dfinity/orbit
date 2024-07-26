//! dfx-orbit external canister management commands.

use clap::{Parser, Subcommand};
use std::fmt::Debug;

/// Station management commands.
#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum CanisterArgs {
    /// Puts a canister controlled by the user under Orbit control.
    Claim(CanisterClaimArgs),
}

/// Puts a canister controlled by the user under Orbit control.
#[derive(Debug, Parser)]
pub struct CanisterClaimArgs {
    /// The canister name or `canister_id`.
    pub canister: String,
    /// Make Orbit the exclusive controller of the canister.
    #[clap(long, short, action)]
    pub exclusive: bool,
}
