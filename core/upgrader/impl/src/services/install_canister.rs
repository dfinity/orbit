use crate::model::InstallMode;
use async_trait::async_trait;
use candid::Principal;
use ic_cdk::api::management_canister::main::{self as mgmt, CanisterIdRecord};
use lazy_static::lazy_static;
use orbit_essentials::install_chunked_code::install_chunked_code;
use orbit_essentials::types::WasmModuleExtraChunks;
use std::{collections::BTreeSet, sync::Arc};

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
        wasm_module_extra_chunks: Option<WasmModuleExtraChunks>,
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
        wasm_module_extra_chunks: Option<WasmModuleExtraChunks>,
        arg: Vec<u8>,
        mode: InstallMode,
    ) -> Result<(), String> {
        // For install and reinstall, we need to make the station self controlled.
        match mode {
            InstallMode::Install | InstallMode::Reinstall => {
                // Get the current controllers and add the station to it. We preserve the existing controllers
                // during this step to avoid losing control in case of a failed install.
                let (info,) = mgmt::canister_info(mgmt::CanisterInfoRequest {
                    canister_id,
                    num_requested_changes: None,
                })
                .await
                .map_err(|(code, err)| {
                    format!(
                        "failed to get canister info for canister: \"{}\", rejection code: {}",
                        err, code as i32
                    )
                })?;

                let mut controllers = BTreeSet::new();
                controllers.extend(info.controllers.iter().cloned());
                controllers.insert(canister_id);
                controllers.insert(ic_cdk::id());

                mgmt::update_settings(mgmt::UpdateSettingsArgument {
                    canister_id,
                    settings: mgmt::CanisterSettings {
                        controllers: Some(controllers.into_iter().collect()),
                        ..Default::default()
                    },
                })
                .await
                .map_err(|(code, err)| {
                    format!(
                        "failed to update settings for canister: \"{}\", rejection code: {}",
                        err, code as i32
                    )
                })?;
            }
            InstallMode::Upgrade => {
                // For upgrade, there are no controller changes needed.
            }
        }

        install_chunked_code(
            canister_id,
            mode.into(),
            wasm_module,
            wasm_module_extra_chunks,
            arg,
        )
        .await
        .map_err(|err| format!("failed to {} canister: \"{}\"", mode, err,))
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
