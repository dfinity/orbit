use crate::{
    model::{LogEntryType, UpgradeResultLog},
    services::LOGGER_SERVICE,
    LocalRef, StableValue, StorablePrincipal,
};
use anyhow::{anyhow, Context};
use async_trait::async_trait;
use candid::Principal;
use ic_cdk::api::management_canister::main::{
    self as mgmt, CanisterIdRecord, CanisterInfoRequest, CanisterInstallMode, CanisterSettings,
    UpdateSettingsArgument,
};
use mockall::automock;
use orbit_essentials::api::ApiResult;
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

#[async_trait]
impl Upgrade for Upgrader {
    async fn upgrade(&self, ps: UpgradeParams) -> Result<(), UpgradeError> {
        let target_canister = self
            .target
            .with(|id| id.borrow().get(&()).context("canister id not set"))?
            .0;

        install_chunked_code(
            target_canister,
            ps.install_mode,
            ps.module,
            ps.module_extra_chunks,
            ps.arg,
        )
        .await
        .map_err(|e| anyhow!(e).into())
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

        if mgmt::stop_canister(CanisterIdRecord { canister_id: id.0 })
            .await
            .is_err()
        {
            // set the target canister's compute allocation to 1
            // so that it can more likely stop within timeout
            // we ignore errors here since we can't do much
            // if there's no compute allocation available on the subnet
            let _ = mgmt::update_settings(UpdateSettingsArgument {
                canister_id: id.0,
                settings: CanisterSettings {
                    compute_allocation: Some(1_u64.into()),
                    ..Default::default()
                },
            })
            .await;
            // we retry the stop canister call in any case
            mgmt::stop_canister(CanisterIdRecord { canister_id: id.0 })
                .await
                .map_err(|(_, err)| anyhow!("failed to stop canister: {err}"))?;
        }

        let res = self.0.upgrade(ps).await;

        // reset the target canister's compute allocation back to 0
        // so that we don't end up paying too much for the compute allocation
        // this is unlikely to fail, but even if it did,
        // the target canister can be upgraded again and then this call will be retried
        let _ = mgmt::update_settings(UpdateSettingsArgument {
            canister_id: id.0,
            settings: CanisterSettings {
                compute_allocation: Some(0_u64.into()),
                ..Default::default()
            },
        })
        .await;

        res
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
