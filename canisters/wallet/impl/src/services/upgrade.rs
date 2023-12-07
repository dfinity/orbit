use anyhow::anyhow;
use candid::{CandidType, Principal};
use ic_canister_core::api::{ApiError, ServiceResult};
use ic_cdk::api::management_canister::main::{
    self as mgmt, CanisterInstallMode, InstallCodeArgument,
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

        mgmt::install_code(InstallCodeArgument {
            mode: CanisterInstallMode::Upgrade,
            canister_id: upgrader_canister_id,
            wasm_module: module.to_owned(),
            arg: vec![],
        })
        .await
        .map_err(|(_, err)| ApiError::new("UPGRADE_FAILED".to_string(), Some(err), None))?;

        Ok(())
    }
}
