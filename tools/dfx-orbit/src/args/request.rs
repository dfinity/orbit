//! Makes requests to Orbit.
pub mod canister;
pub mod permission;

use clap::Subcommand;
use orbit_station_api::{CreateRequestInput, RequestOperationInput};

/// Request canister changes.
///
// TODO: Add flags for --title, --summary, and --execution-plan.
// Note: I have looked at the docs and the anwer for how to do this really doesn't jump out at me.  Google foo failed as well.  Maybe the sdk repo has some examples.
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
            Args::Permission(permission_args) => RequestOperationInput::from(permission_args),
        }
    }
}

impl From<Args> for CreateRequestInput {
    fn from(args: Args) -> Self {
        let operation = RequestOperationInput::from(args);
        // TODO: Get title, summary, and execution_plan from args.
        CreateRequestInput {
            operation,
            title: None,
            summary: None,
            execution_plan: None,
        }
    }
}
