//! Command line interface for `dfx-orbit`.
pub mod asset;
pub mod request;
pub mod review;
pub mod station;

use asset::AssetArgs;
use clap::{Parser, Subcommand};
use request::RequestArgs;
use review::ReviewArgs;
use station::StationArgs;

/// Manages Orbit on the Internet Computer.
// TODO: Specify --station to not use the default station
// TODO: Better version information
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct DfxOrbitArgs {
    /// Increase verbosity level
    #[clap(short, long, action = clap::ArgAction::Count, conflicts_with = "quiet")]
    pub(crate) verbose: u8,

    /// Reduce verbosity level
    #[clap(short, long, action = clap::ArgAction::Count, conflicts_with = "verbose")]
    pub(crate) quiet: u8,

    /// Name of the station to execute the command on. (Uses default station if unspecified)
    #[clap(short, long)]
    pub(crate) station: Option<String>,

    /// Manage Orbit stations.
    #[clap(subcommand)]
    pub(crate) command: DfxOrbitSubcommands,
}

/// CLI commands for managing Orbit on the Internet Computer.
#[derive(Debug, Subcommand)]
#[clap(version, about, long_about = None)]
pub enum DfxOrbitSubcommands {
    /// Manage Orbit stations.
    #[clap(subcommand)]
    Station(StationArgs),
    /// Make requests to Orbit
    Request(RequestArgs),
    /// View and decide on requests.
    #[clap(subcommand)]
    Review(ReviewArgs),
    /// Manage assets stored in an asset canister through Orbit
    Asset(AssetArgs),
    /// Gets the caller's profile on an Orbit station.
    Me,
}
