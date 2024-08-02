use crate::DfxOrbit;

use super::AssetAgent;
use std::path::Path;

impl DfxOrbit {
    pub async fn check_evidence(
        &self,
        _request_id: String,
        _sources: &[&Path],
    ) -> anyhow::Result<bool> {
        todo!()
    }
}

impl AssetAgent<'_> {
    pub async fn compute_evidence(&self, sources: &[&Path]) -> anyhow::Result<String> {
        Ok(ic_asset::compute_evidence(&self.canister_agent, sources, &self.logger).await?)
    }
}
