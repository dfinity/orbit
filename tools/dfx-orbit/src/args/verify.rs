mod asset;
mod canister;

use crate::DfxOrbit;
use asset::VerifyAssetArgs;
use canister::VerifyCanisterArgs;
use clap::{Parser, Subcommand};
use orbit_station_api::{GetRequestInput, RequestApprovalStatusDTO, SubmitRequestApprovalInput};

#[derive(Debug, Clone, Parser)]
pub struct VerifyArgs {
    /// The ID of the request to verify
    pub(crate) request_id: String,

    /// Approve the request, if the validation succeeds
    #[clap(
        short = 'a',
        long,
        action,
        value_name = "REASON",
        default_missing_value = "None"
    )]
    pub(crate) and_approve: Option<Option<String>>,
    /// Reject the request, if the validation fails
    #[clap(
        short = 'r',
        long,
        action,
        value_name = "REASON",
        default_missing_value = "None"
    )]
    pub(crate) or_reject: Option<Option<String>>,

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
        let request = dfx_orbit
            .station
            .review_id(GetRequestInput {
                request_id: self.request_id.clone(),
            })
            .await?;

        // TODO: Don't allow non-pending requests to be verified, since they might no longer be
        // verifiable after the execution

        let verified = match self.action {
            VerifyArgsAction::Asset(args) => args.verify(dfx_orbit, &request).await,
            VerifyArgsAction::Canister(args) => args.verify(dfx_orbit, &request),
        };

        match verified {
            Ok(()) => {
                dfx_core::cli::ask_for_consent("Do you want to approve the request?")?;
                if let Some(reason) = self.and_approve {
                    dfx_orbit
                        .station
                        .submit(SubmitRequestApprovalInput {
                            decision: RequestApprovalStatusDTO::Approved,
                            request_id: self.request_id,
                            reason,
                        })
                        .await?;
                }
            }
            Err(err) => {
                dfx_core::cli::ask_for_consent("Do you want to reject the request?")?;
                if let Some(reason) = self.or_reject {
                    dfx_orbit
                        .station
                        .submit(SubmitRequestApprovalInput {
                            decision: RequestApprovalStatusDTO::Rejected,
                            request_id: self.request_id,
                            reason,
                        })
                        .await?;
                };

                return Err(err);
            }
        }

        Ok(())
    }
}
