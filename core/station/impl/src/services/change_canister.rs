use crate::{
    errors::ChangeCanisterError,
    models::{CanisterInstallMode, WasmModuleExtraChunks},
};
use candid::Principal;
use ic_cdk::api::management_canister::{main as mgmt, provisional::CanisterIdRecord};
use lazy_static::lazy_static;
use orbit_essentials::api::ServiceResult;
use orbit_essentials::install_chunked_code::install_chunked_code;
use std::sync::Arc;

lazy_static! {
    pub static ref CHANGE_CANISTER_SERVICE: Arc<ChangeCanisterService> =
        Arc::new(ChangeCanisterService::new());
}

#[derive(Default, Debug)]
pub struct ChangeCanisterService {}

impl ChangeCanisterService {
    pub fn new() -> Self {
        Self {}
    }

    /// Execute an install or upgrade of a canister.
    pub async fn install_canister(
        &self,
        canister_id: Principal,
        mode: CanisterInstallMode,
        module: &[u8],
        module_extra_chunks: Option<WasmModuleExtraChunks>,
        arg: Option<Vec<u8>>,
    ) -> ServiceResult<(), ChangeCanisterError> {
        use candid::Encode;

        // Stop canister
        let stop_result = mgmt::stop_canister(CanisterIdRecord {
            canister_id: canister_id.to_owned(),
        })
        .await
        .map_err(|(_, err)| ChangeCanisterError::Failed {
            reason: err.to_string(),
        });

        if stop_result.is_err() {
            // Restart canister if the stop did not succeed (its possible the canister did stop running)
            mgmt::start_canister(CanisterIdRecord {
                canister_id: canister_id.to_owned(),
            })
            .await
            .map_err(|(_, err)| ChangeCanisterError::Failed {
                reason: err.to_string(),
            })?;

            return stop_result;
        }

        // Install or upgrade canister
        let default_bytes = Encode!(&()).unwrap();
        let install_code_result = install_chunked_code(
            canister_id,
            mode.into(),
            module.to_owned(),
            module_extra_chunks.map(|c| c.into()),
            arg.unwrap_or(default_bytes),
        )
        .await
        .map_err(|err| ChangeCanisterError::Failed { reason: err });

        // Restart canister (regardless of whether the upgrade succeeded or not)
        mgmt::start_canister(CanisterIdRecord {
            canister_id: canister_id.to_owned(),
        })
        .await
        .map_err(|(_, err)| ChangeCanisterError::Failed {
            reason: err.to_string(),
        })?;

        install_code_result
    }
}
