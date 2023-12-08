use anyhow::anyhow;
use candid::{CandidType, Encode, Principal};
use ic_canister_core::api::{ApiError, ServiceResult};
use ic_cdk::api::management_canister::{
    main::{self as mgmt, CanisterInstallMode, InstallCodeArgument},
    provisional::CanisterIdRecord,
};
use lazy_static::lazy_static;

lazy_static! {
    pub static ref UPGRADE_SERVICE: UpgradeService = UpgradeService::default();
}

#[derive(Default, Debug)]
pub struct UpgradeService {
    pub upgrader_canister_id: Option<Principal>,
}

#[derive(Clone, CandidType)]
struct UpgradeParams {
    module: Vec<u8>,
    checksum: Vec<u8>,
}

impl UpgradeService {
    /// Execute an upgrade of the wallet by requesting the upgrader to perform it on our behalf.
    pub async fn upgrade_wallet(&self, module: &[u8], checksum: &[u8]) -> ServiceResult<()> {
        let upgrader_canister_id = match self.upgrader_canister_id {
            Some(id) => Ok(id.to_owned()),
            None => Err(ApiError::new(
                "UPGRADER_CANISTER_ID_NOT_SET".to_string(),
                None,
                None,
            )),
        }?;

        let ps = UpgradeParams {
            module: module.to_owned(),
            checksum: checksum.to_owned(),
        };

        ic_cdk::call(upgrader_canister_id, "trigger_upgrade", (ps,))
            .await
            .map_err(|(_, err)| ApiError::new("UPGRADE_FAILED".to_string(), Some(err), None))?;

        Ok(())
    }

    /// Execute an upgrade of the upgrader canister.
    pub async fn upgrade_upgrader(&self, module: &[u8]) -> ServiceResult<()> {
        let upgrader_canister_id = match self.upgrader_canister_id {
            Some(id) => Ok(id.to_owned()),
            None => Err(ApiError::new(
                "UPGRADER_CANISTER_ID_NOT_SET".to_string(),
                None,
                None,
            )),
        }?;

        // Stop canister
        let stop_result = mgmt::stop_canister(CanisterIdRecord {
            canister_id: upgrader_canister_id.to_owned(),
        })
        .await
        .map_err(|(_, err)| ApiError::new("UPGRADE_FAILED".to_string(), Some(err), None));

        if stop_result.is_err() {
            // Restart canister if the stop did not succeed (its possible the canister did stop running)
            mgmt::start_canister(CanisterIdRecord {
                canister_id: upgrader_canister_id.to_owned(),
            })
            .await
            .map_err(|(_, err)| ApiError::new("UPGRADE_FAILED".to_string(), Some(err), None))?;

            return stop_result;
        }

        // Upgrade canister
        let upgrade_result = mgmt::install_code(InstallCodeArgument {
            mode: CanisterInstallMode::Upgrade,
            canister_id: upgrader_canister_id.to_owned(),
            wasm_module: module.to_owned(),
            arg: vec![],
        })
        .await
        .map_err(|(_, err)| ApiError::new("UPGRADE_FAILED".to_string(), Some(err), None));

        // Restart canister (regardless of whether the upgrade succeeded or not)
        mgmt::start_canister(CanisterIdRecord {
            canister_id: upgrader_canister_id.to_owned(),
        })
        .await
        .map_err(|(_, err)| ApiError::new("UPGRADE_FAILED".to_string(), Some(err), None))?;

        upgrade_result
    }
}
