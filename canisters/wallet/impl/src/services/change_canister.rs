use super::ProposalEditInput;
use crate::{
    core::{upgrader_canister_id, CanisterConfig},
    errors::ChangeCanisterError,
    models::ProposalStatus,
    services::{ProposalService, PROPOSAL_SERVICE},
};
use candid::CandidType;
use candid::Principal;
use ic_canister_core::api::ServiceResult;
use ic_cdk::api::management_canister::{
    main::{self as mgmt, CanisterInstallMode, InstallCodeArgument},
    provisional::CanisterIdRecord,
};
use lazy_static::lazy_static;
use std::sync::Arc;

lazy_static! {
    pub static ref CHANGE_CANISTER_SERVICE: Arc<ChangeCanisterService> =
        Arc::new(ChangeCanisterService::new(Arc::clone(&PROPOSAL_SERVICE)));
}

#[derive(Debug)]
pub struct ChangeCanisterService {
    proposal_service: Arc<ProposalService>,
}

#[derive(Clone, CandidType)]
struct ChangeCanisterParams {
    module: Vec<u8>,
    arg: Vec<u8>,
    checksum: Vec<u8>,
}

impl ChangeCanisterService {
    pub fn new(proposal_service: Arc<ProposalService>) -> Self {
        Self { proposal_service }
    }

    /// Execute an upgrade of the wallet by requesting the upgrader to perform it on our behalf.
    pub async fn upgrade_wallet(
        &self,
        module: &[u8],
        arg: &[u8],
        checksum: &[u8],
    ) -> ServiceResult<()> {
        let upgrader_canister_id = upgrader_canister_id();

        ic_cdk::call(
            upgrader_canister_id,
            "trigger_upgrade",
            (ChangeCanisterParams {
                module: module.to_owned(),
                arg: arg.to_owned(),
                checksum: checksum.to_owned(),
            },),
        )
        .await
        .map_err(|(_, err)| ChangeCanisterError::Failed {
            reason: err.to_string(),
        })?;

        Ok(())
    }

    /// Execute an upgrade of the upgrader canister.
    pub async fn upgrade_upgrader(
        &self,
        module: &[u8],
        arg: Option<Vec<u8>>,
    ) -> ServiceResult<(), ChangeCanisterError> {
        let upgrader_canister_id = upgrader_canister_id();
        self.install_canister(
            upgrader_canister_id,
            CanisterInstallMode::Upgrade,
            module,
            arg,
        )
        .await
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
            mode,
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

    /// Verify and mark an upgrade as being performed successfully.
    pub async fn update_change_canister_proposal_status(
        &self,
        cfg: &CanisterConfig,
        status: ProposalStatus,
    ) -> ServiceResult<()> {
        let proposal_id = cfg
            .change_canister_proposal
            .ok_or(ChangeCanisterError::MissingChangeCanisterProposal)?;

        self.proposal_service
            .edit_proposal(ProposalEditInput {
                proposal_id,
                status: Some(status),
            })
            .await?;

        Ok(())
    }
}
