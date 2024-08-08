use std::sync::Arc;

use async_trait::async_trait;
use candid::Principal;
use ic_ledger_types::{Memo, Subaccount, Tokens, TransferArgs};

use crate::api::{
    cmc::{CyclesMintingCanister, NotifyError, NotifyTopUpResult},
    ledger::LedgerCanister,
};

#[derive(Debug)]
pub struct ObtainCycleError {
    /// Details of the error.
    pub details: String,
    /// Whether the operation can be retried.
    pub can_retry: bool,
}

/// Trait to top up the funding canister balance if it is too low.
/// Example sources are minting from or swapping for ICP.
#[async_trait]
pub trait ObtainCycles: Send + Sync {
    async fn obtain_cycles(
        &self,
        amount: u128,
        target_canister_id: Principal,
    ) -> Result<u128, ObtainCycleError>;
}

pub struct MintCycles {
    pub cmc: Arc<dyn CyclesMintingCanister>,
    pub ledger: Arc<dyn LedgerCanister>,
    pub from_subaccount: Subaccount,
}

#[async_trait]
impl ObtainCycles for MintCycles {
    async fn obtain_cycles(
        &self,
        amount: u128,
        target_canister_id: candid::Principal,
    ) -> Result<u128, ObtainCycleError> {
        // get ICP/XDR rate from CMC
        let price = self
            .cmc
            .get_icp_xdr()
            .await
            .map_err(|err| ObtainCycleError {
                details: format!(
                    "Error getting ICP/XDR price from CMC: code={:?}, message={}",
                    err.0, err.1
                ),

                // at this point the process can be safely retried
                can_retry: true,
            })?;

        // convert cycle amount to ICP amount
        let cycles_per_xdr: u128 = 1_000_000_000_000; // 1 trillion cycles per XDR
        let cycles_per_icp: u128 =
            price.data.xdr_permyriad_per_icp as u128 * cycles_per_xdr / 10_000u128;
        let icp_to_mint_cycles_from_e8s = amount * 100_000_000u128 / cycles_per_icp;

        // transfer ICP to ledger account of CMC
        let call_result = self
            .ledger
            .transfer(TransferArgs {
                memo: Memo(0x50555054),
                amount: Tokens::from_e8s(icp_to_mint_cycles_from_e8s as u64),
                fee: Tokens::from_e8s(10_000),
                from_subaccount: Some(self.from_subaccount),
                to: self.cmc.get_top_up_address(target_canister_id),
                created_at_time: None,
            })
            .await
            .map_err(|err| ObtainCycleError {
                details: format!(
                    "Error transferring ICP to CMC account: code={:?}, message={}",
                    err.0, err.1
                ),
                // failed transfers should be safe to retry
                can_retry: true,
            })?;

        let block_index = call_result.map_err(|err| ObtainCycleError {
            can_retry: matches!(&err, ic_ledger_types::TransferError::TxCreatedInFuture),
            details: format!("Error transferring ICP to CMC account: {}", err),
        })?;

        // notify the CMC canister about the transfer so it can mint cycles
        // retry if the transaction is still processing
        let mut retries_left = 10;

        loop {
            retries_left -= 1;

            match self
                .cmc
                .notify_top_up(block_index, target_canister_id)
                .await
            {
                Err(err) => {
                    if retries_left == 0 {
                        // retry the notify call
                        Err(ObtainCycleError {
                            details: format!(
                                "Error notifying CMC about top-up: code={:?}, message={}",
                                err.0, err.1
                            ),
                            can_retry: false,
                        })?;
                    } else {
                        continue;
                    }
                }
                Ok(NotifyTopUpResult::Ok(cycles)) => {
                    // exit the retry loop
                    return Ok(cycles);
                }
                Ok(NotifyTopUpResult::Err(err)) => match &err {
                    NotifyError::Refunded {
                        reason,
                        block_index,
                    } => Err(ObtainCycleError {
                        details: format!(
                            "Top-up transaction refunded: reason={}, block_index={:?}",
                            reason, block_index
                        ),
                        can_retry: true,
                    }),
                    NotifyError::Processing => {
                        if retries_left == 0 {
                            Err(ObtainCycleError {
                                details: "Top-up transaction still processing after retries."
                                    .to_owned(),
                                can_retry: false,
                            })?;
                        }
                        continue;
                    }
                    NotifyError::TransactionTooOld(_) => Err(ObtainCycleError {
                        details: "Top-up transaction too old.".to_owned(),
                        can_retry: false,
                    }),
                    NotifyError::InvalidTransaction(message) => Err(ObtainCycleError {
                        details: format!("Invalid top-up transaction: {}", message),
                        can_retry: false,
                    }),
                    NotifyError::Other {
                        error_code,
                        error_message,
                    } => {
                        if retries_left == 0 {
                            Err(ObtainCycleError {
                                details: format!(
                                    "Error notifying CMC about top-up: code={}, message={}",
                                    error_code, error_message
                                ),
                                can_retry: false,
                            })?;
                        }
                        continue;
                    }
                }?,
            };
        }
    }
}

#[cfg(test)]
mod test {

    use ic_cdk::api::call::RejectionCode;

    use super::*;
    use crate::api::{cmc::test::TestCmcCanister, ledger::test::TestLedgerCanister};

    #[tokio::test]
    async fn test_obtain_by_minting() {
        let cmc = Arc::new(TestCmcCanister::default());
        let ledger = Arc::new(TestLedgerCanister::default());

        let obtain = MintCycles {
            cmc: cmc.clone(),
            ledger: ledger.clone(),
            from_subaccount: Subaccount([0u8; 32]),
        };

        obtain
            .obtain_cycles(1_000_000_000_000, Principal::anonymous())
            .await
            .expect("obtain_cycles failed");

        // calls to get the ICP price
        assert!(*cmc.get_icp_xdr_called.read().await);

        // calls to transfer ICP to the CMC account
        assert!(matches!(
            ledger.transfer_called_with.read().await.first(),
            Some(TransferArgs { amount, .. }) if amount == &Tokens::from_e8s(100_000_000 / 5)
        ));
    }

    #[tokio::test]
    async fn test_cycle_minting_notify_retries() {
        let notify_return_values_retried = vec![
            (Err((RejectionCode::SysFatal, "error".to_string())), true),
            (Ok(NotifyTopUpResult::Err(NotifyError::Processing)), true),
            (
                Ok(NotifyTopUpResult::Err(NotifyError::Other {
                    error_code: 0,
                    error_message: String::new(),
                })),
                true,
            ),
            (
                Ok(NotifyTopUpResult::Err(NotifyError::Refunded {
                    block_index: Some(0),
                    reason: "reason".to_string(),
                })),
                false,
            ),
        ];

        for test in notify_return_values_retried {
            let cmc = Arc::new(TestCmcCanister {
                notify_top_up_returns_with: Some(test.0),
                ..Default::default()
            });

            let ledger = Arc::new(TestLedgerCanister::default());

            let obtain = MintCycles {
                cmc: cmc.clone(),
                ledger: ledger.clone(),
                from_subaccount: Subaccount([0u8; 32]),
            };

            obtain
                .obtain_cycles(1_000_000_000_000, Principal::anonymous())
                .await
                .expect_err("obtain_cycles should fail");

            // transfer was called only once
            assert!(ledger.transfer_called_with.read().await.len() == 1);

            if test.1 {
                // notify was retried
                assert!(cmc.notify_top_up_called_with.read().await.len() > 1);
            } else {
                assert_eq!(cmc.notify_top_up_called_with.read().await.len(), 1);
            }
        }
    }
}
