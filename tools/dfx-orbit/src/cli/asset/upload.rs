use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use candid::{Nat, Principal};
use walkdir::WalkDir;

use crate::DfxOrbit;

impl DfxOrbit {
    pub async fn upload_assets_actual(
        &self,
        canister_id: Principal,
        sources: &[&Path],
    ) -> anyhow::Result<Nat> {
        let canister_agent = self.canister_agent(canister_id)?;
        let assets = assets_as_hash_map(sources);
        Ok(ic_asset::upload_and_propose(&canister_agent, assets, &self.logger).await?)
    }

    // TODO: Implement request_upload_commit
}

/// A hash map of all assets.
fn assets_as_hash_map(asset_dirs: &[&Path]) -> HashMap<String, PathBuf> {
    asset_dirs
        .iter()
        .flat_map(|asset_dir| {
            list_assets(asset_dir).into_iter().map(move |asset_path| {
                let relative_path = asset_path.strip_prefix(asset_dir).expect(
                    "Internal error: list_assets should have returned only files in the asset_dir",
                );
                let http_path = format!(
                    "/{relative_path}",
                    relative_path = relative_path.to_string_lossy()
                );
                (http_path, asset_path)
            })
        })
        .collect()
}

/// Lists all the files at the given path.
///
/// - Links are followed.
/// - Only files are returned.
/// - The files are sorted by name.
/// - Any files that cannot be read are ignored.
/// - The path includes the prefix.
fn list_assets(path: &Path) -> Vec<PathBuf> {
    WalkDir::new(path)
        .sort_by_file_name()
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.into_path())
        .collect()
}
