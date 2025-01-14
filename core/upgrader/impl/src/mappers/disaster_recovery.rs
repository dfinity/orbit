use crate::model::{
    DisasterRecovery, DisasterRecoveryV0, RequestDisasterRecoveryInstallCodeLog,
    RequestDisasterRecoveryOperationLog, StationRecoveryRequest,
    StationRecoveryRequestInstallCodeOperation,
    StationRecoveryRequestInstallCodeOperationFootprint, StationRecoveryRequestOperation,
    StationRecoveryRequestOperationFootprint, StationRecoveryRequestV0,
};
use orbit_essentials::utils::sha256_hash;

impl From<upgrader_api::RequestDisasterRecoveryInput> for StationRecoveryRequestOperation {
    fn from(request: upgrader_api::RequestDisasterRecoveryInput) -> Self {
        match request {
            upgrader_api::RequestDisasterRecoveryInput::InstallCode(install_code) => {
                let wasm_sha256 =
                    if let Some(ref module_extra_chunks) = install_code.module_extra_chunks {
                        module_extra_chunks.wasm_module_hash.clone()
                    } else {
                        sha256_hash(&install_code.module)
                    };
                StationRecoveryRequestOperation::InstallCode(
                    StationRecoveryRequestInstallCodeOperation {
                        install_mode: install_code.install_mode.into(),
                        wasm_module: install_code.module,
                        wasm_module_extra_chunks: install_code.module_extra_chunks,
                        wasm_sha256,
                        arg_sha256: sha256_hash(&install_code.arg),
                        arg: install_code.arg,
                    },
                )
            }
        }
    }
}

impl From<&StationRecoveryRequestOperation> for StationRecoveryRequestOperationFootprint {
    fn from(operation: &StationRecoveryRequestOperation) -> Self {
        match operation {
            StationRecoveryRequestOperation::InstallCode(ref install_code) => {
                StationRecoveryRequestOperationFootprint::InstallCode(
                    StationRecoveryRequestInstallCodeOperationFootprint {
                        install_mode: install_code.install_mode,
                        wasm_sha256: install_code.wasm_sha256.clone(),
                        arg_sha256: install_code.arg_sha256.clone(),
                    },
                )
            }
        }
    }
}

impl From<&StationRecoveryRequestOperation> for RequestDisasterRecoveryOperationLog {
    fn from(operation: &StationRecoveryRequestOperation) -> Self {
        match operation {
            StationRecoveryRequestOperation::InstallCode(ref install_code) => {
                RequestDisasterRecoveryOperationLog::InstallCode(
                    RequestDisasterRecoveryInstallCodeLog {
                        install_mode: install_code.install_mode.to_string(),
                        wasm_sha256: hex::encode(&install_code.wasm_sha256),
                        arg_sha256: hex::encode(&install_code.arg_sha256),
                    },
                )
            }
        }
    }
}

impl From<&StationRecoveryRequestOperation> for upgrader_api::StationRecoveryRequestOperation {
    fn from(operation: &StationRecoveryRequestOperation) -> Self {
        match operation {
            StationRecoveryRequestOperation::InstallCode(ref install_code) => {
                upgrader_api::StationRecoveryRequestOperation::InstallCode(
                    upgrader_api::StationRecoveryRequestInstallCodeOperation {
                        install_mode: install_code.install_mode.into(),
                        wasm_sha256: install_code.wasm_sha256.clone(),
                        arg: install_code.arg.clone(),
                    },
                )
            }
        }
    }
}

// legacy types

impl From<StationRecoveryRequestV0> for StationRecoveryRequest {
    fn from(request: StationRecoveryRequestV0) -> Self {
        Self {
            user_id: request.user_id,
            operation: StationRecoveryRequestOperation::InstallCode(
                StationRecoveryRequestInstallCodeOperation {
                    install_mode: request.install_mode,
                    wasm_module: request.wasm_module,
                    wasm_module_extra_chunks: request.wasm_module_extra_chunks,
                    wasm_sha256: request.wasm_sha256,
                    arg: request.arg,
                    arg_sha256: request.arg_sha256,
                },
            ),
            submitted_at: request.submitted_at,
        }
    }
}

impl From<DisasterRecoveryV0> for DisasterRecovery {
    fn from(disaster_recovery: DisasterRecoveryV0) -> Self {
        Self {
            accounts: disaster_recovery.accounts,
            multi_asset_accounts: disaster_recovery.multi_asset_accounts,
            assets: disaster_recovery.assets,
            committee: disaster_recovery.committee,
            recovery_requests: disaster_recovery
                .recovery_requests
                .into_iter()
                .map(|request| request.into())
                .collect(),
            recovery_status: disaster_recovery.recovery_status,
            last_recovery_result: disaster_recovery.last_recovery_result,
        }
    }
}
