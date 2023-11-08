use anyhow::{anyhow, Context};
use async_trait::async_trait;
use candid::Principal;
use ic_cdk::api::management_canister::main::{
    self as mgmt, CanisterInfoRequest, CanisterInstallMode, InstallCodeArgument,
};

use crate::{
    hash::Hash, interface::UpgradeParams, CheckController, LocalRef, StableValue, VerifyChecksum,
    WithLogs,
};

#[derive(Debug, thiserror::Error)]
pub enum UpgradeError {
    #[error("upgrade checksum mismatch")]
    ChecksumMismatch,
    #[error("canister is not a controller of target canister")]
    NotController,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[async_trait]
pub trait Upgrade: Sync + Send {
    async fn upgrade(&self, ps: UpgradeParams) -> Result<(), UpgradeError>;
}

#[derive(Clone)]
pub struct Upgrader {
    target: LocalRef<StableValue<String>>,
}

impl Upgrader {
    pub fn new(target: LocalRef<StableValue<String>>) -> Self {
        Self { target }
    }
}

#[async_trait]
impl Upgrade for Upgrader {
    async fn upgrade(&self, ps: UpgradeParams) -> Result<(), UpgradeError> {
        let id = self.target.with(|id| {
            id.borrow()
                .get(&())
                .map(Principal::from_text)
                .context("canister id not set")?
                .context("failed to parse principal")
        })?;

        mgmt::install_code(InstallCodeArgument {
            mode: CanisterInstallMode::Upgrade,
            canister_id: id,
            wasm_module: ps.module,
            arg: vec![],
        })
        .await
        .map_err(|(_, err)| anyhow!("failed to install code: {err}"))?;

        Ok(())
    }
}

pub struct WithCleanup<T>(pub T, pub LocalRef<StableValue<UpgradeParams>>);

#[async_trait]
impl<T: Upgrade> Upgrade for WithCleanup<T> {
    async fn upgrade(&self, ps: UpgradeParams) -> Result<(), UpgradeError> {
        let out = self.0.upgrade(ps).await;

        // Clear queue
        self.1.with(|q| {
            let mut q = q.borrow_mut();
            q.remove(&());
        });

        out
    }
}

#[async_trait]
impl<T: Upgrade> Upgrade for CheckController<T> {
    async fn upgrade(&self, ps: UpgradeParams) -> Result<(), UpgradeError> {
        let id = self.1.with(|id| {
            id.borrow()
                .get(&())
                .map(Principal::from_text)
                .context("canister id not set")?
                .context("failed to parse principal")
        })?;

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

#[async_trait]
impl<T: Upgrade, H: Hash> Upgrade for VerifyChecksum<T, H> {
    async fn upgrade(&self, ps: UpgradeParams) -> Result<(), UpgradeError> {
        if !self.1.hash(&ps.module).eq(&ps.checksum) {
            return Err(UpgradeError::ChecksumMismatch);
        }

        self.0.upgrade(ps).await
    }
}

#[async_trait]
impl<T: Upgrade> Upgrade for WithLogs<T> {
    async fn upgrade(&self, ps: UpgradeParams) -> Result<(), UpgradeError> {
        let out = self.0.upgrade(ps).await;

        let status = match &out {
            Ok(_) => "ok",
            Err(err) => match err {
                UpgradeError::ChecksumMismatch => "checksum-mismatch",
                UpgradeError::NotController => "not-controller",
                UpgradeError::UnexpectedError(_) => "fail",
            },
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
