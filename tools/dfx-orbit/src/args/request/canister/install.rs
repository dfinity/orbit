//! Arguments for `dfx orbit canister change wasm`.

use clap::{Parser, ValueEnum};
use station_api::{
    CanisterInstallMode, ChangeExternalCanisterOperationInput, RequestOperationInput,
};

use crate::DfxOrbit;

/// Requests that a canister be installed or updated.  Equivalent to `station_api::CanisterInstallMode`.
#[derive(Debug, Clone, Parser)]
pub struct RequestCanisterInstallArgs {
    // TODO: Poll, waiting for the request to be accepted.
    /// The canister name or ID.
    canister: String,
    /// The installation mode.
    #[clap(long, value_enum, rename_all = "kebab-case", default_value = "install")]
    mode: CanisterInstallModeArgs,
    /// The path to the Wasm file to install.
    #[clap(short, long)]
    wasm: String,
    /// The argument to pass to the canister.
    #[clap(short, long, conflicts_with = "arg_file")]
    arg: Option<String>,
    /// The path to a file containing the argument to pass to the canister.
    #[clap(short = 'f', long, conflicts_with = "arg")]
    arg_file: Option<String>,
}

impl RequestCanisterInstallArgs {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    pub(crate) fn into_create_request_input(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        let RequestCanisterInstallArgs {
            canister,
            mode,
            wasm,
            arg,
            arg_file,
        } = self;
        let canister_id = dfx_orbit.canister_id(&canister)?;

        let operation = {
            let module = std::fs::read(wasm)
                .expect("Could not read Wasm file")
                .to_vec();
            let arg = if let Some(file) = arg_file {
                Some(
                    std::fs::read(file)
                        .expect("Could not read argument file")
                        .to_vec(),
                )
            } else {
                arg.map(|arg| arg.as_bytes().to_vec())
            };
            let mode = mode.into();
            ChangeExternalCanisterOperationInput {
                canister_id,
                mode,
                module,
                arg,
            }
        };
        Ok(RequestOperationInput::ChangeExternalCanister(operation))
    }
}

/// Canister installation mode equivalent to `dfx canister install --mode XXX` and `station_api::CanisterInstallMode`.
#[derive(Copy, Clone, Eq, PartialEq, Debug, ValueEnum)]
pub enum CanisterInstallModeArgs {
    /// Corresponds to `dfx canister install`
    Install,
    /// Corresponds to `dfx canister reinstall`
    Reinstall,
    /// Corresponds to `dfx canister upgrade`
    Upgrade,
}

impl From<CanisterInstallModeArgs> for CanisterInstallMode {
    fn from(mode: CanisterInstallModeArgs) -> Self {
        match mode {
            CanisterInstallModeArgs::Install => CanisterInstallMode::Install,
            CanisterInstallModeArgs::Reinstall => CanisterInstallMode::Reinstall,
            CanisterInstallModeArgs::Upgrade => CanisterInstallMode::Upgrade,
        }
    }
}
