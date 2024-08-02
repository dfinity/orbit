//! Implements the `dfx-orbit canister upload-http-assets` CLI command.

mod evidence;
mod upload;
mod util;

use crate::{
    args::asset::{AssetArgs, AssetArgsAction},
    DfxOrbit,
};
use candid::Principal;
use ic_utils::Canister;
use slog::Logger;
use std::path::{Path, PathBuf};

pub struct AssetAgent<'agent> {
    canister_agent: Canister<'agent>,
    logger: Logger,
}

impl DfxOrbit {
    pub async fn exec_asset(&mut self, args: AssetArgs) -> anyhow::Result<()> {
        match args.action {
            AssetArgsAction::Upload(upload_args) => {
                let pathbufs = as_path_bufs(upload_args.files);
                let paths = as_paths(&pathbufs);

                let canister_name = upload_args.canister;
                let canister_id = self.canister_id(&canister_name)?;
                let (batch_id, evidence) = self
                    .upload(canister_id, &paths, upload_args.ignore_evidence)
                    .await?;

                if !upload_args.skip_commit {
                    let result = self
                        .request_commit_batch(canister_id, batch_id.clone(), evidence)
                        .await?;
                    let request_id = result.request.id;

                    println!("Created request to commit batches. To verify the batch against local files, run:");
                    println!("dfx-orbit asset check {canister_name} {request_id} [FILES]");
                } else {
                    let evidence = hex::encode(&evidence);
                    println!("Prepared the batches. To commit, run:");
                    println!("dfx-orbit asset commit {canister_id} {batch_id} {evidence}");
                }
                Ok(())
            }
            AssetArgsAction::Commit(_) => todo!(),
            AssetArgsAction::ComputeEvidence(compute_args) => {
                let pathbufs = as_path_bufs(compute_args.files);
                let paths = as_paths(&pathbufs);

                let canister_id = self.canister_id(&compute_args.canister)?;
                let asset_agent = self.asset_agent(canister_id)?;

                let evidence = asset_agent.compute_evidence(&paths).await?;
                println!("{evidence}");
                Ok(())
            }
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

fn as_path_bufs(paths: Vec<String>) -> Vec<PathBuf> {
    paths.iter().map(|source| PathBuf::from(&source)).collect()
}

fn as_paths(paths: &Vec<PathBuf>) -> Vec<&Path> {
    paths.iter().map(|pathbuf| pathbuf.as_path()).collect()
}
