//! Implements the `dfx-orbit canister upload-http-assets` CLI command.

mod evidence;
mod upload;
mod util;

use crate::{
    args::asset::{
        AssetArgs, AssetArgsAction, AssetCheckArgs, AssetComputeEvidenceArgs,
        AssetReqeustPreparePermissionArgs, AssetUploadArgs,
    },
    DfxOrbit,
};
use candid::Principal;
use ic_utils::Canister;
use orbit_station_api::{RequestApprovalStatusDTO, SubmitRequestApprovalInput};
use slog::Logger;

pub struct AssetAgent<'agent> {
    canister_agent: Canister<'agent>,
    logger: Logger,
}

impl DfxOrbit {
    // TODO: Remove this and all downstream functions that are not needed anymore
    pub async fn exec_asset(&self, args: AssetArgs) -> anyhow::Result<()> {
        match args.action {
            AssetArgsAction::RequestPreparePermission(args) => {
                self.asset_request_prepare_permission(args).await
            }
            AssetArgsAction::Upload(args) => self.asset_upload(args).await,
            AssetArgsAction::ComputeEvidence(args) => self.asset_compute_evidence(args).await,
            AssetArgsAction::Check(args) => self.asset_check(args).await,
        }
    }

    pub fn asset_agent(&self, canister_id: Principal) -> anyhow::Result<AssetAgent> {
        Ok(AssetAgent {
            canister_agent: self.canister_agent(canister_id)?,
            logger: self.logger.clone(),
        })
    }

    async fn asset_request_prepare_permission(
        &self,
        args: AssetReqeustPreparePermissionArgs,
    ) -> anyhow::Result<()> {
        let canister_id = self.canister_id(&args.canister)?;
        let request = self
            .request_prepare_permission(canister_id, args.title, args.summary)
            .await?;
        self.print_create_request_info(&request);

        Ok(())
    }

    async fn asset_upload(&self, args: AssetUploadArgs) -> anyhow::Result<()> {
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
        println!(
            "Created request to commit batches. To verify the batch against local files, run:"
        );
        println!("dfx-orbit asset check {canister_name} {request_id} {batch_id} {files}");

        Ok(())
    }

    async fn asset_compute_evidence(
        &self,
        args: AssetComputeEvidenceArgs,
    ) -> Result<(), anyhow::Error> {
        let pathbufs = self.as_path_bufs(&args.canister, &args.files)?;
        let paths = Self::as_paths(&pathbufs);

        let canister_id = self.canister_id(&args.canister)?;
        let asset_agent = self.asset_agent(canister_id)?;

        let evidence = asset_agent.compute_evidence(&paths).await?;
        println!("{evidence}");
        Ok(())
    }

    async fn asset_check(&self, args: AssetCheckArgs) -> Result<(), anyhow::Error> {
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
