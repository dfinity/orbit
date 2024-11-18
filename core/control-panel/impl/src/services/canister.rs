use crate::core::ic_cdk::api::{print, time};
use crate::core::{canister_config, write_canister_config, CallContext};
use crate::errors::CanisterError;
use crate::repositories::{USER_REPOSITORY};
use crate::SYSTEM_VERSION;
use control_panel_api::UploadCanisterModulesInput;
use lazy_static::lazy_static;
use orbit_essentials::api::ServiceResult;
use orbit_essentials::repository::Repository;
use std::sync::Arc;

lazy_static! {
    pub static ref CANISTER_SERVICE: Arc<CanisterService> = Arc::new(CanisterService::new());
}

#[derive(Default, Debug)]
pub struct CanisterService {}

impl CanisterService {
    pub fn new() -> Self {
        Self {}
    }

    /// Checks if the caller is a controller.
    fn assert_controller(&self, ctx: &CallContext, method: String) -> ServiceResult<()> {
        if !ctx.is_controller() {
            Err(CanisterError::Forbidden { method })?
        }

        Ok(())
    }

    pub async fn upload_canister_modules(
        &self,
        input: UploadCanisterModulesInput,
    ) -> ServiceResult<()> {
        self.assert_controller(&CallContext::get(), "upload_canister_modules".to_string())?;

        let mut config = canister_config().unwrap_or_default();
        if let Some(upgrader_wasm_module) = input.upgrader_wasm_module {
            config.upgrader_wasm_module = upgrader_wasm_module;
        }
        if let Some(station_wasm_module) = input.station_wasm_module {
            config.station_wasm_module = station_wasm_module;
        }
        if let Some(station_wasm_module_extra_chunks) = input.station_wasm_module_extra_chunks {
            config.station_wasm_module_extra_chunks = station_wasm_module_extra_chunks;
        }
        write_canister_config(config);

        Ok(())
    }

    pub async fn init_canister(&self) -> ServiceResult<()> {
        if let Some(mut config) = canister_config() {
            config.last_upgrade_timestamp = time();
            self.handle_version_upgrades(config.version.as_deref());

            config.version = Some(SYSTEM_VERSION.to_string());
            write_canister_config(config);
        }

        Ok(())
    }

    pub fn handle_version_upgrades(&self, version: Option<&str>) {
        match version {
            // None is the initial version when the canister was not yet storing the version to stable memory.
            None => USER_REPOSITORY.list().iter_mut().for_each(|user| {
                user.stations.iter_mut().for_each(|station| {
                    station.labels = vec!["orbit-wallet".to_string()];
                });

                USER_REPOSITORY.insert(user.to_key(), user.clone());
            }),
            Some(version) => print(format!("No migration for version: {}", version)),
        };
    }
}
