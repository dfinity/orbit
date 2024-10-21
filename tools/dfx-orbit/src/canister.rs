use crate::DfxOrbit;
use clap::{Parser, Subcommand};
use station_api::{GetRequestResponse, RequestOperationInput};

mod call;
mod install;
mod settings;
mod util;

pub use self::{
    call::RequestCanisterCallArgs, install::CanisterInstallModeArgs,
    install::RequestCanisterInstallArgs, settings::RequestCanisterUpdateSettingsArgs,
};

// TODO: Support Canister create + integration test
// TODO: Canister get response functionality

/// Request canister operations through Orbit
#[derive(Debug, Clone, Parser)]
pub struct RequestCanisterArgs {
    /// The operation to request
    #[clap(subcommand)]
    pub action: RequestCanisterActionArgs,
}

#[derive(Debug, Clone, Subcommand)]
#[clap(version, about, long_about = None)]
pub enum RequestCanisterActionArgs {
    /// Request to upgrade the canister wasm
    Install(RequestCanisterInstallArgs),
    /// Request to call a canister method
    Call(RequestCanisterCallArgs),
    /// Update a canister's settings (i.e its controller, compute allocation, or memory allocation.)
    UpdateSettings(RequestCanisterUpdateSettingsArgs),
}

impl RequestCanisterArgs {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    pub(crate) async fn into_request(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        self.action.into_request(dfx_orbit).await
    }
}

impl RequestCanisterActionArgs {
    /// Converts the CLI arg type into the equivalent Orbit API type.
    pub(crate) async fn into_request(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        match self {
            RequestCanisterActionArgs::Install(args) => args.into_request(dfx_orbit).await,
            RequestCanisterActionArgs::Call(args) => args.into_request(dfx_orbit),
            RequestCanisterActionArgs::UpdateSettings(args) => args.into_request(dfx_orbit).await,
        }
    }
}

#[derive(Debug, Clone, Parser)]
pub struct VerifyCanisterArgs {
    /// The operation to verify
    #[clap(subcommand)]
    pub action: VerifyCanisterActionArgs,
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
