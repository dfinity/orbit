//! Defines the command line arguments for `dfx-orbit request`.  These correspond to Orbit station `create_request` API calls.
pub mod canister;
pub mod permission;

use crate::StationAgent;
use canister::RequestCanisterArgs;
use clap::{Parser, Subcommand};
use orbit_station_api::CreateRequestInput;
use permission::RequestPermissionArgs;

/// Request canister changes.
///
// TODO: Add flags for --title, --summary, and --execution-plan.
// Note: I have looked at the docs and the anwer for how to do this really doesn't jump out at me.  Google foo failed as well.  Maybe the sdk repo has some examples.

#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct RequestArgs {
    #[command(subcommand)]
    action: RequestArgsActions,
}

#[derive(Debug, Clone, Subcommand)]
#[command(version, about, long_about = None)]
pub enum RequestArgsActions {
    /// Request changes to a canister.
    #[command(subcommand)]
    Canister(RequestCanisterArgs),
    /// Request permissions.
    #[command(subcommand)]
    Permission(RequestPermissionArgs),
}

/// Converts the CLI arg type into the equivalent Orbit API type.
pub trait CreateRequestArgs {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    fn into_create_request_input(
        self,
        station_agent: &StationAgent,
    ) -> anyhow::Result<CreateRequestInput>;
}

impl RequestArgs {
    pub(crate) fn into_create_request_input(
        self,
        station_agent: &StationAgent,
    ) -> anyhow::Result<CreateRequestInput> {
        let operation = match self.action {
            RequestArgsActions::Canister(canister_args) => {
                canister_args.into_create_request_input(station_agent)?
            }
            RequestArgsActions::Permission(permission_args) => {
                permission_args.into_create_request_input(station_agent)?
            }
        };

        Ok(CreateRequestInput {
            operation,
            title: None,
            summary: None,
            execution_plan: None,
        })
    }
}
