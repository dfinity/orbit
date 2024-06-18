//! Makes `EditPermission` requests to Orbit.
pub mod canister;

use clap::Subcommand;

/// Request permission.
#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum Args {
    /// Request changes to canister permissions.
    #[command(subcommand)]
    Canister(canister::Args),
}

impl From<Args> for orbit_station_api::RequestOperationInput {
    fn from(args: Args) -> Self {
        match args {
            Args::Canister(change_args) => change_args.try_into().expect("TODO: bubble this up"),
        }
    }
}
