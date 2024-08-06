use async_trait::async_trait;
use candid::{CandidType, Principal};
use ic_cdk::api::call::CallResult;
use ic_ledger_types::{AccountIdentifier, Subaccount};
use serde::Deserialize;

#[derive(CandidType, Deserialize)]
pub struct GetIcpXdrResultData {
    pub xdr_permyriad_per_icp: u64,
    pub timestamp_seconds: u64,
}
#[derive(CandidType, Deserialize)]
pub struct GetIcpXdrResult {
    pub data: GetIcpXdrResultData,
    pub certificate: Vec<u8>,
    pub hash_tree: Vec<u8>,
}

#[derive(CandidType)]
pub struct NotifyTopUpArg {
    pub block_index: u64,
    pub canister_id: Principal,
}

#[derive(CandidType, Deserialize)]
pub enum NotifyTopUpResult {
    Ok(u128),
    Err(NotifyError),
}

#[derive(CandidType, Deserialize)]
pub enum NotifyError {
    Refunded {
        reason: String,
        block_index: Option<u64>,
    },
    Processing,
    TransactionTooOld(u64),
    InvalidTransaction(String),
    Other {
        error_code: u64,
        error_message: String,
    },
}

#[async_trait]
pub trait CyclesMintingCanister: Send + Sync {
    async fn get_icp_xdr(&self) -> CallResult<GetIcpXdrResult>;

    async fn notify_top_up(
        &self,
        block_index: u64,
        canister_id: Principal,
    ) -> CallResult<NotifyTopUpResult>;

    fn get_top_up_address(&self, target_canister_id: Principal) -> AccountIdentifier;
}

pub struct IcCyclesMintingCanister {
    cmc_canister_id: Principal,
}

impl IcCyclesMintingCanister {
    pub fn new(cmc_canister_id: Principal) -> Self {
        Self { cmc_canister_id }
    }
}

#[async_trait]
impl CyclesMintingCanister for IcCyclesMintingCanister {
    async fn get_icp_xdr(&self) -> CallResult<GetIcpXdrResult> {
        ic_cdk::call::<(), (GetIcpXdrResult,)>(
            self.cmc_canister_id,
            "get_icp_xdr_conversion_rate",
            (),
        )
        .await
        .map(|result| result.0)
    }

    async fn notify_top_up(
        &self,
        block_index: u64,
        canister_id: Principal,
    ) -> CallResult<NotifyTopUpResult> {
        let result: (NotifyTopUpResult,) = ic_cdk::call(
            self.cmc_canister_id,
            "notify_top_up",
            (NotifyTopUpArg {
                block_index,
                canister_id,
            },),
        )
        .await?;

        Ok(result.0)
    }

    fn get_top_up_address(&self, target_canister_id: Principal) -> AccountIdentifier {
        AccountIdentifier::new(&self.cmc_canister_id, &Subaccount::from(target_canister_id))
    }
}
