//! CLI arguments for `dfx-orbit canister call`.

use crate::DfxOrbit;
use anyhow::Context;
use clap::Parser;
use station_api::{CallExternalCanisterOperationInput, CanisterMethodDTO, RequestOperationInput};

/// Requests that a call be made to a canister.
#[derive(Debug, Clone, Parser)]
pub struct RequestCanisterCallArgs {
    /// The canister name or ID.
    canister: String,
    /// The name of the method to call.
    method_name: String,
    /// The argument to pass to the method.
    argument: Option<String>,
    // TODO:
    // /// The format of the argument.
    // #[clap(short, long)]
    // r#type: Option<CandidFormat>,
    // TODO: Read argument from a file
    /// Specifies the amount of cycles to send on the call.
    #[clap(short, long)]
    with_cycles: Option<u64>,
}

impl RequestCanisterCallArgs {
    /// Converts the CLI arg stype into the equivalent Orbit API type.
    pub(crate) fn into_create_request_input(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        let RequestCanisterCallArgs {
            canister,
            method_name,
            with_cycles,
            argument,
        } = self;
        let canister_id = dfx_orbit.canister_id(&canister)?;

        // TODO: It would be really nice to be able to use `blob_from_arguments(..)` here, as in dfx, to geta ll the nice things such as help composing the argument.
        let arg = if let Some(argument) = argument {
            Some(
                candid_parser::parse_idl_args(&argument)
                    .with_context(|| "Invalid Candid values".to_string())?
                    .to_bytes()?,
            )
        } else {
            None
        };
        Ok(RequestOperationInput::CallExternalCanister(
            CallExternalCanisterOperationInput {
                validation_method: None,
                execution_method: CanisterMethodDTO {
                    canister_id,
                    method_name,
                },
                arg,
                execution_method_cycles: with_cycles,
            },
        ))
    }
}
