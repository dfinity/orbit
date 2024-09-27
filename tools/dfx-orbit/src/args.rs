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

    // TODO: Allow to specify --network, to overwrite the network specified by the station
    /// The user identity to run this command as
    #[clap(short, long)]
    pub(crate) identity: Option<String>,

    /// Manage Orbit stations.
    #[clap(subcommand)]
    pub(crate) command: DfxOrbitSubcommands,
}

impl std::fmt::Display for DfxOrbitArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "dfx-orbit")?;

        if let Some(station) = &self.station {
            write!(f, " --station {}", station)?;
        }
        if let Some(identity) = &self.identity {
            write!(f, " --identity {}", identity)?;
        }
        write!(f, " {}", self.command)?;
        Ok(())
    }
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

impl std::fmt::Display for DfxOrbitSubcommands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DfxOrbitSubcommands::Station(_args) => write!(f, "station [TODO]"),
            DfxOrbitSubcommands::Request(_args) => write!(f, "request [TODO]"),
            DfxOrbitSubcommands::Verify(args) => write!(f, "verify {}", args),
            DfxOrbitSubcommands::Review(_args) => write!(f, "review [TODO]"),
            DfxOrbitSubcommands::Me(args) => write!(f, " me {}", args),
        }
    }
}

#[derive(Debug, Clone, Parser)]
pub struct MeArgs {
    /// Return output as JSON
    #[clap(short, long)]
    pub(crate) json: bool,
}

impl std::fmt::Display for MeArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.json {
            write!(f, "--json")?;
        }
        Ok(())
    }
}
