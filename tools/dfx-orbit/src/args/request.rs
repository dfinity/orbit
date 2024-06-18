//! Makes requests to Orbit.
pub mod canister;
pub mod permission;

use clap::Subcommand;
use orbit_station_api::RequestOperationInput;

/// Request canister changes.
#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum Args {
    /// Request changes to a canister.
    #[command(subcommand)]
    Canister(canister::Args),
    /// Request changes to a canister.
    #[command(subcommand)]
    Permission(permission::Args),
}

impl From<Args> for RequestOperationInput {
    fn from(args: Args) -> Self {
        match args {
            Args::Canister(canister_args) => RequestOperationInput::from(canister_args),
            Args::Permission(_permission_args) => unimplemented!(), //RequestOperationInput::from(permission_args),
        }
    }
}
