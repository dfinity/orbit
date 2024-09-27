mod asset;
mod canister;

use crate::DfxOrbit;
pub use asset::{VerifyAssetActionArgs, VerifyAssetArgs, VerifyAssetUploadArgs};
pub use canister::{VerifyCanisterActionArgs, VerifyCanisterArgs};
use clap::{Parser, Subcommand};
use station_api::GetRequestResponse;

#[derive(Debug, Clone, Parser)]
pub struct VerifyArgs {
    /// The ID of the request to verify
    pub request_id: String,

    /// Approve the request, if the validation succeeds
    #[clap(short = 'a', long)]
    pub and_approve: bool,
    /// Reject the request, if the validation fails
    #[clap(short = 'r', long)]
    pub or_reject: bool,

    /// The type of request to verify
    #[clap(subcommand)]
    pub action: VerifyArgsAction,
}

impl std::fmt::Display for VerifyArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "verify {}", self.request_id)?;
        if self.and_approve {
            write!(f, " --and-approve")?;
        }
        if self.or_reject {
            write!(f, " --or-reject")?;
        }
        write!(f, " {}", self.action)?;

        Ok(())
    }
}

impl VerifyArgs {
    pub async fn verify(
        &self,
        dfx_orbit: &DfxOrbit,
        request: &GetRequestResponse,
    ) -> anyhow::Result<()> {
        // TODO: Don't allow non-pending requests to be verified, since they might no longer be
        // verifiable after the execution

        match &self.action {
            VerifyArgsAction::Asset(args) => args.verify(dfx_orbit, request).await?,
            VerifyArgsAction::Canister(args) => args.verify(dfx_orbit, request).await?,
        };

        Ok(())
    }

    pub(crate) async fn conditionally_execute_actions(
        &self,
        dfx_orbit: &DfxOrbit,
        verified: anyhow::Result<()>,
    ) -> anyhow::Result<()> {
        match verified {
            Ok(()) => {
                if self.and_approve {
                    dfx_core::cli::ask_for_consent(
                        "Verification successful, approve the request?",
                    )?;
                    dfx_orbit
                        .station
                        .approve(self.request_id.clone(), None)
                        .await?;
                } else {
                    println!("Verification successful!");
                }
            }
            Err(err) => {
                if self.or_reject {
                    dfx_core::cli::ask_for_consent(&format!(
                        "Verification failed: {err}. Reject the request?"
                    ))?;
                    dfx_orbit
                        .station
                        .reject(self.request_id.clone(), None)
                        .await?;
                } else {
                    println!("Verification failed!");
                };

                return Err(err);
            }
        };

        Ok(())
    }
}

#[derive(Debug, Clone, Subcommand)]
pub enum VerifyArgsAction {
    /// Manage assets stored in an asset canister through Orbit
    Asset(VerifyAssetArgs),
    /// Request canister operations through Orbit
    Canister(VerifyCanisterArgs),
}

impl std::fmt::Display for VerifyArgsAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VerifyArgsAction::Asset(args) => write!(f, "asset {}", args),
            VerifyArgsAction::Canister(args) => write!(f, "canister {}", args),
        }
    }
}
