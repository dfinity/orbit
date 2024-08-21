use crate::{errors::ChangeCanisterError, models::CanisterInstallMode};
use candid::Principal;
use ic_cdk::api::management_canister::{
    main::{self as mgmt, InstallCodeArgument},
    provisional::CanisterIdRecord,
};
use lazy_static::lazy_static;
use orbit_essentials::api::ServiceResult;
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
        let install_code_result = mgmt::install_code(InstallCodeArgument {
            mode: mode.into(),
            canister_id: canister_id.to_owned(),
            wasm_module: module.to_owned(),
            arg: arg.unwrap_or(default_bytes),
        })
        .await
        .map_err(|(_, err)| ChangeCanisterError::Failed {
            reason: err.to_string(),
        });

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
