use crate::{
    core::{
        canister_config_mut, CallContext, CanisterConfig, WithCallContext, PERMISSION_ADMIN,
        PERMISSION_READ_FEATURES,
    },
    jobs::register_jobs,
    services::BankService,
    transport::{BankCanisterInit, BankFeaturesResponse, BankSettingsResponse},
};
use ic_canister_core::api::ApiResult;
use ic_cdk_macros::{init, post_upgrade, query};

#[init]
async fn initialize(input: Option<BankCanisterInit>) {
    let input = input.unwrap_or_default();
    let config = CanisterConfig::default();

    BankService::with_call_context(CallContext::get())
        .register_canister_config(config, input)
        .await;

    register_jobs().await;
}

#[post_upgrade]
async fn post_upgrade(input: Option<BankCanisterInit>) {
    let input = input.unwrap_or_default();
    let config = canister_config_mut();

    BankService::with_call_context(CallContext::get())
        .register_canister_config(config, input)
        .await;

    register_jobs().await;
}

#[query(name = "features")]
async fn get_bank_features() -> ApiResult<BankFeaturesResponse> {
    CallContext::get().check_access(PERMISSION_READ_FEATURES);

    let features = BankService::with_call_context(CallContext::get()).get_features()?;

    Ok(BankFeaturesResponse {
        features: features.into(),
    })
}

#[query(name = "bank_settings")]
async fn bank_settings() -> ApiResult<BankSettingsResponse> {
    CallContext::get().check_access(PERMISSION_ADMIN);

    let settings = BankService::with_call_context(CallContext::get()).get_bank_settings()?;

    Ok(BankSettingsResponse {
        settings: settings.into(),
    })
}
