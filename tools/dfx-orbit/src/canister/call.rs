use super::util::parse_arguments;
use crate::{canister::util::log_hashes, DfxOrbit};
use anyhow::bail;
use clap::Parser;
use sha2::{Digest, Sha256};
use station_api::{
    CallExternalCanisterOperationInput, CanisterMethodDTO, GetRequestResponse, RequestOperationDTO,
    RequestOperationInput,
};

/// Requests that a call be made to a canister.
#[derive(Debug, Clone, Parser)]
pub struct RequestCanisterCallArgs {
    /// The canister name or ID.
    pub canister: String,
    /// The name of the method to call.
    pub method_name: String,
    /// The candid argument to pass to the method.
    pub argument: Option<String>,
    // TODO: The format of the argument.
    // #[clap(short, long)]
    // r#type: Option<CandidFormat>,
    /// Pass the argument as a candid encoded file.
    #[clap(short = 'f', long, conflicts_with = "argument")]
    pub arg_file: Option<String>,
    /// Pass the argument as a raw hex string.
    #[clap(short = 'f', long, conflicts_with = "argument, arg_file")]
    pub raw_arg: Option<String>,
    /// Specifies the amount of cycles to send on the call.
    #[clap(short, long)]
    pub with_cycles: Option<u64>,
}

impl RequestCanisterCallArgs {
    /// Converts the CLI arg stype into the equivalent Orbit API type.
    pub(crate) fn into_request(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        let canister_id = dfx_orbit.canister_id(&self.canister)?;
        let arg = parse_arguments(&self.argument, &self.arg_file, &self.raw_arg)?;

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

    pub(crate) fn verify(
        &self,
        dfx_orbit: &DfxOrbit,
        request: &GetRequestResponse,
    ) -> anyhow::Result<()> {
        let canister_id = dfx_orbit.canister_id(&self.canister)?;
        let arg = parse_arguments(&self.argument, &self.arg_file, &self.raw_arg)?;
        let arg_checksum = arg.map(|arg| hex::encode(Sha256::digest(arg)));

        let RequestOperationDTO::CallExternalCanister(op) = &request.request.operation else {
            bail!("This request is not an external canister call");
        };
        if op.execution_method.canister_id != canister_id {
            bail!(
                "Canister id of request \"{}\" does not match expected id",
                op.execution_method.canister_id
            )
        }
        if op.execution_method.method_name != self.method_name {
            bail!(
                "The request targets another method: \"{}\"",
                op.execution_method.method_name
            )
        }
        if op.arg_checksum != arg_checksum {
            log_hashes(
                &dfx_orbit.logger,
                "argument",
                &arg_checksum,
                &op.arg_checksum,
            );
            bail!("Argument checksum does not match");
        }
        if op.execution_method_cycles != self.with_cycles {
            bail!("Attached cycles do not match");
        }

        Ok(())
    }
}
