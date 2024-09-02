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
use station_api::{RequestApprovalStatusDTO, SubmitRequestApprovalInput};

pub struct AssetAgent<'agent> {
    canister_agent: Canister<'agent>,
    logger: Logger,
}

impl DfxOrbit {
    pub async fn exec_asset(&mut self, args: AssetArgs) -> anyhow::Result<()> {
        match args.action {
            AssetArgsAction::RequestPreparePermission(args) => {
                let canister_id = self.canister_id(&args.canister)?;
                let request = self
                    .request_prepare_permission(canister_id, args.title, args.summary)
                    .await?;
                self.print_create_request_info(&request);

                Ok(())
            }
            AssetArgsAction::Upload(args) => {
                let pathbufs = self.as_path_bufs(&args.canister, &args.files)?;
                let paths = Self::as_paths(&pathbufs);

                let canister_name = args.canister;
                let canister_id = self.canister_id(&canister_name)?;
                let (batch_id, evidence) = self
                    .upload(canister_id, &paths, args.ignore_evidence)
                    .await?;

                let result = self
                    .request_commit_batch(
                        canister_id,
                        batch_id.clone(),
                        evidence,
                        args.title,
                        args.summary,
                    )
                    .await?;
                let request_id = result.request.id;

                let files = args.files.join(" ");
                println!("Created request to commit batches. To verify the batch against local files, run:");
                println!("dfx-orbit asset check {canister_name} {request_id} {batch_id} {files}");

                Ok(())
            }
            AssetArgsAction::ComputeEvidence(args) => {
                let pathbufs = self.as_path_bufs(&args.canister, &args.files)?;
                let paths = Self::as_paths(&pathbufs);

                let canister_id = self.canister_id(&args.canister)?;
                let asset_agent = self.asset_agent(canister_id)?;

                let evidence = asset_agent.compute_evidence(&paths).await?;
                println!("{evidence}");
                Ok(())
            }
            AssetArgsAction::Check(args) => {
                let pathbufs = self.as_path_bufs(&args.canister, &args.files)?;
                let paths = Self::as_paths(&pathbufs);

                let canister_id = self.canister_id(&args.canister)?;
                let asset_agent = self.asset_agent(canister_id)?;

                let evidence = asset_agent.compute_evidence(&paths).await?;
                self.check_evidence(
                    canister_id,
                    args.request_id.clone(),
                    args.batch_id,
                    evidence,
                )
                .await?;

                println!("Local evidence matches expected arguments");

                if args.then_approve {
                    dfx_core::cli::ask_for_consent("Do you want to approve the request?")?;
                    let args = SubmitRequestApprovalInput {
                        decision: RequestApprovalStatusDTO::Approved,
                        request_id: args.request_id,
                        reason: None,
                    };
                    self.station.submit(args).await?;
                }
                Ok(())
            }
        }
    }

    pub fn asset_agent(&self, canister_id: Principal) -> anyhow::Result<AssetAgent> {
        Ok(AssetAgent {
            canister_agent: self.canister_agent(canister_id)?,
            logger: self.logger.clone(),
        })
    }
}
