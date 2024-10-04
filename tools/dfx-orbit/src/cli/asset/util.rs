use crate::DfxOrbit;
use anyhow::bail;
use dfx_core::config::model::dfinity::CanisterTypeProperties;
use std::path::{Path, PathBuf};

impl DfxOrbit {
    pub fn as_path_bufs(&self, canister: &str, paths: &[String]) -> anyhow::Result<Vec<PathBuf>> {
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
