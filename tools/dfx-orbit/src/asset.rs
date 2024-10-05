use crate::DfxOrbit;
use clap::{Parser, Subcommand};
use station_api::{GetRequestResponse, RequestOperationInput};

mod cancel;
mod commit;
mod permission;
mod upload;
mod util;

pub use self::{
    cancel::RequestAssetCancelUploadArgs,
    commit::RequestAssetCommitArgs,
    permission::RequestAssetPermissionArgs,
    upload::{RequestAssetUploadArgs, VerifyAssetUploadArgs},
};

#[derive(Debug, Clone, Parser)]
pub struct RequestAssetArgs {
    #[clap(subcommand)]
    pub action: RequestAssetActionArgs,
}

#[derive(Debug, Clone, Subcommand)]
#[clap(version, about, long_about = None)]
pub enum RequestAssetActionArgs {
    /// Request to grant a user permissions for an asset canister
    Permission(RequestAssetPermissionArgs),
    /// Upload assets to an asset canister, and then request to commit to it
    Upload(RequestAssetUploadArgs),
    /// Commit to an already prepared batch
    Commit(RequestAssetCommitArgs),
    /// Cancel an upload
    CancelUpload(RequestAssetCancelUploadArgs),
}

impl RequestAssetArgs {
    pub(crate) async fn into_request(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        match self.action {
            RequestAssetActionArgs::Permission(args) => args.into_request(dfx_orbit),
            RequestAssetActionArgs::Upload(args) => args.into_request(dfx_orbit).await,
            RequestAssetActionArgs::Commit(args) => args.into_request(dfx_orbit).await,
            RequestAssetActionArgs::CancelUpload(args) => args.into_request(dfx_orbit),
        }
    }
}

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
