use crate::core::ic_cdk::api::time;
use crate::core::{canister_config, write_canister_config, CanisterConfig};
use control_panel_api::{CanisterInit, CanisterUpgrade};
use ic_canister_core::api::ServiceResult;
use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    pub static ref CANISTER_SERVICE: Arc<CanisterService> = Arc::new(CanisterService::new());
}

#[derive(Default, Debug)]
pub struct CanisterService;

impl CanisterService {
    pub fn new() -> Self {
        Self
    }

    pub async fn init_canister(&self, input: CanisterInit) -> ServiceResult<()> {
        let mut config = CanisterConfig::new(input.upgrader_wasm_module, input.wallet_wasm_module);

        config.last_upgrade_timestamp = time();

        write_canister_config(config);

        Ok(())
    }

    pub async fn upgrade_canister(&self, input: CanisterUpgrade) -> ServiceResult<()> {
        let mut config = canister_config();

        if let Some(upgrader_wasm_module) = input.upgrader_wasm_module {
            config.upgrader_wasm_module = upgrader_wasm_module;
        }

        if let Some(wallet_wasm_module) = input.wallet_wasm_module {
            config.wallet_wasm_module = wallet_wasm_module;
        }

        config.last_upgrade_timestamp = time();

        write_canister_config(config);

        Ok(())
    }
}
