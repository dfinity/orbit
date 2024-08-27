mod asset;
mod canister;

use crate::DfxOrbit;
use asset::VerifyAssetArgs;
use canister::VerifyCanisterArgs;
use clap::{Parser, Subcommand};
use station_api::GetRequestInput;

#[derive(Debug, Clone, Parser)]
pub struct VerifyArgs {
    /// The ID of the request to verify
    pub(crate) request_id: String,

    /// Approve the request, if the validation succeeds
    #[clap(short = 'a', long)]
    pub(crate) and_approve: bool,
    /// Reject the request, if the validation fails
    #[clap(short = 'r', long)]
    pub(crate) or_reject: bool,

    /// The type of request to verify
    #[clap(subcommand)]
    pub(crate) action: VerifyArgsAction,
}

#[derive(Debug, Clone, Subcommand)]
pub enum VerifyArgsAction {
    /// Manage assets stored in an asset canister through Orbit
    Asset(VerifyAssetArgs),
    /// Request canister operations through Orbit
    Canister(VerifyCanisterArgs),
}

impl VerifyArgs {
    pub(crate) async fn verify(self, dfx_orbit: &DfxOrbit) -> anyhow::Result<()> {
        // TODO: Move fetching the request and displaying it up a level
        let request = dfx_orbit
            .station
            .review_id(GetRequestInput {
                request_id: self.request_id.clone(),
            })
            .await?;

        println!(
            "{}",
            dfx_orbit.display_get_request_response(request.clone())
        );
        // TODO: Don't allow non-pending requests to be verified, since they might no longer be
        // verifiable after the execution

        let verified = match self.action {
            VerifyArgsAction::Asset(args) => args.verify(dfx_orbit, &request).await,
            VerifyArgsAction::Canister(args) => args.verify(dfx_orbit, &request),
        };

        match verified {
            Ok(()) => {
                println!("Verification successful!");
                if self.and_approve {
                    dfx_core::cli::ask_for_consent("Do you want to approve the request?")?;
                    dfx_orbit.station.approve(self.request_id, None).await?;
                }
            }
            Err(err) => {
                println!("Verification failed: {err}");
                if self.or_reject {
                    dfx_core::cli::ask_for_consent("Do you want to reject the request?")?;
                    dfx_orbit.station.reject(self.request_id, None).await?;
                };

                return Err(err);
            }
        }

        Ok(())
    }
}
