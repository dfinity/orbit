use crate::{args::request::asset::RequestAssetPermissionArgs, DfxOrbit};
use candid::Nat;
use clap::{Parser, Subcommand};
use station_api::GetRequestResponse;

#[derive(Debug, Clone, Parser)]
pub struct VerifyAssetArgs {
    /// The operation to verify
    #[clap(subcommand)]
    pub action: VerifyAssetActionArgs,
}

impl std::fmt::Display for VerifyAssetArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.action {
            VerifyAssetActionArgs::Upload(args) => write!(f, "upload {}", args),
            VerifyAssetActionArgs::Permission(args) => write!(f, "permission {}", args),
        }
    }
}

#[derive(Debug, Clone, Subcommand)]
#[clap(version, about, long_about = None)]
pub enum VerifyAssetActionArgs {
    /// Upload assets to an asset canister
    Upload(VerifyAssetUploadArgs),
    /// Request to grant a user permissions for an asset canister
    Permission(RequestAssetPermissionArgs),
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
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Parser)]
pub struct VerifyAssetUploadArgs {
    /// The name of the asset canister targeted by this action
    pub canister: String,

    /// The batch ID to commit to
    pub batch_id: Nat,

    /// The source directories of the asset upload (multiple values possible)
    pub files: Vec<String>,
}

impl std::fmt::Display for VerifyAssetUploadArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "canister {}", self.canister)?;
        write!(f, " batch_id {}", self.batch_id)?;
        for file in &self.files {
            write!(f, " --files {}", file)?;
        }

        Ok(())
    }
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
