use crate::{LocalRef, StableValue, StorablePrincipal};
use anyhow::{anyhow, Context};
use async_trait::async_trait;
use ic_cdk::api::management_canister::main::{
    self as mgmt, CanisterIdRecord, CanisterInfoRequest, CanisterInstallMode, InstallCodeArgument,
};
use mockall::automock;
use std::sync::Arc;
use upgrader_api::UpgradeParams;

#[derive(Debug, thiserror::Error)]
pub enum UpgradeError {
    #[error("canister is not a controller of target canister")]
    NotController,
    #[error("unauthorized")]
    Unauthorized,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
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
        let id = self
            .target
            .with(|id| id.borrow().get(&()).context("canister id not set"))?;

        mgmt::install_code(InstallCodeArgument {
            mode: CanisterInstallMode::Upgrade,
            canister_id: id.0,
            wasm_module: ps.module,
            arg: ps.arg,
        })
        .await
        .map_err(|(_, err)| anyhow!("failed to install code: {err}"))?;

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

pub struct WithBackground<T>(pub Arc<T>);

#[async_trait]
impl<T: Upgrade> Upgrade for WithBackground<T> {
    /// Spawn a background task performing the upgrade
    /// so that it is performed in a non-blocking manner
    async fn upgrade(&self, ps: UpgradeParams) -> Result<(), UpgradeError> {
        let u = self.0.clone();

        ic_cdk::spawn(async move {
            let _ = u.upgrade(ps).await;
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

        let status = match &out {
            Ok(_) => "ok".to_string(),
            Err(err) => err.to_string(),
        };

        ic_cdk::println!(
            "action = {}, status = {}, error = {:?}",
            self.1,
            status,
            out.as_ref().err()
        );

        out
    }
}
