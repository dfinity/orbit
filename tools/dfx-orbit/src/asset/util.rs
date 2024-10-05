use std::path::Path;

use anyhow::bail;
use candid::{Nat, Principal};
use ic_utils::Canister;
use serde_bytes::ByteBuf;
use slog::Logger;
use station_api::{GetRequestResponse, RequestOperationDTO};

use crate::DfxOrbit;

pub(super) fn verify_call(
    request: &GetRequestResponse,
    expected_canister_id: &Principal,
    expected_method: &str,
    expected_arg_checksum: &Option<String>,
) -> anyhow::Result<()> {
    let RequestOperationDTO::CallExternalCanister(operation) = &request.request.operation else {
        bail!("The request is not a call external canister request");
    };
    if &operation.execution_method.canister_id != expected_canister_id {
        bail!(
            "The request targets an unexpected canister. Expected: {}, actual: {}",
            expected_canister_id,
            operation.execution_method.canister_id
        );
    }
    if operation.execution_method.method_name != expected_method {
        bail!(
            "The method of this request is not \"{}\" but \"{}\" instead",
            expected_method,
            operation.execution_method.method_name
        );
    }
    if &operation.arg_checksum != expected_arg_checksum {
        bail!("Argument checksum does not match");
    }

    Ok(())
}

pub(super) struct AssetAgent<'agent> {
    canister_agent: Canister<'agent>,
    logger: Logger,
}

impl DfxOrbit {
    pub(super) fn asset_agent(&self, canister_id: Principal) -> anyhow::Result<AssetAgent> {
        Ok(AssetAgent {
            canister_agent: self.canister_agent(canister_id)?,
            logger: self.logger.clone(),
        })
    }
}

impl AssetAgent<'_> {
    pub(super) async fn upload_assets(&self, sources: &[&Path]) -> anyhow::Result<(Nat, ByteBuf)> {
        Ok(
            ic_asset::prepare_sync_for_proposal(&self.canister_agent, sources, &self.logger)
                .await?,
        )
    }

    pub(super) async fn compute_evidence(&self, sources: &[&Path]) -> anyhow::Result<String> {
        Ok(ic_asset::compute_evidence(&self.canister_agent, sources, &self.logger).await?)
    }
}
