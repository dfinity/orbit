use candid::Nat;
use clap::{Parser, Subcommand};
use orbit_station_api::GetRequestResponse;

use crate::DfxOrbit;

#[derive(Debug, Clone, Parser)]
pub struct VerifyAssetArgs {
    /// The operation to verify
    #[clap(subcommand)]
    pub(crate) action: VerifyAssetActionArgs,
}

#[derive(Debug, Clone, Subcommand)]
#[clap(version, about, long_about = None)]
pub enum VerifyAssetActionArgs {
    /// Upload assets to an asset canister
    Upload(VerifyAssetUploadArgs),
}

impl VerifyAssetArgs {
    pub(crate) async fn verify(
        self,
        dfx_orbit: &DfxOrbit,
        request: &GetRequestResponse,
    ) -> anyhow::Result<()> {
        match self.action {
            VerifyAssetActionArgs::Upload(args) => args.verify(dfx_orbit, request).await?,
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Parser)]
pub struct VerifyAssetUploadArgs {
    /// The name of the asset canister targeted by this action
    pub(crate) canister: String,

    /// The batch ID to commit to
    pub(crate) batch_id: Nat,

    /// The source directories of the asset upload (multiple values possible)
    pub(crate) files: Vec<String>,
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

        DfxOrbit::check_evidence(request, canister_id, self.batch_id.clone(), evidence)?;

        Ok(())
    }
}
