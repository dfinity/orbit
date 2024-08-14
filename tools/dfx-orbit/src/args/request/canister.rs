//! CLI arguments for `dfx-orbit canister`.

pub mod call;
pub mod install;

use crate::DfxOrbit;
use call::RequestCanisterCallArgs;
use clap::{Parser, Subcommand};
use install::RequestCanisterInstallArgs;
use orbit_station_api::RequestOperationInput;

// TODO: Support Canister create + integration test
// TODO: Support Canister install check
// TODO: Canister get response functionality

/// Request canister operations through Orbit
#[derive(Debug, Clone, Parser)]
pub struct RequestCanisterArgs {
    /// The operation to request
    #[clap(subcommand)]
    action: RequestCanisterActionArgs,
}

#[derive(Debug, Clone, Subcommand)]
#[clap(version, about, long_about = None)]
pub enum RequestCanisterActionArgs {
    /// Request to upgrade the canister wasm
    Install(RequestCanisterInstallArgs),
    /// Request to call a canister method
    Call(RequestCanisterCallArgs),
}

impl RequestCanisterArgs {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    pub(crate) fn into_create_request_input(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        self.action.into_create_request_input(dfx_orbit)
    }
}

impl RequestCanisterActionArgs {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    pub(crate) fn into_create_request_input(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        match self {
            RequestCanisterActionArgs::Install(change_args) => {
                change_args.into_create_request_input(dfx_orbit)
            }
            RequestCanisterActionArgs::Call(call_args) => {
                call_args.into_create_request_input(dfx_orbit)
            }
        }
    }
}
