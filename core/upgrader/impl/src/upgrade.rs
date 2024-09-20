use crate::{
    model::{LogEntryType, UpgradeResultLog},
    services::LOGGER_SERVICE,
    LocalRef, StableValue, StorablePrincipal,
};
use anyhow::{anyhow, Context};
use async_trait::async_trait;
use candid::{CandidType, Deserialize, Principal};
use ic_cdk::api::management_canister::main::{
    self as mgmt, CanisterIdRecord, CanisterInfoRequest, CanisterInstallMode, ChunkHash,
    ClearChunkStoreArgument, InstallChunkedCodeArgument, InstallCodeArgument, UploadChunkArgument,
};
use mockall::automock;
use orbit_essentials::api::ApiResult;
use orbit_essentials::cdk::{call, print};
use sha2::{Digest, Sha256};
use station_api::NotifyFailedStationUpgradeInput;
use std::sync::Arc;
use upgrader_api::WasmModuleExtraChunks;

#[derive(Debug, thiserror::Error)]
pub enum UpgradeError {
    #[error("canister is not a controller of target canister")]
    NotController,
    #[error("unauthorized")]
    Unauthorized,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

pub struct UpgradeParams {
    pub module: Vec<u8>,
    pub module_extra_chunks: Option<WasmModuleExtraChunks>,
    pub arg: Vec<u8>,
    pub install_mode: CanisterInstallMode,
}

#[automock]
#[async_trait]
pub trait Upgrade: 'static + Sync + Send {
    async fn upgrade(&self, ps: UpgradeParams) -> Result<(), UpgradeError>;
}

#[derive(Clone)]
pub struct Upgrader {
    target: LocalRef<StableValue<StorablePrincipal>>,
}

impl Upgrader {
    pub fn new(target: LocalRef<StableValue<StorablePrincipal>>) -> Self {
        Self { target }
    }
}

// asset canister argument types

#[derive(CandidType)]
struct GetArg {
    pub key: String,
    pub accept_encodings: Vec<String>,
}

#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct EncodedAsset {
    #[serde(with = "serde_bytes")]
    pub content: Vec<u8>,
    pub content_type: String,
    pub content_encoding: String,
    pub total_length: candid::Nat,
    #[serde(deserialize_with = "orbit_essentials::deserialize::deserialize_option_blob")]
    pub sha256: Option<Vec<u8>>,
}

// uploads a wasm chunk to the ICP chunk store
async fn upload_chunk(
    target_canister: Principal,
    chunk: Vec<u8>,
    expected_hash: &[u8],
) -> Result<(), UpgradeError> {
    let actual_hash = mgmt::upload_chunk(UploadChunkArgument {
        canister_id: target_canister,
        chunk,
    })
    .await
    .map_err(|(_, err)| anyhow!("failed to upload chunk: {err}"))?
    .0;
    if actual_hash.hash != *expected_hash {
        return Err(UpgradeError::UnexpectedError(anyhow!(
            "chunk hash mismatch (expected hash: {}, actual hash: {})",
            hex::encode(expected_hash),
            hex::encode(&actual_hash.hash)
        )));
    }
    Ok(())
}

#[async_trait]
impl Upgrade for Upgrader {
    async fn upgrade(&self, ps: UpgradeParams) -> Result<(), UpgradeError> {
        let target_canister = self
            .target
            .with(|id| id.borrow().get(&()).context("canister id not set"))?
            .0;

        if let Some(mut module_extra_chunks) = ps.module_extra_chunks {
            // clear the ICP chunk store of the target canister
            mgmt::clear_chunk_store(ClearChunkStoreArgument {
                canister_id: target_canister,
            })
            .await
            .map_err(|(_, err)| anyhow!("failed to clear chunk store: {err}"))?;
            // upload the provided module as the first chunk
            // to the ICP chunk store of the target canister
            let mut hasher = Sha256::new();
            hasher.update(ps.module.clone());
            let module_hash = hasher.finalize().to_vec();
            upload_chunk(target_canister, ps.module, &module_hash).await?;
            // fetch all extra chunks from the asset store canister
            // and upload them to the ICP chunk store of the target canister
            for hash in &module_extra_chunks.chunk_hashes_list {
                let asset = call::<_, (EncodedAsset,)>(
                    module_extra_chunks.store_canister,
                    "get",
                    (GetArg {
                        key: hex::encode(hash),
                        accept_encodings: vec!["identity".to_string()],
                    },),
                )
                .await
                .map_err(|(_, err)| anyhow!("failed to fetch chunk: {err}"))?
                .0;
                if asset.content.len() != asset.total_length {
                    return Err(UpgradeError::UnexpectedError(anyhow!(
                        "failed to fetch chunk (expected length: {}, actual length: {})",
                        asset.total_length,
                        asset.content.len()
                    )));
                }
                upload_chunk(target_canister, asset.content, hash).await?;
            }
            // install target canister from chunks stored in the ICP chunk store of the target canister
            let mut chunk_hashes_list = vec![module_hash];
            chunk_hashes_list.append(&mut module_extra_chunks.chunk_hashes_list);
            mgmt::install_chunked_code(InstallChunkedCodeArgument {
                mode: ps.install_mode,
                target_canister,
                store_canister: Some(target_canister),
                chunk_hashes_list: chunk_hashes_list
                    .into_iter()
                    .map(|hash| ChunkHash { hash })
                    .collect(),
                wasm_module_hash: module_extra_chunks.wasm_module_hash,
                arg: ps.arg,
            })
            .await
            .map_err(|(_, err)| anyhow!("failed to install code from chunks: {err}"))?;
        } else {
            mgmt::install_code(InstallCodeArgument {
                mode: ps.install_mode,
                canister_id: target_canister,
                wasm_module: ps.module,
                arg: ps.arg,
            })
            .await
            .map_err(|(_, err)| anyhow!("failed to install code: {err}"))?;
        }

        Ok(())
    }
}

