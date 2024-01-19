use crate::{
    core::{
        access_control::AccessControlEvaluator,
        evaluation::Evaluate,
        ic_cdk::api::trap,
        is_canister_initialized,
        middlewares::{authorize, call_context},
    },
    errors::AccessControlError,
    models::access_control::{
        CanisterSettingsActionSpecifier, CommonActionSpecifier, ResourceSpecifier, ResourceType,
    },
    services::{WalletService, WALLET_SERVICE},
};
use ic_canister_core::api::ApiResult;
use ic_canister_macros::with_middleware;
use ic_cdk_macros::{init, post_upgrade, query};
use lazy_static::lazy_static;
use std::sync::Arc;
use wallet_api::{
    GetConfigResponse, HealthStatus, WalletInit, WalletInstall, WalletSettingsResponse,
    WalletUpgrade,
};

// Canister entrypoints for the controller.
#[init]
async fn initialize(input: Option<WalletInstall>) {
    match input {
        Some(WalletInstall::Init(input)) => CONTROLLER.initialize(input).await,
        _ => trap("Invalid init args to install canister"),
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

#[query(name = "config")]
async fn get_config() -> ApiResult<GetConfigResponse> {
    CONTROLLER.get_config().await
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
        args = [ResourceSpecifier::CanisterSettings(CanisterSettingsActionSpecifier::ReadConfig)],
        is_async = true
    )]
    async fn get_config(&self) -> ApiResult<GetConfigResponse> {
        let ctx = &call_context();
        let evaluator = AccessControlEvaluator::new(
            ctx,
            ResourceSpecifier::Common(ResourceType::UserGroup, CommonActionSpecifier::List),
        );
        let can_view_user_groups = evaluator
            .evaluate()
            .await
            .map_err(|e| AccessControlError::UnexpectedError(e.into()))?;

        let config = self.wallet_service.get_config(can_view_user_groups)?;

        Ok(GetConfigResponse {
            config: config.into(),
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
