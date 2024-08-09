//! Defines the command line arguments for `dfx-orbit request`.  These correspond to Orbit station `create_request` API calls.
pub mod canister;
pub mod permission;

use crate::DfxOrbit;
use canister::RequestCanisterArgs;
use clap::{Parser, Subcommand};
use orbit_station_api::CreateRequestInput;
use permission::RequestPermissionArgs;

/// Request canister changes.
#[derive(Debug, Clone, Parser)]
#[command(version, about, long_about = None)]
pub struct RequestArgs {
    /// Title of the request
    #[clap(long)]
    title: Option<String>,

    /// Summary of the request
    #[clap(long)]
    summary: Option<String>,

    // TODO: Summary file as an alternative to summary
    // TODO: Execution plan
    #[command(subcommand)]
    action: RequestArgsActions,
}

#[derive(Debug, Clone, Subcommand)]
#[command(version, about, long_about = None)]
pub enum RequestArgsActions {
    /// Request canister operations through Orbit
    Canister(RequestCanisterArgs),
    /// Request permissions
    #[command(subcommand)]
    Permission(RequestPermissionArgs),
}

impl RequestArgs {
    pub(crate) fn into_create_request_input(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<CreateRequestInput> {
        let operation = match self.action {
            RequestArgsActions::Canister(canister_args) => {
                canister_args.into_create_request_input(dfx_orbit)?
            }
            RequestArgsActions::Permission(permission_args) => {
                permission_args.into_create_request_input(dfx_orbit)?
            }
        };

        Ok(CreateRequestInput {
            operation,
            title: self.title,
            summary: self.summary,
            execution_plan: None,
        })
    }
}
