//! Library for interacting with Orbit on the Internet Computer.
use clap::Parser;

pub mod station;
use station::StationArgs;

/// Manages Orbit on the Internet Computer.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct DfxOrbitArgs {
    /// Manage Orbit stations.
    #[command(subcommand)]
    field: StationArgs,
}

pub fn main(args: DfxOrbitArgs) {
    println!("Hello args: {args:?}");
}
