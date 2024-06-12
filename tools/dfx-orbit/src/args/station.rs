//! dfx-orbit station management commands.
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
}

/// Lists Orbit station in the local dfx configuration.
#[derive(Debug, Parser)]
pub struct List {}

/// Shows the local configuration for an Orbit station.
#[derive(Debug, Parser)]
pub struct Show {
    /// Station name.
    #[structopt(long)]
    pub name: String,
}

/// Renames an Orbit station in the local dfx configuration.
#[derive(Debug, Parser)]
pub struct Rename {
    /// Station name.
    #[structopt(long)]
    pub name: String,
    /// New station name.
    #[structopt(long)]
    pub new_name: String,
}

/// Removes an Orbit station from the local dfx configuration.
#[derive(Debug, Parser)]
pub struct Remove {
    /// Station name.
    #[structopt(long)]
    pub name: String,
}
