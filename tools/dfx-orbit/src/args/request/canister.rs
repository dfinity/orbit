//! CLI arguments for `dfx-orbit canister`.

pub mod call;
pub mod install;

use crate::DfxOrbit;
use anyhow::Context;
use call::RequestCanisterCallArgs;
use clap::{Parser, Subcommand};
use install::RequestCanisterInstallArgs;
use station_api::RequestOperationInput;

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

fn candid_from_string_or_file(
    arg_string: &Option<String>,
    arg_path: &Option<String>,
) -> anyhow::Result<Option<Vec<u8>>> {
    // TODO: It would be really nice to be able to use `blob_from_arguments(..)` here, as in dfx, to geta ll the nice things such as help composing the argument.
    // First try to read the argument file, if it was provided
    Ok(arg_path
        .as_ref()
        .map(std::fs::read_to_string)
        .transpose()?
        // Otherwise use the argument from the command line
        .or_else(|| arg_string.clone())
        // Parse the candid
        .map(|arg_string| {
            candid_parser::parse_idl_args(&arg_string)
                .with_context(|| "Invalid Candid values".to_string())?
                .to_bytes()
        })
        .transpose()?)
}
