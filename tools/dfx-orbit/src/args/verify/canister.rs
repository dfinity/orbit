use clap::{Parser, Subcommand};
use orbit_station_api::GetRequestResponse;

use crate::{
    args::request::canister::{RequestCanisterCallArgs, RequestCanisterInstallArgs},
    DfxOrbit,
};

#[derive(Debug, Clone, Parser)]
pub struct VerifyCanisterArgs {
    /// The operation to verify
    #[clap(subcommand)]
    pub(crate) action: VerifyCanisterActionArgs,
}

#[derive(Debug, Clone, Subcommand)]
#[clap(version, about, long_about = None)]
pub enum VerifyCanisterActionArgs {
    /// Verify upgrade the canister wasm
    Install(RequestCanisterInstallArgs),
    ///Verify call a canister method
    Call(RequestCanisterCallArgs),
}

impl VerifyCanisterArgs {
    pub(crate) fn verify(
        self,
        dfx_orbit: &DfxOrbit,
        request: &GetRequestResponse,
    ) -> anyhow::Result<()> {
        match self.action {
            VerifyCanisterActionArgs::Install(args) => args.verify(dfx_orbit, request)?,
            VerifyCanisterActionArgs::Call(args) => args.verify(dfx_orbit, request)?,
        }

        Ok(())
    }
}
