//! Command line interface for `dfx-orbit`.
pub mod canister;
pub mod dfx_extension_api;
pub mod request;
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

/// CLI commands for managing Orbit on the Internet Computer.
#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum DfxOrbitSubcommands {
    /// Manages Orbit stations.
    #[command(subcommand)]
    Station(StationArgs),
    /// Manages external canisters with Orbit.
    #[command(subcommand)]
    Canister(canister::Args),
    /// Make requests to Orbit
    #[command(subcommand)]
    Request(request::Args),
    /// Exercises the experimental DFX extension API.
    ///
    /// As the API is brand new and prototypical, this is exposed as a subcommand.  Once stable it can be removed.
    #[command(subcommand)]
    DfxExtension(dfx_extension_api::Args),
    /// Gets the caller's profile on an Orbit station.
    Me,
}
