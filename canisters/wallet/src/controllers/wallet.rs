use crate::{
    core::{
        canister_config_mut, CallContext, CanisterConfig, WithCallContext, PERMISSION_ADMIN,
        PERMISSION_READ_FEATURES,
    },
    jobs::register_jobs,
    services::WalletService,
    transport::{WalletCanisterInit, WalletFeaturesResponse, WalletSettingsResponse},
};
use ic_canister_core::api::ApiResult;
use ic_cdk_macros::{init, post_upgrade, query};

#[init]
async fn initialize(input: Option<WalletCanisterInit>) {
    let input = input.unwrap_or_default();
    let config = CanisterConfig::default();

    WalletService::with_call_context(CallContext::get())
        .register_canister_config(config, input)
        .await;

    register_jobs().await;
}

#[post_upgrade]
async fn post_upgrade(input: Option<WalletCanisterInit>) {
    let input = input.unwrap_or_default();
    let config = canister_config_mut();

    WalletService::with_call_context(CallContext::get())
        .register_canister_config(config, input)
        .await;

    register_jobs().await;
}

#[query(name = "features")]
async fn get_wallet_features() -> ApiResult<WalletFeaturesResponse> {
    CallContext::get().check_access(PERMISSION_READ_FEATURES);

    let features = WalletService::with_call_context(CallContext::get()).get_features()?;

    Ok(WalletFeaturesResponse {
        features: features.into(),
    })
}

#[query(name = "wallet_settings")]
async fn wallet_settings() -> ApiResult<WalletSettingsResponse> {
    CallContext::get().check_access(PERMISSION_ADMIN);

    let settings = WalletService::with_call_context(CallContext::get()).get_wallet_settings()?;

    Ok(WalletSettingsResponse {
        settings: settings.into(),
    })
}
