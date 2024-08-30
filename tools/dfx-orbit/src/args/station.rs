//! dfx-orbit station management commands
use crate::station_agent::StationConfig;
use candid::Principal;
use clap::{Parser, Subcommand};
use std::fmt::{self, Display, Formatter};

/// Station management commands
#[derive(Debug, Subcommand)]
#[clap(version, about, long_about = None)]
pub enum StationArgs {
    /// Adds an Orbit station to the local dfx configuration
    Add(StationAddArgs),
    /// Lists Orbit stations in the local dfx configuration
    List(StationListArgs),
    /// Print the name of the current default station
    Default,
    /// Sets the default station in the local dfx configuration
    Use(StationUseArgs),
    /// Shows the local configuration for an Orbit station
    Show(StationShowArgs),
    /// Edit the orbit station in the local dfx configuration
    Edit(StationEditArgs),
    /// Removes an Orbit station from the local dfx configuration
    Remove(StationRemoveArgs),
}

/// Adds an Orbit station to the local dfx configuration
#[derive(Debug, Parser)]
pub struct StationAddArgs {
    /// Wallet name
    pub(crate) name: String,
    /// Station canister ID, called "Wallet ID" in the Orbit UI
    #[clap(short, long)]
    pub(crate) station_id: Principal,
    /// The dfx network name
    #[clap(short, long)]
    pub(crate) network: String,
    /// The URL pointing to the Orbit Web UI ()
    #[clap(short, long)]
    // TODO: Allow to default this to --url https://orbitwallet.io
    pub(crate) url: String,
}

impl From<StationAddArgs> for StationConfig {
    fn from(add: StationAddArgs) -> Self {
        Self {
            name: add.name,
            station_id: add.station_id,
            network: add.network,
            url: add.url,
        }
    }
}

/// Lists Orbit station in the local dfx configuration.
#[derive(Debug, Parser)]
pub struct StationListArgs {}

/// Response to a List command.
#[derive(Debug, serde::Serialize)]
pub struct ListResponse {
    /// List of station names.
    pub(crate) stations: Vec<String>,
}
impl Display for ListResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for station in &self.stations {
            writeln!(f, "{station}")?;
        }
        Ok(())
    }
}

/// Shows the local configuration for an Orbit station.
#[derive(Debug, Parser)]
pub struct StationShowArgs {
    /// Station name
    #[clap(short, long)]
    pub(crate) name: Option<String>,
    /// Return output as JSON
    #[clap(short, long)]
    pub(crate) json: bool,
}

/// Renames an Orbit station in the local dfx configuration
#[derive(Debug, Parser)]
pub struct StationEditArgs {
    /// The name of the station to edit (default if ommitted)
    pub(crate) station: Option<String>,
    /// Change the station name
    #[clap(long)]
    pub(crate) rename: Option<String>,
    /// Change the station id
    #[clap(short, long)]
    pub(crate) station_id: Option<Principal>,
    /// Change the network of the station
    #[clap(short, long)]
    pub(crate) network: Option<String>,
    #[clap(short, long)]
    pub(crate) url: Option<String>,
}

/// Removes an Orbit station from the local dfx configuration
#[derive(Debug, Parser)]
pub struct StationRemoveArgs {
    /// Station name
    #[clap(long)]
    pub name: String,
}

/// Sets the default station in the local dfx configuration
#[derive(Debug, Parser)]
pub struct StationUseArgs {
    /// Station name
    pub name: String,
}
