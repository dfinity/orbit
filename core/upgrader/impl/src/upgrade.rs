use crate::{
    get_target_canister,
    model::{LogEntryType, UpgradeResultLog},
    services::LOGGER_SERVICE,
};
use anyhow::anyhow;
use async_trait::async_trait;
use ic_cdk::api::management_canister::main::{
    self as mgmt, CanisterIdRecord, CanisterInfoRequest, CanisterInstallMode,
};
use mockall::automock;
use orbit_essentials::api::ApiResult;
use orbit_essentials::cdk::api::canister_version;
use orbit_essentials::cdk::api::management_canister::main::{
    load_canister_snapshot, LoadCanisterSnapshotArgs,
};
use orbit_essentials::cdk::{call, print};
use orbit_essentials::install_chunked_code::install_chunked_code;
use orbit_essentials::types::WasmModuleExtraChunks;
use station_api::NotifyFailedStationUpgradeInput;
use std::sync::Arc;

#[derive(Debug, thiserror::Error)]
pub enum UpgradeError {
    #[error("canister is not a controller of target canister")]
    NotController,
    #[error("unauthorized")]
    Unauthorized,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

pub enum ChangeParams {
    Upgrade(UpgradeParams),
    Restore(RestoreParams),
}

pub struct UpgradeParams {
    pub module: Vec<u8>,
    pub module_extra_chunks: Option<WasmModuleExtraChunks>,
    pub arg: Vec<u8>,
    pub install_mode: CanisterInstallMode,
}

pub struct RestoreParams {
    pub snapshot_id: Vec<u8>,
}

#[automock]
#[async_trait]
pub trait Upgrade: 'static + Sync + Send {
    async fn upgrade(&self, ps: ChangeParams) -> Result<(), UpgradeError>;
}

#[derive(Clone)]
pub struct Upgrader {}

#[async_trait]
impl Upgrade for Upgrader {
    async fn upgrade(&self, ps: ChangeParams) -> Result<(), UpgradeError> {
        let target_canister = get_target_canister();

        match ps {
            ChangeParams::Upgrade(ps) => install_chunked_code(
                target_canister,
                ps.install_mode,
                ps.module,
                ps.module_extra_chunks,
                ps.arg,
            )
            .await
            .map_err(|e| anyhow!(e).into()),
            ChangeParams::Restore(ps) => load_canister_snapshot(LoadCanisterSnapshotArgs {
                canister_id: target_canister,
                snapshot_id: ps.snapshot_id,
                sender_canister_version: Some(canister_version()),
            })
            .await
            .map_err(|(_, e)| anyhow!(e).into()),
        }
    }
}

pub struct WithStop<T>(pub T);

#[async_trait]
impl<T: Upgrade> Upgrade for WithStop<T> {
    /// Perform an upgrade but ensure that the target canister is stopped first
    async fn upgrade(&self, ps: ChangeParams) -> Result<(), UpgradeError> {
        let id = get_target_canister();

        mgmt::stop_canister(CanisterIdRecord { canister_id: id })
            .await
            .map_err(|(_, err)| anyhow!("failed to stop canister: {err}"))?;

        self.0.upgrade(ps).await
    }
}

pub struct WithStart<T>(pub T);

#[async_trait]
impl<T: Upgrade> Upgrade for WithStart<T> {
    /// Perform an upgrade but ensure that the target canister is restarted
    /// regardless of the upgrade succeeding or not
    async fn upgrade(&self, ps: ChangeParams) -> Result<(), UpgradeError> {
        let out = self.0.upgrade(ps).await;

        let id = get_target_canister();

        mgmt::start_canister(CanisterIdRecord { canister_id: id })
            .await
            .map_err(|(_, err)| anyhow!("failed to start canister: {err}"))?;

        out
    }
}

pub struct WithBackground<T>(pub Arc<T>);

#[async_trait]
impl<T: Upgrade> Upgrade for WithBackground<T> {
    /// Spawn a background task performing the upgrade
    /// so that it is performed in a non-blocking manner
    async fn upgrade(&self, ps: ChangeParams) -> Result<(), UpgradeError> {
        let u = self.0.clone();
        let target_canister_id = get_target_canister();

        ic_cdk::spawn(async move {
            let res = u.upgrade(ps).await;
            // Notify the target canister about a failed upgrade or restore unless the call is unauthorized
            // (we don't want to spam the target canister with such errors).
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
        });

        Ok(())
    }
}

pub struct WithAuthorization<T>(pub T);

#[async_trait]
impl<T: Upgrade> Upgrade for WithAuthorization<T> {
    async fn upgrade(&self, ps: ChangeParams) -> Result<(), UpgradeError> {
        let id = get_target_canister();

        if !ic_cdk::caller().eq(&id) {
            return Err(UpgradeError::Unauthorized);
        }

        self.0.upgrade(ps).await
    }
}

pub struct CheckController<T>(pub T);

#[async_trait]
impl<T: Upgrade> Upgrade for CheckController<T> {
    async fn upgrade(&self, ps: ChangeParams) -> Result<(), UpgradeError> {
        let id = get_target_canister();

        let (resp,) = mgmt::canister_info(CanisterInfoRequest {
            canister_id: id,
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
    async fn upgrade(&self, ps: ChangeParams) -> Result<(), UpgradeError> {
        let out = self.0.upgrade(ps).await;

        LOGGER_SERVICE.log(LogEntryType::UpgradeResult(match &out {
            Ok(_) => UpgradeResultLog::Success,
            Err(err) => UpgradeResultLog::Failure(err.to_string()),
        }));

        out
    }
}
