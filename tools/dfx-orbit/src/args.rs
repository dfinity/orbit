//! Command line interface for `dfx-orbit`.
pub mod dfx_extension_api;
pub mod station;

use clap::{Parser, Subcommand};
use station::StationArgs;

/// Manages Orbit on the Internet Computer.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct DfxOrbitArgs {
    /// Manage Orbit stations.
    #[command(subcommand)]
    pub command: DfxOrbitSubcommands,
}

#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum DfxOrbitSubcommands {
    #[command(subcommand)]
    Station(StationArgs),
    #[command(subcommand)]
    DfxExtension(dfx_extension_api::Args),
}