pub struct WithStop<T>(pub T, pub LocalRef<StableValue<StorablePrincipal>>);

#[async_trait]
impl<T: Upgrade> Upgrade for WithStop<T> {
    /// Perform an upgrade but ensure that the target canister is stopped first
    async fn upgrade(&self, ps: UpgradeParams) -> Result<(), UpgradeError> {
        let id = self
            .1
            .with(|id| id.borrow().get(&()).context("canister id not set"))?;

        mgmt::stop_canister(CanisterIdRecord { canister_id: id.0 })
            .await
            .map_err(|(_, err)| anyhow!("failed to stop canister: {err}"))?;

        self.0.upgrade(ps).await
    }
}

pub struct WithStart<T>(pub T, pub LocalRef<StableValue<StorablePrincipal>>);

#[async_trait]
impl<T: Upgrade> Upgrade for WithStart<T> {
    /// Perform an upgrade but ensure that the target canister is restarted
    /// regardless of the upgrade succeeding or not
    async fn upgrade(&self, ps: UpgradeParams) -> Result<(), UpgradeError> {
        let out = self.0.upgrade(ps).await;

        let id = self
            .1
            .with(|id| id.borrow().get(&()).context("canister id not set"))?;

        mgmt::start_canister(CanisterIdRecord { canister_id: id.0 })
            .await
            .map_err(|(_, err)| anyhow!("failed to start canister: {err}"))?;

        out
    }
}

pub struct WithBackground<T>(pub Arc<T>, pub LocalRef<StableValue<StorablePrincipal>>);

#[async_trait]
impl<T: Upgrade> Upgrade for WithBackground<T> {
    /// Spawn a background task performing the upgrade
    /// so that it is performed in a non-blocking manner
    async fn upgrade(&self, ps: UpgradeParams) -> Result<(), UpgradeError> {
        let u = self.0.clone();
        let target_canister_id: Option<Principal> =
            self.1.with(|p| p.borrow().get(&()).map(|sp| sp.0));

        ic_cdk::spawn(async move {
            let res = u.upgrade(ps).await;
            // Notify the target canister about a failed upgrade unless the call is unauthorized
            // (we don't want to spam the target canister with such errors).
            if let Some(target_canister_id) = target_canister_id {
                if let Err(ref err) = res {
                    let err = match err {
                        UpgradeError::UnexpectedError(err) => Some(err.to_string()),
                        UpgradeError::NotController => Some(
                            "The upgrader canister is not a controller of the target canister"
                                .to_string(),
                        ),
                        UpgradeError::Unauthorized => None,
                    };
                    if let Some(err) = err {
                        let notify_failed_station_upgrade_input =
                            NotifyFailedStationUpgradeInput { reason: err };
                        let notify_res = call::<_, (ApiResult<()>,)>(
                            target_canister_id,
                            "notify_failed_station_upgrade",
                            (notify_failed_station_upgrade_input,),
                        )
                        .await
                        .map(|r| r.0);
                        // Log an error if the notification can't be made.
                        if let Err(e) = notify_res {
                            print(format!("notify_failed_station_upgrade failed: {:?}", e));
                        }
                    }
                }
            }
        });

        Ok(())
    }
}

pub struct WithAuthorization<T>(pub T, pub LocalRef<StableValue<StorablePrincipal>>);

#[async_trait]
impl<T: Upgrade> Upgrade for WithAuthorization<T> {
    async fn upgrade(&self, ps: UpgradeParams) -> Result<(), UpgradeError> {
        let id = self
            .1
            .with(|id| id.borrow().get(&()).context("canister id not set"))?;

        if !ic_cdk::caller().eq(&id.0) {
            return Err(UpgradeError::Unauthorized);
        }

        self.0.upgrade(ps).await
    }
}

pub struct CheckController<T>(pub T, pub LocalRef<StableValue<StorablePrincipal>>);

#[async_trait]
impl<T: Upgrade> Upgrade for CheckController<T> {
    async fn upgrade(&self, ps: UpgradeParams) -> Result<(), UpgradeError> {
        let id = self
            .1
            .with(|id| id.borrow().get(&()).context("canister id not set"))?;

        let (resp,) = mgmt::canister_info(CanisterInfoRequest {
            canister_id: id.0,
            num_requested_changes: None,
        })
        .await
        .map_err(|(code, err)| anyhow!("failed to get canister info: {code:?} {err}"))?;

        if !resp.controllers.contains(&ic_cdk::id()) {
            return Err(UpgradeError::NotController);
        }

        self.0.upgrade(ps).await
    }
}

pub struct WithLogs<T>(pub T, pub String);

#[async_trait]
impl<T: Upgrade> Upgrade for WithLogs<T> {
    async fn upgrade(&self, ps: UpgradeParams) -> Result<(), UpgradeError> {
        let out = self.0.upgrade(ps).await;

        LOGGER_SERVICE.log(LogEntryType::UpgradeResult(match &out {
            Ok(_) => UpgradeResultLog::Success,
            Err(err) => UpgradeResultLog::Failure(err.to_string()),
        }));

        out
    }
}
