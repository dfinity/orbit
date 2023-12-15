use crate::{
    core::{
        ic_cdk::api::trap,
        is_canister_initialized,
        middlewares::{authorize, call_context},
    },
    models::access_control::{CanisterSettingsActionSpecifier, ResourceSpecifier},
    services::{WalletService, WALLET_SERVICE},
};
use ic_canister_core::api::ApiResult;
use ic_canister_macros::with_middleware;
use ic_cdk_macros::{init, post_upgrade, query};
use lazy_static::lazy_static;
use std::sync::Arc;
use wallet_api::{
    HealthStatus, WalletFeaturesResponse, WalletInit, WalletInstall, WalletSettingsResponse,
    WalletUpgrade,
};

// Canister entrypoints for the controller.
#[init]
async fn initialize(input: Option<WalletInstall>) {
    match input {
        Some(WalletInstall::Init(input)) => CONTROLLER.initialize(input).await,
        _ => trap("Missing init args to install canister"),
    }
}

#[post_upgrade]
async fn post_upgrade(input: Option<WalletInstall>) {
    match input {
        None => CONTROLLER.post_upgrade(None).await,
        Some(WalletInstall::Upgrade(input)) => CONTROLLER.post_upgrade(Some(input)).await,
        _ => trap("Wrong upgrade args for canister upgrade"),
    }
}

#[query(name = "health_status")]
async fn health_status() -> HealthStatus {
    match is_canister_initialized() {
        true => HealthStatus::Healthy,
        false => HealthStatus::Uninitialized,
    }
}

#[query(name = "features")]
async fn get_wallet_features() -> ApiResult<WalletFeaturesResponse> {
    CONTROLLER.get_wallet_features().await
}

#[query(name = "wallet_settings")]
async fn wallet_settings() -> ApiResult<WalletSettingsResponse> {
    CONTROLLER.wallet_settings().await
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: WalletController = WalletController::new(Arc::clone(&WALLET_SERVICE));
}

#[derive(Debug)]
pub struct WalletController {
    wallet_service: Arc<WalletService>,
}

impl WalletController {
    fn new(wallet_service: Arc<WalletService>) -> Self {
        Self { wallet_service }
    }

    async fn initialize(&self, input: WalletInit) {
        let ctx = &call_context();
        self.wallet_service
            .init_canister(input, ctx)
            .await
            .unwrap_or_else(|err| {
                trap(&format!("Error: initializing canister failed {err}"));
            });
    }

    async fn post_upgrade(&self, input: Option<WalletUpgrade>) {
        let ctx = &call_context();
        self.wallet_service
            .upgrade_canister(input, ctx)
            .await
            .unwrap_or_else(|err| {
                trap(&format!("Error: upgrading canister failed {err}"));
            });
    }

    #[with_middleware(
        guard = "authorize",
        context = "call_context",
        args = [ResourceSpecifier::CanisterSettings(CanisterSettingsActionSpecifier::ReadFeatures)],
        is_async = true
    )]
    async fn get_wallet_features(&self) -> ApiResult<WalletFeaturesResponse> {
        let features = self.wallet_service.get_features()?;

        Ok(WalletFeaturesResponse {
            features: features.into(),
        })
    }

    #[with_middleware(
        guard = "authorize",
        context = "call_context",
        args = [ResourceSpecifier::CanisterSettings(CanisterSettingsActionSpecifier::Read)],
        is_async = true
    )]
    async fn wallet_settings(&self) -> ApiResult<WalletSettingsResponse> {
        let settings = self.wallet_service.get_wallet_settings()?;

        Ok(WalletSettingsResponse {
            settings: settings.into(),
        })
    }
}
