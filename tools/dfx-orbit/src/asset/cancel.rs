use super::util::verify_call;
use crate::DfxOrbit;
use candid::Nat;
use clap::Parser;
use ic_certified_assets::types::DeleteBatchArguments;
use station_api::{GetRequestResponse, RequestOperationInput};

#[derive(Debug, Clone, Parser)]
pub struct RequestAssetCancelUploadArgs {
    /// The name of the asset canister targeted by this action
    pub canister: String,

    /// The batch ID to cancel
    #[clap(short, long)]
    pub batch_id: Nat,
}

impl RequestAssetCancelUploadArgs {
    pub(super) fn into_request(
        self,
        dfx_orbit: &DfxOrbit,
    ) -> anyhow::Result<RequestOperationInput> {
        let canister_id = dfx_orbit.canister_id(&self.canister)?;
        DfxOrbit::cancel_batch_input(canister_id, self.batch_id)
    }

    pub(crate) fn verify(
        &self,
        dfx_orbit: &DfxOrbit,
        request: &GetRequestResponse,
    ) -> anyhow::Result<()> {
        let asset_canister = dfx_orbit.canister_id(&self.canister)?;
        let args = DeleteBatchArguments {
            batch_id: self.batch_id.clone(),
        };
        let encoded_args: String = hex::encode(candid::encode_one(args)?);

        verify_call(
            request,
            &asset_canister,
            "delete_batch",
            &Some(encoded_args),
        )?;
        Ok(())
    }
}
