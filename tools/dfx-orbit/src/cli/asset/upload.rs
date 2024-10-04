use super::AssetAgent;
use crate::DfxOrbit;
use anyhow::bail;
use candid::{Nat, Principal};
use ic_certified_assets::types::{CommitProposedBatchArguments, DeleteBatchArguments};
use serde_bytes::ByteBuf;
use slog::{info, warn};
use station_api::{CallExternalCanisterOperationInput, CanisterMethodDTO, RequestOperationInput};
use std::path::Path;

impl DfxOrbit {
    pub async fn upload(
        &self,
        canister_id: Principal,
        sources: &[&Path],
        ignore_evidence: bool,
    ) -> anyhow::Result<(Nat, ByteBuf)> {
        let asset_agent = self.asset_agent(canister_id)?;
        let (batch_id, evidence) = asset_agent.upload_assets(sources).await?;

        let remote_evidence = hex::encode(&evidence);
        let local_evidence = asset_agent.compute_evidence(sources).await?;

        if !ignore_evidence {
            if local_evidence != remote_evidence {
                warn!(
                    self.logger,
                    "Local evidence does not match remotely calculated evidence"
                );
                warn!(self.logger, "Local:  {local_evidence}");
                warn!(self.logger, "Remote: {remote_evidence}");
                bail!("Evidence did not match!");
            } else {
                info!(self.logger, "Local and remote evidence match!");
            }
        }

        Ok((batch_id, evidence))
    }

    pub fn commit_batch_input(
        canister_id: Principal,
        batch_id: Nat,
        evidence: ByteBuf,
    ) -> anyhow::Result<RequestOperationInput> {
        let args = CommitProposedBatchArguments { batch_id, evidence };
        let arg = candid::encode_one(args)?;

        Ok(RequestOperationInput::CallExternalCanister(
            CallExternalCanisterOperationInput {
                validation_method: None,
                execution_method: CanisterMethodDTO {
                    canister_id,
                    method_name: String::from("commit_proposed_batch"),
                },
                arg: Some(arg),
                execution_method_cycles: None,
            },
        ))
    }

    pub fn cancel_batch_input(
        canister_id: Principal,
        batch_id: Nat,
    ) -> anyhow::Result<RequestOperationInput> {
        let args = DeleteBatchArguments { batch_id };
        let arg = candid::encode_one(args)?;

        Ok(RequestOperationInput::CallExternalCanister(
            CallExternalCanisterOperationInput {
                validation_method: None,
                execution_method: CanisterMethodDTO {
                    canister_id,
                    method_name: String::from("delete_batch"),
                },
                arg: Some(arg),
                execution_method_cycles: None,
            },
        ))
    }
}

impl AssetAgent<'_> {
    pub async fn upload_assets(&self, sources: &[&Path]) -> anyhow::Result<(Nat, ByteBuf)> {
        Ok(
            ic_asset::prepare_sync_for_proposal(&self.canister_agent, sources, &self.logger)
                .await?,
        )
    }
}
