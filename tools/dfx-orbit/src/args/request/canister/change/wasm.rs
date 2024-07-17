//! Arguments for `dfx orbit canister change wasm`.
use crate::{args::request::CreateRequestArgs, StationAgent};
use clap::{Parser, ValueEnum};
use orbit_station_api::{
    ChangeExternalCanisterOperationInput, CreateRequestInput, RequestOperationInput,
};

/// Requests that a canister be installed or updated.  Equivalent to `orbit_station_api::CanisterInstallMode`.
#[derive(Debug, Parser)]
pub struct RequestCanisterChangeWasmArgs {
    // TODO: Poll, waiting for the request to be accepted.
    /// The canister name or ID.
    #[clap(short, long)]
    canister: String,
    /// The installation mode.
    #[clap(long, value_enum, rename_all = "kebab-case")]
    mode: CanisterInstallMode,
    /// The path to the Wasm file to install.
    #[clap(short, long)]
    wasm: String,
    /// The argument to pass to the canister.
    #[clap(short, long)]
    arg: Option<String>,
    // TODO: exclusive OR
    /// The path to a file containing the argument to pass to the canister.
    #[clap(short = 'f', long)]
    arg_file: Option<String>,
}

impl CreateRequestArgs for RequestCanisterChangeWasmArgs {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    fn into_create_request_input(
        self,
        station_agent: &StationAgent,
    ) -> anyhow::Result<CreateRequestInput> {
        let RequestCanisterChangeWasmArgs {
            canister,
            mode,
            wasm,
            arg,
            arg_file,
        } = self;
        let canister_id = station_agent.canister_id(&canister)?;
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
        let operation = RequestOperationInput::ChangeExternalCanister(operation);
        Ok(CreateRequestInput {
            operation,
            title: None,
            summary: None,
            execution_plan: None,
        })
    }
}

/// Canister installation mode equivalent to `dfx canister install --mode XXX` and `orbit_station_api::CanisterInstallMode`.
#[derive(Copy, Clone, Eq, PartialEq, Debug, ValueEnum)]
pub enum CanisterInstallMode {
    /// Corresponds to `dfx canister install`
    Install,
    /// Corresponds to `dfx canister reinstall`
    Reinstall,
    /// Corresponds to `dfx canister upgrade`
    Upgrade,
}

impl From<CanisterInstallMode> for orbit_station_api::CanisterInstallMode {
    fn from(mode: CanisterInstallMode) -> Self {
        match mode {
            CanisterInstallMode::Install => orbit_station_api::CanisterInstallMode::Install,
            CanisterInstallMode::Reinstall => orbit_station_api::CanisterInstallMode::Reinstall,
            CanisterInstallMode::Upgrade => orbit_station_api::CanisterInstallMode::Upgrade,
        }
    }
}
