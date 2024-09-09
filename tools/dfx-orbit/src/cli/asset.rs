//! Implements the `dfx-orbit canister upload-http-assets` CLI command.

mod evidence;
mod upload;
mod util;

use crate::DfxOrbit;
use candid::Principal;
use ic_utils::Canister;
use slog::Logger;

pub struct AssetAgent<'agent> {
    canister_agent: Canister<'agent>,
    logger: Logger,
}

impl DfxOrbit {
    pub fn asset_agent(&self, canister_id: Principal) -> anyhow::Result<AssetAgent> {
        Ok(AssetAgent {
            canister_agent: self.canister_agent(canister_id)?,
            logger: self.logger.clone(),
        })
    }
}
