use super::util::parse_arguments;
use crate::{canister::util::log_hashes, DfxOrbit};
use anyhow::{bail, Context};
use candid::CandidType;
use clap::{Parser, ValueEnum};
use orbit_essentials::types::WasmModuleExtraChunks;
use sha2::{Digest, Sha256};
use station_api::{
    CanisterInstallMode, ChangeExternalCanisterOperationDTO, ChangeExternalCanisterOperationInput,
    GetRequestResponse, RequestOperationDTO, RequestOperationInput,
};
use std::fmt::Write;

/// The default maximum length in bytes of module chunks (the IC does not allow chunks larger than 1MiB).
const DEFAULT_MAX_CHUNK_LEN: usize = 1 << 20;

/// Requests that a canister be installed or updated.  Equivalent to `orbit_station_api::CanisterInstallMode`.
#[derive(Debug, Clone, Parser)]
pub struct RequestCanisterInstallArgs {
    /// The canister name or ID.
    pub canister: String,
    /// The installation mode.
    #[clap(long, value_enum, rename_all = "kebab-case")]
    pub mode: CanisterInstallModeArgs,
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
    /// The maximum length in bytes of module chunks.
    #[clap(long)]
    pub max_chunk_len: Option<usize>,
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
        let mode = self.mode.into();

        let (module, module_extra_chunks) = if let Some(ref asset_canister) = self.asset_canister {
            if self.max_chunk_len > Some(DEFAULT_MAX_CHUNK_LEN) {
                bail!(
                    "--max-chunk-len must not be larger than {}",
                    DEFAULT_MAX_CHUNK_LEN
                );
            }

            let asset_canister_id = dfx_orbit.canister_id(asset_canister)?;
            let asset_agent = dfx_orbit.canister_agent(asset_canister_id)?;

            // compute module hash
            let canister_wasm_hash = hash(&module);

            // chunk module
            let chunks: Vec<&[u8]> = module
                .chunks(self.max_chunk_len.unwrap_or(DEFAULT_MAX_CHUNK_LEN))
                .collect();

            // upload chunks to asset canister
            for chunk in &chunks {
                let chunk_hash = hash(chunk);
                let store_arg = StoreArg {
                    key: format!("/{}", hex::encode(chunk_hash.clone())),
                    content: chunk.to_vec(),
                    content_type: "application/octet-stream".to_string(),
                    content_encoding: "identity".to_string(),
                    sha256: Some(chunk_hash),
                };
                asset_agent
                    .update("store")
                    .with_arg(store_arg)
                    .build()
                    .call_and_wait()
                    .await?;
            }

            (
                vec![],
                Some(WasmModuleExtraChunks {
                    store_canister: asset_canister_id,
                    chunk_hashes_list: chunks.iter().map(|c| hash(c)).collect(),
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
        let args = parse_arguments(&self.argument, &self.arg_file, &None)?;

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
            CanisterInstallMode::Upgrade => "Upgrade",
        };
        writeln!(output, "Mode: {}", mode)?;

        writeln!(output, "Module checksum: {}", &op.module_checksum)?;
        if let Some(arg_checksum) = &op.arg_checksum {
            writeln!(output, "Argument checksum: {}", arg_checksum)?;
        }
        Ok(())
    }
}
