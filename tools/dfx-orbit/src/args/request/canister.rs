//! CLI arguments for `dfx-orbit canister`.

use crate::DfxOrbit;
use anyhow::{bail, Context};
use candid::Principal;
use clap::{Parser, Subcommand, ValueEnum};
use sha2::{Digest, Sha256};
use slog::{info, Logger};
use station_api::{
    CallExternalCanisterOperationInput, CanisterInstallMode, CanisterMethodDTO,
    ChangeExternalCanisterOperationInput, ConfigureExternalCanisterOperationInput,
    ConfigureExternalCanisterOperationKindDTO, DefiniteCanisterSettingsInput, GetRequestResponse,
    RequestOperationDTO, RequestOperationInput,
};
use std::collections::BTreeSet;

// TODO: Support Canister create + integration test
// TODO: Canister get response functionality
// ^ Utility function to get the latests response directly printed, to get UX similar to dfx canister call

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
    /// Update a canister's settings (i.e its controller, compute allocation, or memory allocation.)
    UpdateSettings(RequestCanisterUpdateSettingsArgs),
}

impl RequestCanisterArgs {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    pub(crate) async fn into_request(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        self.action.into_create_request_input(dfx_orbit).await
    }
}

impl RequestCanisterActionArgs {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    pub(crate) async fn into_create_request_input(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        match self {
            RequestCanisterActionArgs::Install(args) => args.into_request(dfx_orbit),
            RequestCanisterActionArgs::Call(args) => args.into_request(dfx_orbit),
            RequestCanisterActionArgs::UpdateSettings(args) => args.into_request(dfx_orbit).await,
        }
    }
}

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
    /// Pass the argument as a file.
    #[clap(short = 'f', long, conflicts_with = "argument")]
    arg_file: Option<String>,
    /// Specifies the amount of cycles to send on the call.
    #[clap(short, long)]
    with_cycles: Option<u64>,
}

impl RequestCanisterCallArgs {
    /// Converts the CLI arg stype into the equivalent Orbit API type.
    pub(crate) fn into_request(
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

    pub(crate) fn verify(
        &self,
        dfx_orbit: &DfxOrbit,
        request: &GetRequestResponse,
    ) -> anyhow::Result<()> {
        let canister_id = dfx_orbit.canister_id(&self.canister)?;
        let arg = candid_from_string_or_file(&self.argument, &self.arg_file)?;
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

/// Requests that a canister be installed or updated.  Equivalent to `orbit_station_api::CanisterInstallMode`.
#[derive(Debug, Clone, Parser)]
pub struct RequestCanisterInstallArgs {
    /// The canister name or ID.
    canister: String,
    /// The installation mode.
    #[clap(long, value_enum, rename_all = "kebab-case", default_value = "install")]
    mode: CanisterInstallModeArgs,
    // TODO: On verify, allow a --wasm-hash instead
    /// The path to the wasm file to install (can also be a wasm.gz).
    #[clap(short, long)]
    wasm: String,
    /// The argument to pass to the canister.
    #[clap(short, long, conflicts_with = "arg_file")]
    argument: Option<String>,
    /// The path to a file containing the argument to pass to the canister.
    #[clap(short = 'f', long, conflicts_with = "arg")]
    arg_file: Option<String>,
}

impl RequestCanisterInstallArgs {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    pub(crate) fn into_request(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        let canister_id = dfx_orbit.canister_id(&self.canister)?;

        let (module, arg) = self.load_module_and_args()?;
        let mode = self.mode.into();
        let operation = ChangeExternalCanisterOperationInput {
            canister_id,
            mode,
            module,
            arg,
        };
        Ok(RequestOperationInput::ChangeExternalCanister(operation))
    }

    pub(crate) fn verify(
        &self,
        dfx_orbit: &DfxOrbit,
        request: &GetRequestResponse,
    ) -> anyhow::Result<()> {
        let canister_id = dfx_orbit.canister_id(&self.canister)?;
        let (module, arg) = self.load_module_and_args()?;

        let module_checksum = hex::encode(Sha256::digest(module));
        let arg_checksum = arg.map(|arg| hex::encode(Sha256::digest(arg)));

        let RequestOperationDTO::ChangeExternalCanister(op) = &request.request.operation else {
            bail!("This request is not a change external canister request");
        };
        if op.canister_id != canister_id {
            bail!(
                "Canister id {} does not match expected canister id",
                op.canister_id
            );
        }
        if CanisterInstallModeArgs::from(op.mode.clone()) != self.mode {
            bail!("Canister install mode {:?} does not match", op.mode);
        }
        if op.module_checksum != module_checksum {
            log_hashes(
                &dfx_orbit.logger,
                "module",
                &Some(module_checksum),
                &Some(op.module_checksum.clone()),
            );
            bail!("Module checksum does not match");
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

        Ok(())
    }

    fn load_module_and_args(&self) -> anyhow::Result<(Vec<u8>, Option<Vec<u8>>)> {
        let module = std::fs::read(&self.wasm)
            .with_context(|| "Could not read Wasm file")?
            .to_vec();
        let args = candid_from_string_or_file(&self.argument, &self.arg_file)?;

        Ok((module, args))
    }
}

/// Canister installation mode equivalent to `dfx canister install --mode XXX` and `orbit_station_api::CanisterInstallMode`.
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
            CanisterInstallModeArgs::Install => Self::Install,
            CanisterInstallModeArgs::Reinstall => Self::Reinstall,
            CanisterInstallModeArgs::Upgrade => Self::Upgrade,
        }
    }
}

impl From<CanisterInstallMode> for CanisterInstallModeArgs {
    fn from(mode: CanisterInstallMode) -> Self {
        match mode {
            CanisterInstallMode::Install => Self::Install,
            CanisterInstallMode::Reinstall => Self::Reinstall,
            CanisterInstallMode::Upgrade => Self::Upgrade,
        }
    }
}

#[derive(Debug, Clone, Parser)]
pub struct RequestCanisterUpdateSettingsArgs {
    /// The canister name or ID.
    canister: String,

