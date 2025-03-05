use crate::model::{
    DisasterRecovery, DisasterRecoveryV0, RequestDisasterRecoveryInstallCodeLog,
    RequestDisasterRecoveryOperationLog, RequestDisasterRecoveryPruneLog,
    RequestDisasterRecoveryRestoreLog, RequestDisasterRecoverySnapshotLog, StationRecoveryRequest,
    StationRecoveryRequestInstallCodeOperation,
    StationRecoveryRequestInstallCodeOperationFootprint, StationRecoveryRequestOperation,
    StationRecoveryRequestOperationFootprint, StationRecoveryRequestPruneOperation,
    StationRecoveryRequestPruneOperationFootprint, StationRecoveryRequestRestoreOperation,
    StationRecoveryRequestRestoreOperationFootprint, StationRecoveryRequestSnapshotOperation,
    StationRecoveryRequestSnapshotOperationFootprint, StationRecoveryRequestV0,
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
            upgrader_api::RequestDisasterRecoveryInput::Snapshot(snapshot) => {
                StationRecoveryRequestOperation::Snapshot(StationRecoveryRequestSnapshotOperation {
                    replace_snapshot: snapshot.replace_snapshot.map(|replace_snapshot| {
                        hex::decode(replace_snapshot).expect("Failed to parse `replace_snapshot`")
                    }),
                    force: snapshot.force,
                })
            }
            upgrader_api::RequestDisasterRecoveryInput::Restore(snapshot) => {
                StationRecoveryRequestOperation::Restore(StationRecoveryRequestRestoreOperation {
                    snapshot_id: hex::decode(snapshot.snapshot_id)
                        .expect("Failed to parse `snapshot_id`"),
                })
            }
            upgrader_api::RequestDisasterRecoveryInput::Prune(prune) => {
                let prune_op = match prune {
                    upgrader_api::RequestDisasterRecoveryPruneInput::Snapshot(snapshot_id) => {
                        StationRecoveryRequestPruneOperation::Snapshot(
                            hex::decode(snapshot_id).expect("Failed to parse `snapshot_id`"),
                        )
                    }
                    upgrader_api::RequestDisasterRecoveryPruneInput::ChunkStore => {
                        StationRecoveryRequestPruneOperation::ChunkStore
                    }
                    upgrader_api::RequestDisasterRecoveryPruneInput::State => {
                        StationRecoveryRequestPruneOperation::State
                    }
                };
                StationRecoveryRequestOperation::Prune(prune_op)
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
            StationRecoveryRequestOperation::Snapshot(ref snapshot) => {
                StationRecoveryRequestOperationFootprint::Snapshot(
                    StationRecoveryRequestSnapshotOperationFootprint {
                        replace_snapshot: snapshot.replace_snapshot.clone(),
                        force: snapshot.force,
                    },
                )
            }
            StationRecoveryRequestOperation::Restore(ref snapshot) => {
                StationRecoveryRequestOperationFootprint::Restore(
                    StationRecoveryRequestRestoreOperationFootprint {
                        snapshot_id: snapshot.snapshot_id.clone(),
                    },
                )
            }
            StationRecoveryRequestOperation::Prune(ref prune) => {
                let prune_op = match prune {
                    StationRecoveryRequestPruneOperation::Snapshot(snapshot_id) => {
                        StationRecoveryRequestPruneOperationFootprint::Snapshot(hex::encode(
                            snapshot_id,
                        ))
                    }
                    StationRecoveryRequestPruneOperation::ChunkStore => {
                        StationRecoveryRequestPruneOperationFootprint::ChunkStore
                    }
                    StationRecoveryRequestPruneOperation::State => {
                        StationRecoveryRequestPruneOperationFootprint::State
                    }
                };
                StationRecoveryRequestOperationFootprint::Prune(prune_op)
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
            StationRecoveryRequestOperation::Snapshot(ref snapshot) => {
                RequestDisasterRecoveryOperationLog::Snapshot(RequestDisasterRecoverySnapshotLog {
                    replace_snapshot: snapshot.replace_snapshot.as_ref().map(hex::encode),
                    force: snapshot.force,
                })
            }
            StationRecoveryRequestOperation::Restore(ref snapshot) => {
                RequestDisasterRecoveryOperationLog::Restore(RequestDisasterRecoveryRestoreLog {
                    snapshot_id: hex::encode(&snapshot.snapshot_id),
                })
            }
            StationRecoveryRequestOperation::Prune(ref prune) => {
                let prune_op = match prune {
                    StationRecoveryRequestPruneOperation::Snapshot(snapshot_id) => {
                        RequestDisasterRecoveryPruneLog::Snapshot(hex::encode(snapshot_id))
                    }
                    StationRecoveryRequestPruneOperation::ChunkStore => {
                        RequestDisasterRecoveryPruneLog::ChunkStore
                    }
                    StationRecoveryRequestPruneOperation::State => {
                        RequestDisasterRecoveryPruneLog::State
                    }
                };
                RequestDisasterRecoveryOperationLog::Prune(prune_op)
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
            StationRecoveryRequestOperation::Snapshot(ref snapshot) => {
                upgrader_api::StationRecoveryRequestOperation::Snapshot(
                    upgrader_api::StationRecoveryRequestSnapshotOperation {
                        replace_snapshot: snapshot.replace_snapshot.as_ref().map(hex::encode),
                        force: snapshot.force,
                    },
                )
            }
            StationRecoveryRequestOperation::Restore(ref snapshot) => {
                upgrader_api::StationRecoveryRequestOperation::Restore(
                    upgrader_api::StationRecoveryRequestRestoreOperation {
                        snapshot_id: hex::encode(&snapshot.snapshot_id),
                    },
                )
            }
            StationRecoveryRequestOperation::Prune(ref prune) => {
                let prune_op = match prune {
                    StationRecoveryRequestPruneOperation::Snapshot(snapshot_id) => {
                        upgrader_api::StationRecoveryRequestPruneOperation::Snapshot(hex::encode(
                            snapshot_id,
                        ))
                    }
                    StationRecoveryRequestPruneOperation::ChunkStore => {
                        upgrader_api::StationRecoveryRequestPruneOperation::ChunkStore
                    }
                    StationRecoveryRequestPruneOperation::State => {
                        upgrader_api::StationRecoveryRequestPruneOperation::State
                    }
                };
                upgrader_api::StationRecoveryRequestOperation::Prune(prune_op)
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
