use super::AssetAgent;
use crate::DfxOrbit;
use candid::{Nat, Principal};
use ic_certified_assets::types::CommitProposedBatchArguments;
use orbit_station_api::{
    CallExternalCanisterOperationInput, CanisterMethodDTO, CreateRequestInput,
    CreateRequestResponse, RequestOperationInput,
};
use serde_bytes::ByteBuf;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

impl DfxOrbit {
    pub async fn upload(&self) -> anyhow::Result<()> {
        todo!()
    }

    pub async fn request_commit_batch(
        &self,
        canister_id: Principal,
        batch_id: Nat,
        evidence: ByteBuf,
    ) -> anyhow::Result<CreateRequestResponse> {
        let args = CommitProposedBatchArguments { batch_id, evidence };
        let arg = candid::encode_one(args)?;

        let response = self
            .station
            .request(CreateRequestInput {
                operation: RequestOperationInput::CallExternalCanister(
                    CallExternalCanisterOperationInput {
                        validation_method: None,
                        execution_method: CanisterMethodDTO {
                            canister_id,
                            method_name: String::from("commit_proposed_batch"),
                        },
                        arg: Some(arg),
                        execution_method_cycles: None,
                    },
                ),
                title: None,
                summary: None,
                execution_plan: None,
            })
            .await?;

        Ok(response)
    }
}

impl AssetAgent<'_> {
    pub async fn upload_assets(&self, sources: &[&Path]) -> anyhow::Result<Nat> {
        let assets = assets_as_hash_map(sources);
        Ok(ic_asset::upload_and_propose(&self.canister_agent, assets, &self.logger).await?)
    }
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
