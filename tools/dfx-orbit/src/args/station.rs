//! dfx-orbit station management commands.
use std::fmt::{self, Display, Formatter};

use candid::Principal;
use clap::{Parser, Subcommand};

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
    #[structopt(long)]
    pub name: String,
    /// Wallet canister ID.
    #[structopt(long)]
    pub canister_id: Principal,
    /// The dfx network name.
    #[structopt(long)]
    pub network: String,
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
    #[structopt(long)]
    pub name: String,
}
