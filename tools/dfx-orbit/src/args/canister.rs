//! dfx-orbit external canister management commands.

use candid::Principal;
use clap::{Parser, Subcommand};

/// Station management commands.
#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum Args {
    /// Puts a canister controlled by the user under Orbit control.
    Claim(Claim),
}

/// Puts a canister controlled by the user under Orbit control.
#[derive(Debug, Parser)]
pub struct Claim {
    /// The canister ID.
    #[structopt(long)]
    pub canister_id: Principal,
    /// Make Orbit the exclusive controller of the canister.
    #[clap(long, short, action)]
    pub exclusive: bool,
}