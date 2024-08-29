use crate::DfxOrbit;
use clap::{Parser, Subcommand};
use station_api::RequestOperationInput;

#[derive(Debug, Clone, Parser)]
pub struct RequestAssetArgs {
    #[clap(subcommand)]
    pub(crate) action: RequestAssetActionArgs,
}

#[derive(Debug, Clone, Subcommand)]
#[clap(version, about, long_about = None)]
pub enum RequestAssetActionArgs {
    /// Request to grant this user Prepare permission for the asset canister
    PreparePermission(RequestAssetPreparePermissionArgs),
    /// Upload assets to an asset canister
    Upload(RequestAssetUploadArgs),
}

impl RequestAssetArgs {
    pub(crate) async fn into_create_request_input(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        match self.action {
            RequestAssetActionArgs::PreparePermission(args) => {
                args.into_create_request_input(dfx_orbit)
            }
            RequestAssetActionArgs::Upload(args) => args.into_create_request_input(dfx_orbit).await,
        }
    }
}

#[derive(Debug, Clone, Parser)]
pub struct RequestAssetPreparePermissionArgs {
    /// The name of the asset canister targeted by this action
    pub(crate) canister: String,
}

impl RequestAssetPreparePermissionArgs {
    pub(crate) fn into_create_request_input(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        let me = dfx_orbit.own_principal()?;
        let asset_canister = dfx_orbit.canister_id(&self.canister)?;
        DfxOrbit::grant_permission_request(asset_canister, me)
    }
}

#[derive(Debug, Clone, Parser)]
pub struct RequestAssetUploadArgs {
    /// The name of the asset canister targeted by this action
    pub(crate) canister: String,

    /// Do not abort the upload, if the evidence does not match between local and remote calculation
    #[clap(long)]
    pub(crate) ignore_evidence: bool,

    /// The source directories to upload (multiple values possible)
    pub(crate) files: Vec<String>,
}

impl RequestAssetUploadArgs {
    pub(crate) async fn into_create_request_input(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        let pathbufs = dfx_orbit.as_path_bufs(&self.canister, &self.files)?;
        let paths = DfxOrbit::as_paths(&pathbufs);
        let canister_id = dfx_orbit.canister_id(&self.canister)?;

        let (batch_id, evidence) = dfx_orbit
            .upload(canister_id, &paths, self.ignore_evidence)
            .await?;
        println!("Batch id: {batch_id}");
        println!("Evidence: {}", hex::encode(&evidence));

        DfxOrbit::commit_batch_input(canister_id, batch_id, evidence)
    }
}
