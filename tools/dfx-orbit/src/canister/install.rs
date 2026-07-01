use super::util::parse_arguments;
use crate::{canister::util::log_hashes, DfxOrbit};
use anyhow::{bail, Context};
use candid::CandidType;
use clap::{Parser, ValueEnum};
use orbit_essentials::types::WasmModuleExtraChunks;
use sha2::{Digest, Sha256};
use station_api::{
    CanisterInstallMode, CanisterUpgradeOptionsInput, ChangeExternalCanisterOperationDTO,
    ChangeExternalCanisterOperationInput, GetRequestResponse, RequestOperationDTO,
    RequestOperationInput, WasmMemoryPersistence,
};
use std::{collections::HashMap, fmt::Write, path::PathBuf};

/// Requests that a canister be installed or updated.  Equivalent to `orbit_station_api::CanisterInstallMode`.
#[derive(Debug, Clone, Parser)]
pub struct RequestCanisterInstallArgs {
    /// The canister name or ID.
    pub canister: String,
    /// The installation mode.
    #[clap(long, value_enum, rename_all = "kebab-case")]
    pub mode: CanisterInstallModeArgs,
    /// Controls whether the canister's Wasm main memory is kept or replaced during
    /// an upgrade. Only valid with `--mode upgrade`. Motoko canisters that use
    /// Enhanced Orthogonal Persistence require `keep`, otherwise the IC clears
    /// their main memory. When omitted, the IC default (`replace`) applies.
    #[clap(long, value_enum, rename_all = "kebab-case")]
    pub wasm_memory_persistence: Option<WasmMemoryPersistenceArgs>,
    /// Skip the canister's `pre_upgrade` hook during an upgrade. Only valid with
    /// `--mode upgrade`. Useful for recovery when the existing `pre_upgrade` hook
    /// traps.
    #[clap(long)]
    pub skip_pre_upgrade: bool,
    /// The path to the wasm file to install (can also be a wasm.gz).
    #[clap(short, long)]
    pub wasm: String,
    /// The argument to pass to the canister.
    #[clap(short, long, conflicts_with = "arg_file")]
    pub argument: Option<String>,
    /// The path to a file containing the argument to pass to the canister.
    #[clap(short = 'f', long, conflicts_with = "argument")]
    pub arg_file: Option<String>,
    /// The asset canister name or ID to upload module chunks to.
    #[clap(long)]
    pub asset_canister: Option<String>,
}

#[derive(CandidType)]
struct StoreArg {
    pub key: String,
    pub content: Vec<u8>,
    pub content_type: String,
    pub content_encoding: String,
    pub sha256: Option<Vec<u8>>,
}

fn hash(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    hasher.finalize().to_vec()
}

