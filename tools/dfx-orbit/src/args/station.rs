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
    name: String,
    /// Wallet canister ID.
    #[structopt(long)]
    canister_id: Principal,
}

/// Lists Orbit station in the local dfx configuration.
#[derive(Debug, Parser)]
pub struct List {}

/// Renames an Orbit station in the local dfx configuration.
#[derive(Debug, Parser)]
pub struct Rename {
    /// Station name.
    #[structopt(long)]
    name: String,
    /// New station name.
    #[structopt(long)]
    new_name: String,
}

/// Removes an Orbit station from the local dfx configuration.
#[derive(Debug, Parser)]
pub struct Remove {
    /// Station name.
    #[structopt(long)]
    name: String,
}
