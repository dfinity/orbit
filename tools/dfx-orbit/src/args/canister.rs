//! dfx-orbit external canister management commands.

use clap::{Parser, Subcommand};
use std::fmt::Debug;

/// Station management commands.
#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum Args {
    /// Puts a canister controlled by the user under Orbit control.
    Claim(Claim),
    /// Uploads assets to an HTTP asset canister and requests that the assets be used.
    UploadHttpAssets(UploadHttpAssets),
}

/// Puts a canister controlled by the user under Orbit control.
#[derive(Debug, Parser)]
pub struct Claim {
    /// The canister name or `canister_id`.
    pub canister: String,
    /// Make Orbit the exclusive controller of the canister.
    #[clap(long, short, action)]
    pub exclusive: bool,
}

/// Uploads assets to an HTTP asset canister.
#[derive(Debug, Parser)]
pub struct UploadHttpAssets {
    /// The canister name or `canister_id`.
    #[structopt(long)]
    pub canister: String,
    /// A directory of assets to upload.
    #[structopt(long)]
    pub source: Vec<String>,
    /// Provide a running commentary.
    #[arg(short, long, default_value_t = false)]
    pub verbose: bool,
}
