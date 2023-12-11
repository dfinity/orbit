use crate::{
    core::{
        canister_config_mut,
        middlewares::{authorize, call_context},
        CanisterConfig,
    },
    jobs::register_jobs,
    models::access_control::{CanisterSettingsActionSpecifier, ResourceSpecifier},
    services::WalletService,
};
use ic_canister_core::api::ApiResult;
use ic_canister_macros::with_middleware;
use ic_cdk_macros::{init, post_upgrade, query};
use lazy_static::lazy_static;
use wallet_api::{WalletCanisterInit, WalletFeaturesResponse, WalletSettingsResponse};

// Canister entrypoints for the controller.
#[init]
async fn initialize(input: Option<WalletCanisterInit>) {
    CONTROLLER.initialize(input).await
}

#[post_upgrade]
async fn post_upgrade(input: Option<WalletCanisterInit>) {
    CONTROLLER.post_upgrade(input).await
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
    static ref CONTROLLER: WalletController = WalletController::new(WalletService::default());
}

#[derive(Debug)]
pub struct WalletController {
    wallet_service: WalletService,
}

impl WalletController {
    fn new(wallet_service: WalletService) -> Self {
        Self { wallet_service }
    }

    async fn initialize(&self, input: Option<WalletCanisterInit>) {
        let input = input.unwrap_or_default();
        let config = CanisterConfig::default();

        self.wallet_service
            .register_canister_config(config, input, &call_context())
            .await;

        register_jobs().await;
    }

    async fn post_upgrade(&self, input: Option<WalletCanisterInit>) {
        let input = input.unwrap_or_default();
        let config = canister_config_mut();

        self.wallet_service
            .register_canister_config(config, input, &call_context())
            .await;

        register_jobs().await;
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
