//! Library for interacting with Orbit on the Internet Computer.
use clap::{Parser, Subcommand};

pub mod station;
use station::commands::StationArgs;

/// Manages Orbit on the Internet Computer.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct DfxOrbitArgs {
    /// Manage Orbit stations.
    #[command(subcommand)]
    field: DfxOrbitSubcommands,
}

#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum DfxOrbitSubcommands {
    #[command(subcommand)]
    Station(StationArgs),
}

pub fn main(args: DfxOrbitArgs) {
    println!("Hello args: {args:?}");
}
