use anyhow::{anyhow, Context};
use async_trait::async_trait;
use ic_cdk::api::management_canister::main::{self as mgmt, CanisterInfoRequest};
use mockall::automock;

use crate::{
    hash::Hash, interface::UpgradeParams, CheckController, LocalRef, StableValue,
    StorablePrincipal, VerifyChecksum, WithLogs,
};

#[derive(Debug, thiserror::Error)]
pub enum QueueError {
    #[error("upgrade checksum mismatch")]
    ChecksumMismatch,
    #[error("canister is not a controller of target canister")]
    NotController,
    #[error("unauthorized")]
    Unauthorized,
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

#[automock]
#[async_trait]
pub trait Queue: Sync + Send {
    async fn queue(&self, ps: UpgradeParams) -> Result<(), QueueError>;
}

pub struct Queuer {
    queue: LocalRef<StableValue<UpgradeParams>>,
}

impl Queuer {
    pub fn new(queue: LocalRef<StableValue<UpgradeParams>>) -> Self {
        Self { queue }
    }
}

#[async_trait]
impl Queue for Queuer {
    async fn queue(&self, ps: UpgradeParams) -> Result<(), QueueError> {
        self.queue.with(|q| {
            let mut q = q.borrow_mut();
            q.insert((), ps);
        });

        Ok(())
    }
}

#[async_trait]
impl<T: Queue> Queue for CheckController<T> {
    async fn queue(&self, ps: UpgradeParams) -> Result<(), QueueError> {
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
            return Err(QueueError::NotController);
        }

        self.0.queue(ps).await
    }
}

#[async_trait]
impl<T: Queue, H: Hash> Queue for VerifyChecksum<T, H> {
    async fn queue(&self, ps: UpgradeParams) -> Result<(), QueueError> {
        if !self.1.hash(&ps.module).eq(&ps.checksum) {
            return Err(QueueError::ChecksumMismatch);
        }

        self.0.queue(ps).await
    }
}

pub struct WithAuthorization<T>(pub T, pub LocalRef<StableValue<StorablePrincipal>>);

#[async_trait]
impl<T: Queue> Queue for WithAuthorization<T> {
    async fn queue(&self, ps: UpgradeParams) -> Result<(), QueueError> {
        let id = self
            .1
            .with(|id| id.borrow().get(&()).context("canister id not set"))?;

        if !ic_cdk::caller().eq(&id.0) {
            return Err(QueueError::Unauthorized);
        }

        self.0.queue(ps).await
    }
}

pub struct WithHexDecode<T>(pub T);

#[async_trait]
impl<T: Queue> Queue for WithHexDecode<T> {
    async fn queue(&self, ps: UpgradeParams) -> Result<(), QueueError> {
        let ps = UpgradeParams {
            module: hex::decode(ps.module).context("failed to decode module")?,
            checksum: hex::decode(ps.checksum).context("failed to decode checksum")?,
        };

        self.0.queue(ps).await
    }
}

#[async_trait]
impl<T: Queue> Queue for WithLogs<T> {
    async fn queue(&self, ps: UpgradeParams) -> Result<(), QueueError> {
        let out = self.0.queue(ps).await;

        let status = match &out {
            Ok(_) => "ok",
            Err(err) => match err {
                QueueError::ChecksumMismatch => "checksum-mismatch",
                QueueError::NotController => "not-controller",
                QueueError::Unauthorized => "unauthorized",
                QueueError::UnexpectedError(_) => "fail",
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
    use crate::hash::MockHash;

    #[tokio::test]
    async fn verify_checksum_invalid() -> Result<(), Error> {
        // Hash
        let mut h = MockHash::new();
        h.expect_hash()
            .times(1)
            .with(predicate::eq("module".as_bytes().to_vec()))
            .return_const("other".as_bytes().to_vec());

        // Queue
        let mut q = MockQueue::new();
        q.expect_queue().times(0);

        let out = VerifyChecksum(q, h)
            .queue(UpgradeParams {
                module: "module".as_bytes().to_vec(),
                checksum: "hash".as_bytes().to_vec(),
            })
            .await;

        match out {
            Err(QueueError::ChecksumMismatch) => {}
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

        // Queue
        let mut q = MockQueue::new();
        q.expect_queue()
            .times(1)
            .with(predicate::eq(UpgradeParams {
                module: "module".as_bytes().to_vec(),
                checksum: "hash".as_bytes().to_vec(),
            }))
            .returning(|_| Ok(()));

        let out = VerifyChecksum(q, h)
            .queue(UpgradeParams {
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
}
