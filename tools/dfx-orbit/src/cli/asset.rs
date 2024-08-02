//! Implements the `dfx-orbit canister upload-http-assets` CLI command.

mod evidence;
mod upload;
mod util;

use crate::DfxOrbit;
use candid::{Nat, Principal};
use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use slog::{info, warn};
use std::path::{Path, PathBuf};

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
        let logger = self.logger.clone();

        // Upload assets:
        let batch_id = self
            .upload_assets_actual(canister_id, &source_paths)
            .await?;
        info!(self.logger, "Proposed batch_id: {}", batch_id);
        // Compute evidence locally:
        let local_evidence = self.compute_evidence(canister_id, &source_paths).await?;
        let local_evidence = escape_hex_string(&local_evidence);
        // Wait for the canister to compute evidence:

        let canister_evidence_bytes = self.request_evidence(canister_id, batch_id.clone()).await?;
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
