use crate::DfxOrbit;
use anyhow::{anyhow, bail, Context};
use candid::{Nat, Principal};
use ic_certified_assets::types::CommitProposedBatchArguments;
use ic_utils::{call::AsyncCaller, Canister};
use serde_bytes::ByteBuf;
use slog::Logger;
use station_api::{GetRequestResponse, RequestOperationDTO};
use std::path::Path;

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

    pub(super) async fn validate_commit_proposed_batch(
        &self,
        batch_id: Nat,
        evidence: String,
    ) -> anyhow::Result<()> {
        let evidence = hex::decode(evidence)?;
        let arg = CommitProposedBatchArguments {
            batch_id,
            evidence: ByteBuf::from(evidence),
        };

        let method: AsyncCaller<'_, (Result<String, String>,)> = self
            .canister_agent
            .update("validate_commit_proposed_batch")
            .with_arg(arg)
            .build();

        let result: Result<String, String> = method
            .call_and_wait_one()
            .await
            .with_context(|| "Failed to fetch current upload proposal from asset canister")?;
        result.map_err(|str| anyhow!(str))?;

        Ok(())
    }
}
