//! dfx-orbit station management commands.
use std::fmt::{self, Display, Formatter};

use candid::Principal;
use clap::{Parser, Subcommand};

use crate::local_config::StationConfig;

/// Station management commands.
#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum StationArgs {
    /// Adds an Orbit station to the local dfx configuration.
    Add(Add),
    /// Lists Orbit stations in the local dfx configuration.
    List(List),
    /// The default station.
    Default,
    /// Sets the default station in the local dfx configuration.
    Use(Use),
    /// Shows the local configuration for an Orbit station.
    Show(Show),
    /// Renames an Orbit station in the local dfx configuration.
    Rename(Rename),
    /// Removes an Orbit station from the local dfx configuration.
    Remove(Remove),
}

/// Adds an Orbit station to the local dfx configuration.
#[derive(Debug, Parser)]
pub struct Add {
    /// Wallet name.
    pub name: String,
    /// Station canister ID, called "Wallet ID" in the Orbit UI.
    #[structopt(long)]
    pub station_id: Principal,
    /// The dfx network name.
    #[structopt(long)]
    pub network: String,
    /// The Obit user interface URL.
    #[structopt(long)]
    pub url: String,
}

impl From<Add> for StationConfig {
    fn from(add: Add) -> Self {
        Self {
            name: add.name,
            station_id: add.station_id.to_text(),
            network: add.network,
            url: add.url,
        }
    }
}

/// Lists Orbit station in the local dfx configuration.
#[derive(Debug, Parser)]
pub struct List {}

/// Response to a List command.
#[derive(Debug, serde::Serialize)]
pub struct ListResponse {
    /// List of station names.
    pub stations: Vec<String>,
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
pub struct Show {
    /// Station name.
    #[structopt(long)]
    pub name: Option<String>,
}

/// Renames an Orbit station in the local dfx configuration.
#[derive(Debug, Parser)]
pub struct Rename {
    /// Station name.
    #[structopt(long)]
    pub old: String,
    /// New station name.
    #[structopt(long)]
    pub new: String,
}

/// Removes an Orbit station from the local dfx configuration.
#[derive(Debug, Parser)]
pub struct Remove {
    /// Station name.
    #[structopt(long)]
    pub name: String,
}

/// Sets the default station in the local dfx configuration.
#[derive(Debug, Parser)]
pub struct Use {
    /// Station name.
    pub name: String,
}
