//! Command line interface for `dfx-orbit`.
pub mod station;

use clap::{Parser, Subcommand};
use station::StationArgs;

/// Manages Orbit on the Internet Computer.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct DfxOrbitArgs {
    /// Manage Orbit stations.
    #[command(subcommand)]
    field: DfxOrbitSubcommands,
}

#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum DfxOrbitSubcommands {
    #[command(subcommand)]
    Station(StationArgs),
}
