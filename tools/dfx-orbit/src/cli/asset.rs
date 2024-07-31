//! Implements the `dfx-orbit canister upload-http-assets` CLI command.

mod util;

use crate::DfxOrbit;
use candid::{Nat, Principal};
use ic_asset::canister_api::{
    methods::batch::compute_evidence, types::batch_upload::common::ComputeEvidenceArguments,
};
use ic_utils::canister::CanisterBuilder;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use slog::{info, warn};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetUploadRequest {
    station_principal: Principal,
    asset_canister_principal: Principal,
    batch_id: Nat,
    evidence: ByteBuf,
}

// TODO: Use StationAgentResult instead of anyhow result
impl DfxOrbit {
    /// The main entry point for the `dfx orbit canister upload-http-assets` CLI.
    pub async fn upload_assets(
        &mut self,
        canister: String,
        files: Vec<String>,
    ) -> anyhow::Result<AssetUploadRequest> {
        // The path is needed in various forms.
        let source_pathbufs: Vec<PathBuf> =
            files.iter().map(|source| PathBuf::from(&source)).collect();
        let source_paths: Vec<&Path> = source_pathbufs
            .iter()
            .map(|pathbuf| pathbuf.as_path())
            .collect();

        let canister_id = self.canister_id(&canister)?;
        let logger = self.dfx.logger().clone();

        // Upload assets:
        let canister_agent = CanisterBuilder::new()
            .with_agent(self.interface.agent())
            .with_canister_id(canister_id)
            .build()?;
        let assets = assets_as_hash_map(&files);
        let batch_id = ic_asset::upload_and_propose(&canister_agent, assets, &logger).await?;
        println!("Proposed batch_id: {}", batch_id);
        // Compute evidence locally:
        let local_evidence = {
            let local_evidence =
                ic_asset::compute_evidence(&canister_agent, source_paths.as_ref(), &logger).await?;
            escape_hex_string(&local_evidence)
        };
        // Wait for the canister to compute evidence:

        // This part is stolen from ic_asset::sync::prepare_sync_for_proposal.  Unfortunately the relevant functions are private.
        // The docs explicitly include waiting for the evidence so this should really be made easier!  See: https://github.com/dfinity/sdk/blob/2509e81e11e71dce4045c679686c952809525470/docs/design/asset-canister-interface.md?plain=1#L85
        let compute_evidence_arg = ComputeEvidenceArguments {
            batch_id: batch_id.clone(),
            max_iterations: Some(97), // 75% of max(130) = 97.5
        };
        info!(logger, "Computing evidence.");
        let canister_evidence_bytes = loop {
            if let Some(evidence) = compute_evidence(&canister_agent, &compute_evidence_arg).await?
            {
                break evidence;
            }
        };
        let canister_evidence = blob_from_bytes(&canister_evidence_bytes);

        // TODO: Move this out of the agent into the tool
        println!(r#"Proposed batch_id: {batch_id}"#);
        if local_evidence == canister_evidence {
            info!(logger, "Local evidence matches canister evidence.");
        } else {
            warn!(logger, "Local evidence does not match canister evidence:\n  local:    {local_evidence}\n  canister:{canister_evidence}");
        }
        println!(r#"Assets have been uploaded.  For the changes to take effect, run:"#);
        println!(
            r#"dfx-orbit request canister call {canister} commit_proposed_batch '(record {{ batch_id = {batch_id} : nat; evidence = blob "{canister_evidence}" }})'"#
        );

        let upload_request = AssetUploadRequest {
            station_principal: self.station.config.station_id,
            asset_canister_principal: canister_id,
            batch_id,
            evidence: canister_evidence_bytes,
        };

        Ok(upload_request)
    }
}

// TODO: Implement request_upload_commit

// TODO: Move all these funtions to util
/// Lists all the files at the given path.
///
/// - Links are followed.
/// - Only files are returned.
/// - The files are sorted by name.
/// - Any files that cannot be read are ignored.
/// - The path includes the prefix.
fn list_assets(path: &str) -> Vec<PathBuf> {
    WalkDir::new(path)
        .sort_by_file_name()
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.into_path())
        .collect()
}

/// A hash map of all assets.
///
/// Note: Given that ordering in a HashMap is not deterministic, is this really the best API?
fn assets_as_hash_map(asset_dirs: &[String]) -> HashMap<String, PathBuf> {
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

/// Converts a hex string into one escaped as in a candid blob.
fn escape_hex_string(s: &str) -> String {
    let mut ans = String::with_capacity(s.len() + s.len() / 2);
    for chunk in s.chars().collect::<Vec<_>>()[..].chunks(2) {
        ans.push('\\');
        for char in chunk {
            ans.push(*char);
        }
    }
    ans
}

/// Converts a byte array into one escaped as a candid blob
fn blob_from_bytes(bytes: &[u8]) -> String {
    let mut ans = String::with_capacity(bytes.len() + bytes.len() / 2);
    for byte in bytes {
        ans.push('\\');
        ans.push_str(&format!("{:02x}", byte));
    }
    ans
}
