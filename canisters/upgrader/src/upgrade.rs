use anyhow::{anyhow, Context};
use async_trait::async_trait;
use ic_cdk::api::management_canister::{
    main::{self as mgmt, CanisterInfoRequest, CanisterInstallMode, InstallCodeArgument},
    provisional::CanisterIdRecord,
};
use mockall::automock;

use crate::{
    hash::Hash, interface::UpgradeParams, CheckController, LocalRef, StableValue,
    StorablePrincipal, VerifyChecksum, WithLogs,
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

#[automock]
#[async_trait]
pub trait Upgrade: Sync + Send {
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

        // Stop
        mgmt::stop_canister(CanisterIdRecord { canister_id: id.0 })
            .await
            .map_err(|(_, err)| anyhow!("failed to stop canister: {err}"))?;

        // Upgrade
        mgmt::install_code(InstallCodeArgument {
            mode: CanisterInstallMode::Upgrade,
            canister_id: id.0,
            wasm_module: ps.module,
            arg: vec![],
        })
        .await
        .map_err(|(_, err)| anyhow!("failed to install code: {err}"))?;

        // Start
        mgmt::start_canister(CanisterIdRecord { canister_id: id.0 })
            .await
            .map_err(|(_, err)| anyhow!("failed to start canister: {err}"))?;

        Ok(())
    }
}

pub struct WithCleanup<T>(pub T, pub LocalRef<StableValue<UpgradeParams>>);

#[async_trait]
impl<T: Upgrade> Upgrade for WithCleanup<T> {
    async fn upgrade(&self, ps: UpgradeParams) -> Result<(), UpgradeError> {
        // Clear queue
        self.1.with(|q| {
            let mut q = q.borrow_mut();
            q.remove(&());
        });

        self.0.upgrade(ps).await
    }
}

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

#[cfg(test)]
mod tests {
    use anyhow::Error;
    use mockall::predicate;

    use super::*;
    use crate::{hash::MockHash, QUEUED_UPGRADE_PARAMS};

    #[tokio::test]
    async fn verify_checksum_invalid() -> Result<(), Error> {
        // Hash
        let mut h = MockHash::new();
        h.expect_hash()
            .times(1)
            .with(predicate::eq("module".as_bytes().to_vec()))
            .return_const("other".as_bytes().to_vec());

        // Upgrade
        let mut u = MockUpgrade::new();
        u.expect_upgrade().times(0);

        let out = VerifyChecksum(u, h)
            .upgrade(UpgradeParams {
                module: "module".as_bytes().to_vec(),
                checksum: "hash".as_bytes().to_vec(),
            })
            .await;

        match out {
            Err(UpgradeError::ChecksumMismatch) => {}
            _ => return Err(anyhow!("expected a checksum mismatch but none occurred")),
        }

        Ok(())
    }

    #[tokio::test]
    async fn verify_checksum_valid() -> Result<(), Error> {
        // Hash
        let mut h = MockHash::new();
        h.expect_hash()
            .times(1)
            .with(predicate::eq("module".as_bytes().to_vec()))
            .return_const("hash".as_bytes().to_vec());

        // Upgrade
        let mut u = MockUpgrade::new();
        u.expect_upgrade()
            .times(1)
            .with(predicate::eq(UpgradeParams {
                module: "module".as_bytes().to_vec(),
                checksum: "hash".as_bytes().to_vec(),
            }))
            .returning(|_| Ok(()));

        let out = VerifyChecksum(u, h)
            .upgrade(UpgradeParams {
                module: "module".as_bytes().to_vec(),
                checksum: "hash".as_bytes().to_vec(),
            })
            .await;

        match out {
            Ok(()) => {}
            _ => return Err(anyhow!("expected checksum verification to succeed")),
        }

        Ok(())
    }

    #[tokio::test]
    async fn with_cleanup() -> Result<(), Error> {
        // Upgrade
        let mut u = MockUpgrade::new();
        u.expect_upgrade().times(1).returning(|_| Ok(()));

        let ps = UpgradeParams {
            module: "module".as_bytes().to_vec(),
            checksum: "hash".as_bytes().to_vec(),
        };

        QUEUED_UPGRADE_PARAMS.with(|v| v.borrow_mut().insert((), ps.clone()));

        WithCleanup(u, &QUEUED_UPGRADE_PARAMS)
            .upgrade(ps)
            .await
            .context("failed to call upgrade")?;

        if QUEUED_UPGRADE_PARAMS.with(|v| v.borrow().get(&()).is_some()) {
            return Err(anyhow!("expected queued params to be cleaned up"));
        }

        Ok(())
    }
}
