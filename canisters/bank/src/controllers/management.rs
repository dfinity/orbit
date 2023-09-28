use crate::{
    core::{CallContext, WithCallContext},
    services::ManagementService,
    transport::{BankCanisterInit, BankDetailsResponse},
};
use ic_canister_core::api::ApiResult;
use ic_cdk_macros::{init, post_upgrade, query};

#[query(name = "get_bank_details")]
async fn get_bank_details() -> ApiResult<BankDetailsResponse> {
    let bank_details = ManagementService::new()
        .with_call_context(CallContext::get())
        .get_bank_details()
        .await?;

    Ok(BankDetailsResponse {
        details: bank_details,
    })
}

#[init]
async fn initialize(input: Option<BankCanisterInit>) {
    ManagementService::new().canister_init(input).await;
}

#[post_upgrade]
async fn post_upgrade() {
    ManagementService::new().canister_post_upgrade().await;
}
