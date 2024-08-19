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
    /// Manage assets stored in an asset canister through Orbit
    Asset(VerifyAssetArgs),
}

impl VerifyArgs {
    pub(crate) async fn verify(self, dfx_orbit: &DfxOrbit) -> anyhow::Result<()> {
        let request = dfx_orbit
            .station
            .review_id(GetRequestInput {
                request_id: self.request_id,
            })
            .await?;

        match self.action {
            VerifyArgsAction::Asset(args) => args.verify(dfx_orbit, &request).await?,
        }

        // TODO:
        // if args.then_approve {
        //     dfx_core::cli::ask_for_consent("Do you want to approve the request?")?;
        //     let args = SubmitRequestApprovalInput {
        //         decision: RequestApprovalStatusDTO::Approved,
        //         request_id: args.request_id,
        //         reason: None,
        //     };
        //     self.station.submit(args).await?;
        // }

        Ok(())
    }
}
