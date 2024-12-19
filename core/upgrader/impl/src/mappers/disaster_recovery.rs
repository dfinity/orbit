use crate::model::{
    RequestDisasterRecoveryInstallCodeLog, RequestDisasterRecoveryOperationLog,
    StationRecoveryRequestInstallCodeOperation, StationRecoveryRequestOperation,
};
use orbit_essentials::utils::sha256_hash;

impl From<&upgrader_api::RequestDisasterRecoveryInput> for StationRecoveryRequestOperation {
    fn from(request: &upgrader_api::RequestDisasterRecoveryInput) -> Self {
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
                        install_mode: install_code.install_mode.clone().into(),
                        wasm_module: install_code.module.clone(),
                        wasm_module_extra_chunks: install_code.module_extra_chunks.clone(),
                        wasm_sha256,
                        arg: install_code.arg.clone(),
                        arg_sha256: sha256_hash(&install_code.arg),
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
