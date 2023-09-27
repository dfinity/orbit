use crate::{
    core::{
        canister_config_mut, write_canister_config, CallContext, CanisterConfig, WithCallContext,
    },
    services::ManagementService,
    transport::{BankCanisterInit, BankDetailsResponse},
};
use ic_canister_core::{api::ApiResult, cdk::api::time};
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
    let init = input.unwrap_or_default();
    let config = CanisterConfig {
        // By default, the bank canister requires 100% of the votes to approve operations.
        approval_threshold: init.approval_threshold.unwrap_or(100u8),
        // The last time the canister was upgraded or initialized.
        last_upgrade_timestamp: time(),
    };

    write_canister_config(config);
}

#[post_upgrade]
async fn post_upgrade() {
    let mut updated_config = canister_config_mut();
    updated_config.last_upgrade_timestamp = time();

    write_canister_config(updated_config);
}
