use crate::{
    core::{CallContext, WithCallContext},
    jobs::register_jobs,
    services::ManagementService,
    transport::{BankCanisterInit, BankFeaturesResponse, BankSettingsResponse},
};
use ic_canister_core::api::ApiResult;
use ic_cdk_macros::{init, post_upgrade, query};

#[query(name = "features")]
async fn get_bank_features() -> ApiResult<BankFeaturesResponse> {
    let features = ManagementService::new()
        .with_call_context(CallContext::get())
        .get_bank_features()
        .await?;

    Ok(BankFeaturesResponse { features })
}

#[query(name = "bank_settings")]
async fn bank_settings() -> ApiResult<BankSettingsResponse> {
    let settings = ManagementService::new()
        .with_call_context(CallContext::get())
        .get_bank_settings()
        .await?;

    Ok(BankSettingsResponse { settings })
}

#[init]
async fn initialize(input: Option<BankCanisterInit>) {
    ManagementService::new().canister_init(input).await;
    register_jobs().await;
}

#[post_upgrade]
async fn post_upgrade(input: Option<BankCanisterInit>) {
    ManagementService::new().canister_post_upgrade(input).await;
    register_jobs().await;
}
