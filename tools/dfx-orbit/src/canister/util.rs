use crate::DfxOrbit;
use anyhow::{bail, Context};
use dfx_core::config::model::dfinity::CanisterTypeProperties;
use slog::{info, Logger};
use std::path::{Path, PathBuf};

impl DfxOrbit {
    // TODO: Move to top level utils
    pub(crate) fn as_path_bufs(
        &self,
        canister: &str,
        paths: &[String],
    ) -> anyhow::Result<Vec<PathBuf>> {
        if paths.is_empty() {
            let canister_config = self.get_canister_config(canister)?;
            let CanisterTypeProperties::Assets { source, .. } = &canister_config.type_specific
            else {
                bail!("Canister {canister} is not an asset canister");
            };
            Ok(source.clone())
        } else {
            Ok(paths.iter().map(|source| PathBuf::from(&source)).collect())
        }
    }

    pub(crate) fn as_paths(paths: &[PathBuf]) -> Vec<&Path> {
        paths.iter().map(|pathbuf| pathbuf.as_path()).collect()
    }
}

pub(super) fn parse_arguments(
    arg_string: &Option<String>,
    arg_path: &Option<String>,
    raw_arg: &Option<String>,
) -> anyhow::Result<Option<Vec<u8>>> {
    // TODO: It would be really nice to be able to use `blob_from_arguments(..)` here, as in dfx, to get all the nice things such as help composing the argument.
    // First try to read the argument file, if it was provided

    let candid = arg_path
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
        .transpose()?;

    let raw_arg = raw_arg.as_ref().map(hex::decode).transpose()?;
    let arg = candid.or(raw_arg);
    Ok(arg)
}

pub(super) fn log_hashes(
    logger: &Logger,
    name: &str,
    local: &Option<String>,
    remote: &Option<String>,
) {
    info!(logger, "Hash mismatch of {}", name);
    info!(logger, "Request {}: {}", name, display_arg_checksum(remote));
    info!(logger, "Local {}:   {}", name, display_arg_checksum(local));
}

pub(super) fn display_arg_checksum(arg: &Option<String>) -> String {
    arg.as_ref()
        .map(|s| s.to_string())
        .unwrap_or(String::from("None"))
}