    /// Add a principal to the list of controllers of the canister
    #[clap(long)]
    pub(crate) add_controller: Vec<Principal>,

    /// Removes a principal from the list of controllers of the canister
    #[clap(long)]
    pub(crate) remove_controller: Vec<Principal>,
}

impl RequestCanisterUpdateSettingsArgs {
    pub(crate) async fn into_request(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        let canister_id = dfx_orbit.canister_id(&self.canister)?;
        let controllers = get_new_controller_set(
            dfx_orbit,
            canister_id,
            self.add_controller,
            self.remove_controller,
        )
        .await?;

        let operations = ConfigureExternalCanisterOperationInput {
            canister_id,
            kind: ConfigureExternalCanisterOperationKindDTO::NativeSettings(
                DefiniteCanisterSettingsInput {
                    controllers: Some(controllers),
                    compute_allocation: None,
                    memory_allocation: None,
                    freezing_threshold: None,
                    reserved_cycles_limit: None,
                },
            ),
        };

        Ok(RequestOperationInput::ConfigureExternalCanister(operations))
    }

    pub(crate) async fn verify(
        &self,
        dfx_orbit: &DfxOrbit,
        request: &GetRequestResponse,
    ) -> anyhow::Result<()> {
        let canister_id = dfx_orbit.canister_id(&self.canister)?;
        let controllers = get_new_controller_set(
            dfx_orbit,
            canister_id,
            self.add_controller.clone(),
            self.remove_controller.clone(),
        )
        .await?;

        let RequestOperationDTO::ConfigureExternalCanister(op) = &request.request.operation else {
            bail!("This request is not a configure external canister request");
        };
        if op.canister_id != canister_id {
            bail!(
                "Mismatch of canister ids: request: {}, local: {}",
                op.canister_id,
                canister_id
            );
        }
        let ConfigureExternalCanisterOperationKindDTO::NativeSettings(op) = &op.kind else {
            bail!("This request is not a native setting request");
        };
        if op.controllers.as_ref() != Some(&controllers) {
            bail!(
                "Mismatch in the controller sets: request: {:?}, local {:?}",
                op.controllers,
                controllers
            );
        }

        Ok(())
    }
}

async fn get_new_controller_set(
    dfx_orbit: &DfxOrbit,
    canister_id: Principal,
    add: Vec<Principal>,
    remove: Vec<Principal>,
) -> anyhow::Result<Vec<Principal>> {
    // Transform into maps to deduplicates
    let old_controllers = dfx_orbit.get_controllers(canister_id).await?;
    let controllers = old_controllers
        .iter()
        .chain(add.iter())
        .collect::<BTreeSet<_>>();
    let remove = remove.iter().collect::<BTreeSet<_>>();

    let new_controllers = controllers
        .difference(&remove)
        .map(|&&v| v)
        .collect::<Vec<_>>();

    Ok(new_controllers)
}

fn candid_from_string_or_file(
    arg_string: &Option<String>,
    arg_path: &Option<String>,
) -> anyhow::Result<Option<Vec<u8>>> {
    // TODO: It would be really nice to be able to use `blob_from_arguments(..)` here, as in dfx, to get all the nice things such as help composing the argument.
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

fn log_hashes(logger: &Logger, name: &str, local: &Option<String>, remote: &Option<String>) {
    info!(logger, "Hash mismatch of {}", name);
    info!(logger, "Request {}: {}", name, display_arg_checksum(remote));
    info!(logger, "Local {}:   {}", name, display_arg_checksum(local));
}

fn display_arg_checksum(arg: &Option<String>) -> String {
    arg.as_ref()
        .map(|s| s.to_string())
        .unwrap_or(String::from("None"))
}
