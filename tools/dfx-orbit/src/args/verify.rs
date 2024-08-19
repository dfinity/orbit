pub mod asset;

use asset::VerifyAssetArgs;
use clap::{Parser, Subcommand};
use orbit_station_api::GetRequestInput;

use crate::DfxOrbit;

/// Station management commands.
#[derive(Debug, Clone, Parser)]
#[clap(version, about, long_about = None)]
pub struct VerifyArgs {
    /// The ID of the request to verify
    pub(crate) request_id: String,

    // TODO: Auto approve / reject
    /// The type of request to verify
    #[clap(subcommand)]
    pub(crate) action: VerifyArgsAction,
}

#[derive(Debug, Clone, Subcommand)]
pub enum VerifyArgsAction {
    /// Verify orbit actions with regards to assets
    Asset(VerifyAssetArgs),
}

impl VerifyArgs {
    async fn verify(self, dfx_orbit: &DfxOrbit) -> anyhow::Result<()> {
        let request = dfx_orbit
            .station
            .review_id(GetRequestInput {
                request_id: self.request_id,
            })
            .await?;

        match self.action {
            VerifyArgsAction::Asset(args) => args.verify(&dfx_orbit, &request).await?,
        }

        Ok(())
    }
}
