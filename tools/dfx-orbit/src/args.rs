//! Command line interface for `dfx-orbit`.
pub mod request;
pub mod review;
pub mod station;
pub mod verify;

use clap::{Parser, Subcommand};
use request::RequestArgs;
use review::ReviewArgs;
use station::StationArgs;
use verify::VerifyArgs;

/// Manages Orbit on the Internet Computer.
#[derive(Parser, Debug)]
#[clap(version, about)]
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
    /// Verify requests
    Verify(VerifyArgs),
    /// View and decide on requests.
    Review(ReviewArgs),
    /// Gets the caller's profile on an Orbit station.
    Me(MeArgs),
}

#[derive(Debug, Clone, Parser)]
pub struct MeArgs {
    /// Return output as JSON
    #[clap(short, long)]
    pub(crate) json: bool,
}
