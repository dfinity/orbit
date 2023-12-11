use crate::services::UpgradeService;
use crate::{
    core::{
        canister_config_mut,
        ic_cdk::api::print,
        middlewares::{authorize, call_context},
        CanisterConfig,
    },
    models::access_control::{CanisterSettingsActionSpecifier, ResourceSpecifier},
    services::{InstallMode, WalletService},
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
    static ref CONTROLLER: WalletController =
        WalletController::new(WalletService::default(), UpgradeService::default());
}

#[derive(Debug)]
pub struct WalletController {
    wallet_service: WalletService,
    upgrade_service: UpgradeService,
}

impl WalletController {
    fn new(wallet_service: WalletService, upgrade_service: UpgradeService) -> Self {
        Self {
            wallet_service,
            upgrade_service,
        }
    }

    async fn initialize(&self, input: Option<WalletCanisterInit>) {
        let input = input.unwrap_or_default();
        let mut config = CanisterConfig::default();

        self.wallet_service
            .process_canister_install(&mut config, input, &call_context(), InstallMode::Init)
            .await;
    }

    async fn post_upgrade(&self, input: Option<WalletCanisterInit>) {
        let input = input.unwrap_or_default();
        let mut config = canister_config_mut();

        self.wallet_service
            .process_canister_install(&mut config, input, &call_context(), InstallMode::Upgrade)
            .await;

        if let Err(err) = self.upgrade_service.verify_upgrade().await {
            print(format!("Error: verifying upgrade failed {err}"));
        }
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
