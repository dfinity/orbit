use std::sync::Arc;

use async_trait::async_trait;
use candid::Principal;
use ic_cdk::api::management_canister::main::{self as mgmt, CanisterIdRecord, InstallCodeArgument};
use lazy_static::lazy_static;

use crate::model::InstallMode;

lazy_static! {
    pub static ref INSTALL_CANISTER: Arc<StationDisasterRecoveryInstall> =
        Arc::new(StationDisasterRecoveryInstall::default());
}

#[async_trait]
pub trait InstallCanister: Send + Sync {
    async fn stop(&self, canister_id: Principal) -> Result<(), String>;

    async fn start(&self, canister_id: Principal) -> Result<(), String>;

    async fn install(
        &self,
        canister_id: Principal,
        wasm_module: Vec<u8>,
        arg: Vec<u8>,
        mode: InstallMode,
    ) -> Result<(), String>;
}

#[derive(Clone, Default)]
pub struct StationDisasterRecoveryInstall {}

#[async_trait]
impl InstallCanister for StationDisasterRecoveryInstall {
    async fn install(
        &self,
        canister_id: Principal,
        wasm_module: Vec<u8>,
        arg: Vec<u8>,
        mode: InstallMode,
    ) -> Result<(), String> {
        mgmt::install_code(InstallCodeArgument {
            mode: mode.into(),
            canister_id,
            wasm_module,
            arg,
        })
        .await
        .map_err(|(code, err)| {
            format!(
                "failed to {} canister: \"{}\", rejection code: {}",
                mode, err, code as i32
            )
        })
    }

    async fn start(&self, canister_id: Principal) -> Result<(), String> {
        mgmt::start_canister(CanisterIdRecord { canister_id })
            .await
            .map_err(|(code, err)| {
                format!(
                    "failed to start canister: \"{}\", rejection code: {}",
                    err, code as i32
                )
            })
    }

    async fn stop(&self, canister_id: Principal) -> Result<(), String> {
        mgmt::stop_canister(CanisterIdRecord { canister_id })
            .await
            .map_err(|(code, err)| {
                format!(
                    "failed to stop canister: \"{}\", rejection code: {}",
                    err, code as i32
                )
            })
    }
}
