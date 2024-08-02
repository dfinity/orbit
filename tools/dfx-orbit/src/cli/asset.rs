//! Implements the `dfx-orbit canister upload-http-assets` CLI command.

mod evidence;
mod upload;
mod util;

use std::path::{Path, PathBuf};

use crate::{
    args::asset::{AssetArgs, AssetArgsAction},
    DfxOrbit,
};
use candid::Principal;
use ic_utils::Canister;
use slog::Logger;

pub struct AssetAgent<'agent> {
    canister_agent: Canister<'agent>,
    logger: Logger,
}

impl DfxOrbit {
    pub async fn exec_asset(&mut self, args: AssetArgs) -> anyhow::Result<()> {
        match args.action {
            AssetArgsAction::Upload(upload_args) => {
                let source_pathbufs: Vec<PathBuf> = upload_args
                    .files
                    .iter()
                    .map(|source| PathBuf::from(&source))
                    .collect();
                let source_paths: Vec<&Path> = source_pathbufs
                    .iter()
                    .map(|pathbuf| pathbuf.as_path())
                    .collect();

                let canister_id = self.canister_id(&upload_args.canister)?;
                let (batch_id, evidence) = self
                    .upload(canister_id, &source_paths, upload_args.ignore_evidence)
                    .await?;

                if !upload_args.skip_commit {
                    let result = self
                        .request_commit_batch(canister_id, batch_id.clone(), evidence)
                        .await?;
                    let request_id = result.request.id;

                    println!("Created request to commit batches. To verify the batch against local files, run:");
                    println!("dfx-orbit asset check {request_id}");
                } else {
                    let evidence = hex::encode(&evidence);
                    println!("Prepared the batches. To commit, run:");
                    println!("dfx-orbit asset commit {canister_id} {batch_id} {evidence}");
                }
                Ok(())
            }
            AssetArgsAction::Commit(_) => todo!(),
            AssetArgsAction::ComputeEvidence(_) => todo!(),
            AssetArgsAction::Check(_) => todo!(),
        }
    }

    pub fn asset_agent(&self, canister_id: Principal) -> anyhow::Result<AssetAgent> {
        Ok(AssetAgent {
            canister_agent: self.canister_agent(canister_id)?,
            logger: self.logger.clone(),
        })
    }
}
