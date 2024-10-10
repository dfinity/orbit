//! A dfx and IC agent for communicating with an Orbit station.

mod agent;
mod config;
mod error;

use crate::dfx::OrbitExtensionAgent;
use anyhow::Context;
use candid::Principal;
use clap::{Parser, Subcommand};
use std::fmt::{self, Display, Formatter};

pub use self::agent::{StationAgent, StationAgentResult, StationConfig};

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
    /// The URL pointing to the Orbit Web UI (defaults to "https://orbitwallet.io")
    #[clap(short, long)]
    pub(crate) url: Option<String>,
}

impl From<StationAddArgs> for StationConfig {
    fn from(add: StationAddArgs) -> Self {
        Self {
            name: add.name,
            station_id: add.station_id,
            network: add.network,
            url: add.url.unwrap_or(String::from("https://orbitwallet.io")),
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

impl StationArgs {
    /// Implements CLI commands for managing Orbit stations.
    pub(crate) fn execute(self, orbit_agent: OrbitExtensionAgent) -> anyhow::Result<()> {
        match self {
            StationArgs::Add(add_args) => {
                orbit_agent
                    .add_station(add_args)
                    .with_context(|| "Failed to add station to local dfx config")?;
            }
            StationArgs::List(_list_args) => {
                let stations = orbit_agent.list_stations()?;
                let ans = ListResponse { stations };
                // Note: The formatted ans is a sequence of complete lines, so an additional newline, as provided by println, is not needed.
                print!("{ans}");
            }
            StationArgs::Default => {
                let default_station = orbit_agent
                    .default_station_name()
                    .with_context(|| "Failed to get default station from local dfx config")?;
                if let Some(station) = default_station {
                    println!("{station}");
                }
            }
            StationArgs::Use(use_args) => {
                orbit_agent
                    .set_default_station(Some(use_args.name))
                    .with_context(|| "Failed to set default station in local dfx config")?;
            }
            StationArgs::Show(show_args) => {
                let station = orbit_agent
                    .station_or_default(&show_args.name)
                    .with_context(|| "Failed to get station from local dfx config")?;
                if show_args.json {
                    let json = serde_json::to_string_pretty(&station)
                        .with_context(|| "Failed to serialize station")?;
                    println!("{json}");
                } else {
                    println!("{}", station);
                }
            }
            StationArgs::Remove(remove_args) => {
                orbit_agent
                    .remove_station(&remove_args.name)
                    .with_context(|| "Failed to remove station from local dfx config")?;
            }
            StationArgs::Edit(rename_args) => {
                orbit_agent
                    .edit_station(
                        &rename_args.station,
                        rename_args.rename,
                        rename_args.station_id,
                        rename_args.network,
                        rename_args.url,
                    )
                    .with_context(|| "Failed to rename station in local dfx config")?;
            }
        }
        Ok(())
    }
}
