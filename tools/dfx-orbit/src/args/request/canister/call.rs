//! CLI arguments for `dfx-orbit canister call`.

use crate::DfxOrbit;
use clap::Parser;
use orbit_station_api::{
    CallExternalCanisterOperationInput, CanisterMethodDTO, RequestOperationInput,
};

use super::candid_from_string_or_file;

/// Requests that a call be made to a canister.
#[derive(Debug, Clone, Parser)]
pub struct RequestCanisterCallArgs {
    /// The canister name or ID.
    canister: String,
    /// The name of the method to call.
    method_name: String,
    /// The argument to pass to the method.
    argument: Option<String>,
    // TODO: The format of the argument.
    // #[clap(short, long)]
    // r#type: Option<CandidFormat>,
    #[clap(short = 'f', long, conflicts_with = "argument")]
    arg_file: Option<String>,
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
        let canister_id = dfx_orbit.canister_id(&self.canister)?;
        let arg = candid_from_string_or_file(&self.argument, &self.arg_file)?;

        Ok(RequestOperationInput::CallExternalCanister(
            CallExternalCanisterOperationInput {
                validation_method: None,
                execution_method: CanisterMethodDTO {
                    canister_id,
                    method_name: self.method_name,
                },
                arg,
                execution_method_cycles: self.with_cycles,
            },
        ))
    }
}
