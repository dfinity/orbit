use crate::DfxOrbit;
use candid::{Nat, Principal};
use ic_asset::canister_api::{
    methods::batch::compute_evidence, types::batch_upload::common::ComputeEvidenceArguments,
};
use serde_bytes::ByteBuf;
use std::path::Path;

impl DfxOrbit {
    pub async fn request_evidence(
        &self,
        canister_id: Principal,
        batch_id: Nat,
    ) -> anyhow::Result<ByteBuf> {
        let canister_agent = self.canister_agent(canister_id)?;
        // This part is stolen from ic_asset::sync::prepare_sync_for_proposal.  Unfortunately the relevant functions are private.
        // The docs explicitly include waiting for the evidence so this should really be made easier!  See: https://github.com/dfinity/sdk/blob/2509e81e11e71dce4045c679686c952809525470/docs/design/asset-canister-interface.md?plain=1#L85
        let compute_evidence_arg = ComputeEvidenceArguments {
            batch_id: batch_id.clone(),
            max_iterations: Some(97), // 75% of max(130) = 97.5
        };
        let canister_evidence_bytes = loop {
            if let Some(evidence) = compute_evidence(&canister_agent, &compute_evidence_arg).await?
            {
                break evidence;
            }
        };
        Ok(canister_evidence_bytes)
    }

    pub async fn compute_evidence(
        &self,
        canister_id: Principal,
        sources: &[&Path],
    ) -> anyhow::Result<String> {
        let canister_agent = self.canister_agent(canister_id)?;
        Ok(ic_asset::compute_evidence(&canister_agent, sources, &self.logger).await?)
    }
}