impl RequestCanisterInstallArgs {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    pub(crate) async fn into_request(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        let canister_id = dfx_orbit.canister_id(&self.canister)?;

        let (module, arg) = self.load_module_and_args()?;
        let mode = self.install_mode()?;

        let (module, module_extra_chunks) = if let Some(ref asset_canister) = self.asset_canister {
            let asset_canister_id = dfx_orbit.canister_id(asset_canister)?;
            let asset_agent = dfx_orbit.canister_agent(asset_canister_id)?;

            // compute module hash
            let canister_wasm_hash = hash(&module);

            // upload module as extra chunks to asset canister
            let path: PathBuf = self.wasm.into();
            let extra_chunks_key = path
                .file_name()
                .with_context(|| {
                    format!(
                        "Could not derive the WASM file name from {}",
                        path.display()
                    )
                })?
                .to_str()
                .with_context(|| "The WASM file name cannot be converted to a String")?
                .to_string();
            ic_asset::upload(
                &asset_agent,
                HashMap::from([(extra_chunks_key.clone(), path)]),
                &dfx_orbit.logger,
            )
            .await?;

            (
                vec![],
                Some(WasmModuleExtraChunks {
                    store_canister: asset_canister_id,
                    extra_chunks_key,
                    wasm_module_hash: canister_wasm_hash,
                }),
            )
        } else {
            (module, None)
        };
        let operation = ChangeExternalCanisterOperationInput {
            canister_id,
            mode,
            module,
            module_extra_chunks,
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
        let expected_mode = self.install_mode()?;
        if op.mode != expected_mode {
            bail!(
                "Canister install mode {:?} does not match expected {:?}",
                op.mode,
                expected_mode
            );
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
        let args = parse_arguments(&self.argument, &self.arg_file, &None)?;

        Ok((module, args))
    }

    /// Builds the Orbit `CanisterInstallMode`, folding the upgrade-only flags
    /// (`--wasm-memory-persistence`, `--skip-pre-upgrade`) into the upgrade
    /// options record. The flags are rejected for `install`/`reinstall` since
    /// the IC only honours them on upgrades.
    fn install_mode(&self) -> anyhow::Result<CanisterInstallMode> {
        match self.mode {
            CanisterInstallModeArgs::Install => {
                self.ensure_no_upgrade_options()?;
                Ok(CanisterInstallMode::Install)
            }
            CanisterInstallModeArgs::Reinstall => {
                self.ensure_no_upgrade_options()?;
                Ok(CanisterInstallMode::Reinstall)
            }
            CanisterInstallModeArgs::Upgrade => {
                // Collapse to `None` when neither flag is set so the request stays
                // byte-for-byte identical to a plain `--mode upgrade`.
                let options = if self.wasm_memory_persistence.is_some() || self.skip_pre_upgrade {
                    Some(CanisterUpgradeOptionsInput {
                        wasm_memory_persistence: self.wasm_memory_persistence.map(Into::into),
                        skip_pre_upgrade: self.skip_pre_upgrade.then_some(true),
                    })
                } else {
                    None
                };
                Ok(CanisterInstallMode::Upgrade(options))
            }
        }
    }

    fn ensure_no_upgrade_options(&self) -> anyhow::Result<()> {
        if self.wasm_memory_persistence.is_some() || self.skip_pre_upgrade {
            bail!(
                "--wasm-memory-persistence and --skip-pre-upgrade are only valid with --mode upgrade"
            );
        }
        Ok(())
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
            CanisterInstallModeArgs::Upgrade => Self::Upgrade(None),
        }
    }
}

impl From<CanisterInstallMode> for CanisterInstallModeArgs {
    fn from(mode: CanisterInstallMode) -> Self {
        match mode {
            CanisterInstallMode::Install => Self::Install,
            CanisterInstallMode::Reinstall => Self::Reinstall,
            CanisterInstallMode::Upgrade(_) => Self::Upgrade,
        }
    }
}

/// Whether the canister's Wasm main memory is kept or replaced on upgrade.
/// Equivalent to `dfx canister install --wasm-memory-persistence XXX` and
/// `orbit_station_api::WasmMemoryPersistence`.
#[derive(Copy, Clone, Eq, PartialEq, Debug, ValueEnum)]
pub enum WasmMemoryPersistenceArgs {
    /// Keep the canister's main memory (required for Motoko Enhanced Orthogonal Persistence).
    Keep,
    /// Replace (clear) the canister's main memory.
    Replace,
}

impl From<WasmMemoryPersistenceArgs> for WasmMemoryPersistence {
    fn from(mode: WasmMemoryPersistenceArgs) -> Self {
        match mode {
            WasmMemoryPersistenceArgs::Keep => Self::Keep,
            WasmMemoryPersistenceArgs::Replace => Self::Replace,
        }
    }
}

impl DfxOrbit {
    pub(crate) fn display_change_canister_operation(
        &self,
        output: &mut String,
        op: &ChangeExternalCanisterOperationDTO,
    ) -> anyhow::Result<()> {
        writeln!(output, "=== Change External Canister ===")?;
        writeln!(
            output,
            "Target: {}",
            self.try_reverse_lookup(&op.canister_id)
        )?;

        let mode = match op.mode {
            CanisterInstallMode::Install => "Install",
            CanisterInstallMode::Reinstall => "Reinstall",
            CanisterInstallMode::Upgrade(_) => "Upgrade",
        };
        writeln!(output, "Mode: {mode}")?;

        if let CanisterInstallMode::Upgrade(Some(options)) = &op.mode {
            if let Some(persistence) = &options.wasm_memory_persistence {
                let persistence = match persistence {
                    WasmMemoryPersistence::Keep => "keep",
                    WasmMemoryPersistence::Replace => "replace",
                };
                writeln!(output, "Wasm memory persistence: {persistence}")?;
            }
            if let Some(skip_pre_upgrade) = options.skip_pre_upgrade {
                writeln!(output, "Skip pre-upgrade: {skip_pre_upgrade}")?;
            }
        }

        writeln!(output, "Module checksum: {}", &op.module_checksum)?;
        if let Some(arg_checksum) = &op.arg_checksum {
            writeln!(output, "Argument checksum: {arg_checksum}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn args(mode: CanisterInstallModeArgs) -> RequestCanisterInstallArgs {
        RequestCanisterInstallArgs {
            canister: "test".to_string(),
            mode,
            wasm_memory_persistence: None,
            skip_pre_upgrade: false,
            wasm: "test.wasm".to_string(),
            argument: None,
            arg_file: None,
            asset_canister: None,
        }
    }

    #[test]
    fn install_mode_ignores_upgrade_flags_when_unset() {
        assert_eq!(
            args(CanisterInstallModeArgs::Install)
                .install_mode()
                .unwrap(),
            CanisterInstallMode::Install
        );
        assert_eq!(
            args(CanisterInstallModeArgs::Reinstall)
                .install_mode()
                .unwrap(),
            CanisterInstallMode::Reinstall
        );
    }

    #[test]
    fn upgrade_without_flags_produces_no_options() {
        assert_eq!(
            args(CanisterInstallModeArgs::Upgrade)
                .install_mode()
                .unwrap(),
            CanisterInstallMode::Upgrade(None)
        );
    }

    #[test]
    fn upgrade_forwards_wasm_memory_persistence() {
        let mut args = args(CanisterInstallModeArgs::Upgrade);
        args.wasm_memory_persistence = Some(WasmMemoryPersistenceArgs::Keep);

        assert_eq!(
            args.install_mode().unwrap(),
            CanisterInstallMode::Upgrade(Some(CanisterUpgradeOptionsInput {
                wasm_memory_persistence: Some(WasmMemoryPersistence::Keep),
                skip_pre_upgrade: None,
            }))
        );
    }

    #[test]
    fn upgrade_forwards_skip_pre_upgrade() {
        let mut args = args(CanisterInstallModeArgs::Upgrade);
        args.skip_pre_upgrade = true;

        assert_eq!(
            args.install_mode().unwrap(),
            CanisterInstallMode::Upgrade(Some(CanisterUpgradeOptionsInput {
                wasm_memory_persistence: None,
                skip_pre_upgrade: Some(true),
            }))
        );
    }

    #[test]
    fn upgrade_forwards_both_flags() {
        let mut args = args(CanisterInstallModeArgs::Upgrade);
        args.wasm_memory_persistence = Some(WasmMemoryPersistenceArgs::Replace);
        args.skip_pre_upgrade = true;

        assert_eq!(
            args.install_mode().unwrap(),
            CanisterInstallMode::Upgrade(Some(CanisterUpgradeOptionsInput {
                wasm_memory_persistence: Some(WasmMemoryPersistence::Replace),
                skip_pre_upgrade: Some(true),
            }))
        );
    }

    #[test]
    fn upgrade_flags_rejected_for_install_and_reinstall() {
        for mode in [
            CanisterInstallModeArgs::Install,
            CanisterInstallModeArgs::Reinstall,
        ] {
            let mut args = args(mode);
            args.wasm_memory_persistence = Some(WasmMemoryPersistenceArgs::Keep);
            assert!(args.install_mode().is_err());

            let mut args = args;
            args.wasm_memory_persistence = None;
            args.skip_pre_upgrade = true;
            assert!(args.install_mode().is_err());
        }
    }
}
