//! Command line interface for `dfx-orbit`.
pub mod assets;
pub mod canister;
pub mod request;
pub mod review;
pub mod station;

use assets::AssetArgs;
use canister::CanisterArgs;
use clap::{Parser, Subcommand};
use request::RequestArgs;
use review::ReviewArgs;
use station::StationArgs;

/// Manages Orbit on the Internet Computer.
// TODO: Specify --station to not use the default station
// TODO: Better version information
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
    /// Manage Orbit stations.
    #[command(subcommand)]
    Station(StationArgs),
    /// Manage external canisters with Orbit.
    #[command(subcommand)]
    Canister(CanisterArgs),
    /// Make requests to Orbit
    Request(RequestArgs),
    /// View and decide on requests.
    #[command(subcommand)]
    Review(ReviewArgs),
    /// Manage assets stored in an asset canister through Orbit
    Asset(AssetArgs),
    /// Gets the caller's profile on an Orbit station.
    Me,
}
