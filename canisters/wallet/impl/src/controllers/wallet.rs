use crate::{
    core::{
        ic_cdk::api::trap,
        is_canister_initialized,
        middlewares::{authorize, call_context},
    },
    models::access_policy::{Resource, SettingsResourceAction},
    services::{WalletService, WALLET_SERVICE},
};
use ic_canister_core::api::ApiResult;
use ic_canister_macros::with_middleware;
use ic_cdk_macros::{post_upgrade, query};
use lazy_static::lazy_static;
use std::sync::Arc;
use wallet_api::{
    GetConfigResponse, HealthStatus, WalletInstall, WalletSettingsResponse, WalletUpgrade,
};

// Canister entrypoints for the controller.
#[cfg(any(not(feature = "canbench"), test))]
#[ic_cdk_macros::init]
async fn initialize(input: Option<WalletInstall>) {
    match input {
        Some(WalletInstall::Init(input)) => CONTROLLER.initialize(input).await,
        _ => trap("Invalid init args to install canister"),
    }
}

/// The init is overriden for benchmarking purposes.
///
/// This is only used for benchmarking and is not included in the final canister.
#[cfg(all(feature = "canbench", not(test)))]
#[ic_cdk_macros::init]
pub async fn mock_init() {
    // Initialize the random number generator with a fixed seed to ensure deterministic
    // results across runs of the benchmarks.
    ic_canister_core::utils::initialize_rng_from_seed([0u8; 32]);
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

    #[cfg(any(not(feature = "canbench"), test))]
    async fn initialize(&self, input: wallet_api::WalletInit) {
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

    #[with_middleware(guard = authorize(&call_context(), &[Resource::Settings(SettingsResourceAction::ReadConfig)]))]
    async fn get_config(&self) -> ApiResult<GetConfigResponse> {
        let config = self.wallet_service.get_config()?;

        Ok(GetConfigResponse {
            config: config.into(),
        })
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::Settings(SettingsResourceAction::Read)]))]
    async fn wallet_settings(&self) -> ApiResult<WalletSettingsResponse> {
        let settings = self.wallet_service.get_wallet_settings()?;

        Ok(WalletSettingsResponse {
            settings: settings.into(),
        })
    }
}
