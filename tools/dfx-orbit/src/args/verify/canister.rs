use clap::{Parser, Subcommand};
use station_api::GetRequestResponse;

use crate::{
    args::request::canister::{
        RequestCanisterCallArgs, RequestCanisterInstallArgs, RequestCanisterUpdateSettingsArgs,
    },
    DfxOrbit,
};

#[derive(Debug, Clone, Parser)]
pub struct VerifyCanisterArgs {
    /// The operation to verify
    #[clap(subcommand)]
    pub action: VerifyCanisterActionArgs,
}

impl std::fmt::Display for VerifyCanisterArgs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.action {
            VerifyCanisterActionArgs::Install(args) => write!(f, "install {}", args),
            VerifyCanisterActionArgs::Call(args) => write!(f, "call {}", args),
            VerifyCanisterActionArgs::UpdateSettings(args) => {
                write!(f, "update-settings {}", args)
            }
        }
    }
}

#[derive(Debug, Clone, Subcommand)]
#[clap(version, about, long_about = None)]
pub enum VerifyCanisterActionArgs {
    /// Verify upgrade the canister wasm
    Install(RequestCanisterInstallArgs),
    /// Verify call a canister method
    Call(RequestCanisterCallArgs),
    /// Verify an update settings request
    UpdateSettings(RequestCanisterUpdateSettingsArgs),
}

impl VerifyCanisterArgs {
    pub(crate) async fn verify(
        &self,
        dfx_orbit: &DfxOrbit,
        request: &GetRequestResponse,
    ) -> anyhow::Result<()> {
        match &self.action {
            VerifyCanisterActionArgs::Install(args) => args.verify(dfx_orbit, request)?,
            VerifyCanisterActionArgs::Call(args) => args.verify(dfx_orbit, request)?,
            VerifyCanisterActionArgs::UpdateSettings(args) => {
                args.verify(dfx_orbit, request).await?
            }
        }

        Ok(())
    }
}
