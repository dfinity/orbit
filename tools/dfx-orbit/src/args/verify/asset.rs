use crate::{
    args::request::asset::{RequestAssetCancelUploadArgs, RequestAssetPermissionArgs},
    DfxOrbit,
};
use candid::Nat;
use clap::{Parser, Subcommand};
use station_api::GetRequestResponse;

#[derive(Debug, Clone, Parser)]
pub struct VerifyAssetArgs {
    /// The operation to verify
    #[clap(subcommand)]
    pub action: VerifyAssetActionArgs,
}

#[derive(Debug, Clone, Subcommand)]
#[clap(version, about, long_about = None)]
pub enum VerifyAssetActionArgs {
    /// Request to grant a user permissions for an asset canister
    Permission(RequestAssetPermissionArgs),
    /// Upload assets to an asset canister
    Upload(VerifyAssetUploadArgs),
    /// Cancel an uppload
    CancelUpload(RequestAssetCancelUploadArgs),
}

impl VerifyAssetArgs {
    pub(crate) async fn verify(
        &self,
        dfx_orbit: &DfxOrbit,
        request: &GetRequestResponse,
    ) -> anyhow::Result<()> {
        match &self.action {
            VerifyAssetActionArgs::Upload(args) => args.verify(dfx_orbit, request).await?,
            VerifyAssetActionArgs::Permission(args) => args.verify(dfx_orbit, request)?,
            VerifyAssetActionArgs::CancelUpload(args) => args.verify(dfx_orbit, request)?,
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Parser)]
pub struct VerifyAssetUploadArgs {
    /// The name of the asset canister targeted by this action
    pub canister: String,

    /// The batch ID to commit to
    #[clap(short, long)]
    pub batch_id: Nat,

    /// The source directories to upload
    /// (multiple values possible, picks up sources from dfx.json by default)
    #[clap(short, long)]
    pub files: Vec<String>,
}

impl VerifyAssetUploadArgs {
    async fn verify(
        &self,
        dfx_orbit: &DfxOrbit,
        request: &GetRequestResponse,
    ) -> anyhow::Result<()> {
        let pathbufs = dfx_orbit.as_path_bufs(&self.canister, &self.files)?;
        let paths = DfxOrbit::as_paths(&pathbufs);

        let canister_id = dfx_orbit.canister_id(&self.canister)?;
        let asset_agent = dfx_orbit.asset_agent(canister_id)?;

        let evidence = asset_agent.compute_evidence(&paths).await?;

        println!("Computed evidence: 0x{evidence}");
        DfxOrbit::check_evidence(request, canister_id, self.batch_id.clone(), evidence)?;

        Ok(())
    }
}
