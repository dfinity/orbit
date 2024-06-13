//! Makes requests to Orbit.

use candid::Principal;
use clap::{Parser, Subcommand, ValueEnum};

/// Request canister changes.
#[derive(Debug, Subcommand)]
#[command(version, about, long_about = None)]
pub enum Args {
    /// Request to update the canister.
    Change(ChangeExternalCanister),
}

/// Requests that a canister be installed or updated.  Equivalent to `orbit_station_api::CanisterInstallMode`.
#[derive(Debug, Parser)]
pub struct ChangeExternalCanister {
    /// The canister ID to install or update.
    #[clap(short, long)]
    canister_id: Principal,
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

impl From<ChangeExternalCanister> for orbit_station_api::ChangeExternalCanisterOperationInput {
    fn from(input: ChangeExternalCanister) -> Self {
        let ChangeExternalCanister {
            canister_id,
            mode,
            wasm,
            arg,
            arg_file,
        } = input;
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
        orbit_station_api::ChangeExternalCanisterOperationInput {
            canister_id,
            mode,
            module,
            arg,
        }
    }
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
