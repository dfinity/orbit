use crate::{
    errors::ChangeCanisterError,
    models::{CanisterInstallMode, PruneExternalCanisterResource, WasmModuleExtraChunks},
};
use candid::Principal;
use ic_cdk::api::management_canister::{
    main as mgmt,
    main::{
        CanisterIdRecord, ClearChunkStoreArgument, DeleteCanisterSnapshotArgs,
        LoadCanisterSnapshotArgs, TakeCanisterSnapshotArgs,
    },
};
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

    async fn start_canister(
        &self,
        canister_id: Principal,
    ) -> ServiceResult<(), ChangeCanisterError> {
        mgmt::start_canister(CanisterIdRecord {
            canister_id: canister_id.to_owned(),
        })
        .await
        .map_err(|(_, err)| ChangeCanisterError::Failed {
            reason: err.to_string(),
        })
    }

    async fn stop_canister(
        &self,
        canister_id: Principal,
    ) -> ServiceResult<(), ChangeCanisterError> {
        let stop_result = mgmt::stop_canister(CanisterIdRecord {
            canister_id: canister_id.to_owned(),
        })
        .await
        .map_err(|(_, err)| ChangeCanisterError::Failed {
            reason: err.to_string(),
        });

        if let Err(e) = stop_result {
            // Restart canister if the call to stop did not succeed (it is possible that the canister did stop)
            self.start_canister(canister_id).await?;

            return Err(e);
        }

        Ok(())
    }

    pub async fn prune_canister(
        &self,
        canister_id: Principal,
        prune: PruneExternalCanisterResource,
    ) -> ServiceResult<(), ChangeCanisterError> {
        match prune {
            PruneExternalCanisterResource::Snapshot(snapshot_id) => {
                mgmt::delete_canister_snapshot(DeleteCanisterSnapshotArgs {
                    canister_id: canister_id.to_owned(),
                    snapshot_id,
                })
                .await
                .map_err(|(_, err)| ChangeCanisterError::Failed {
                    reason: err.to_string(),
                })
            }
            PruneExternalCanisterResource::ChunkStore => {
                mgmt::clear_chunk_store(ClearChunkStoreArgument {
                    canister_id: canister_id.to_owned(),
                })
                .await
                .map_err(|(_, err)| ChangeCanisterError::Failed {
                    reason: err.to_string(),
                })
            }
            PruneExternalCanisterResource::State => mgmt::uninstall_code(CanisterIdRecord {
                canister_id: canister_id.to_owned(),
            })
            .await
            .map_err(|(_, err)| ChangeCanisterError::Failed {
                reason: err.to_string(),
            }),
        }
    }

    /// Take a snapshot of a canister.
    pub async fn snapshot_canister(
        &self,
        canister_id: Principal,
        replace_snapshot: Option<Vec<u8>>,
        force: bool,
    ) -> ServiceResult<Vec<u8>, ChangeCanisterError> {
        let stop_result = self.stop_canister(canister_id).await;

        if !force {
            stop_result?;
        }

        // Take snapshot
        let take_snapshot_result = mgmt::take_canister_snapshot(TakeCanisterSnapshotArgs {
            canister_id,
            replace_snapshot,
        })
        .await
        .map(|res| res.0.id)
        .map_err(|(_, err)| ChangeCanisterError::Failed {
            reason: err.to_string(),
        });

        // Restart canister (regardless of whether the upgrade succeeded or not)
        self.start_canister(canister_id).await?;

        take_snapshot_result
    }

    /// Restore a canister from a snapshot.
    pub async fn restore_canister(
        &self,
        canister_id: Principal,
        snapshot_id: Vec<u8>,
    ) -> ServiceResult<(), ChangeCanisterError> {
        self.stop_canister(canister_id).await?;

        // Take snapshot
        let load_snapshot_result = mgmt::load_canister_snapshot(LoadCanisterSnapshotArgs {
            canister_id,
            snapshot_id,
            sender_canister_version: Some(ic_cdk::api::canister_version()),
        })
        .await
        .map_err(|(_, err)| ChangeCanisterError::Failed {
            reason: err.to_string(),
        });

        // Restart canister (regardless of whether the upgrade succeeded or not)
        self.start_canister(canister_id).await?;

        load_snapshot_result
    }

    /// Execute an install or upgrade of a canister.
    pub async fn install_canister(
        &self,
        canister_id: Principal,
        mode: CanisterInstallMode,
        module: &[u8],
        module_extra_chunks: &Option<WasmModuleExtraChunks>,
        arg: Option<Vec<u8>>,
    ) -> ServiceResult<(), ChangeCanisterError> {
        use candid::Encode;

        self.stop_canister(canister_id).await?;

        // Install or upgrade canister
        let default_bytes = Encode!(&()).unwrap();
        let install_code_result = install_chunked_code(
            canister_id,
            mode.into(),
            module.to_owned(),
            module_extra_chunks.as_ref().map(|c| c.clone().into()),
            arg.unwrap_or(default_bytes),
        )
        .await
        .map_err(|err| ChangeCanisterError::Failed { reason: err });

        // Restart canister (regardless of whether the upgrade succeeded or not)
        self.start_canister(canister_id).await?;

        install_code_result
    }
}
