//! Defines the command line arguments for `dfx-orbit request`.  These correspond to Orbit station `create_request` API calls.
pub mod canister;
pub mod permission;

use clap::Subcommand;
use orbit_station_api::{CreateRequestInput, RequestOperationInput};

use crate::orbit_station_agent::StationAgent;

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

/// Converts the CLI arg type into the equivalent Orbit API type.
pub trait CreateRequestArgs {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    fn into_create_request_input(
        self,
        station_agent: &StationAgent,
    ) -> anyhow::Result<CreateRequestInput>;
}

impl CreateRequestArgs for Args {
    fn into_create_request_input(
        self,
        station_agent: &StationAgent,
    ) -> anyhow::Result<CreateRequestInput> {
        match self {
            Args::Canister(canister_args) => canister_args.into_create_request_input(station_agent),
            Args::Permission(permission_args) => {
                permission_args.into_create_request_input(station_agent)
            }
        }
    }
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
