use crate::DfxOrbit;
use clap::{Parser, Subcommand};
use orbit_station_api::RequestOperationInput;

#[derive(Debug, Clone, Parser)]
pub struct RequestAssetArgs {
    /// The operation to request
    #[clap(subcommand)]
    pub(crate) action: RequestAssetActionArgs,
}

#[derive(Debug, Clone, Subcommand)]
#[clap(version, about, long_about = None)]
pub enum RequestAssetActionArgs {
    /// Request to grant this user Prepare permission for the asset canister
    PreparePermission(RequestAssetPreparePermissionArgs),
    // Upload assets to an asset canister
    //Upload(AssetUploadArgs),
}

impl RequestAssetArgs {
    pub(crate) async fn into_create_request_input(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        match self.action {
            RequestAssetActionArgs::PreparePermission(args) => {
                args.into_create_request_input(dfx_orbit)
            }
        }
    }
}

#[derive(Debug, Clone, Parser)]
pub struct RequestAssetPreparePermissionArgs {
    /// The name of the asset canister targeted by this action
    pub(crate) canister: String,
}

impl RequestAssetPreparePermissionArgs {
    pub(crate) fn into_create_request_input(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        let me = dfx_orbit.own_principal()?;
        let asset_canister = dfx_orbit.canister_id(&self.canister)?;
        DfxOrbit::grant_permission_request(asset_canister, me)
    }
}
