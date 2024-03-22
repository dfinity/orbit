use crate::{
    core::{
        ic_cdk::api::{canister_balance, trap},
        middlewares::{authorize, call_context},
    },
    models::access_policy::{Resource, SettingsResourceAction},
    services::{SystemService, SYSTEM_SERVICE},
};
use ic_canister_core::api::ApiResult;
use ic_canister_macros::with_middleware;
use ic_cdk_macros::{post_upgrade, query};
use lazy_static::lazy_static;
use std::sync::Arc;
use wallet_api::{HealthStatus, SystemInfoResponse, SystemInstall, SystemUpgrade};

// Canister entrypoints for the controller.
#[cfg(any(not(feature = "canbench"), test))]
#[ic_cdk_macros::init]
async fn initialize(input: Option<SystemInstall>) {
    match input {
        Some(SystemInstall::Init(input)) => CONTROLLER.initialize(input).await,
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
async fn post_upgrade(input: Option<SystemInstall>) {
    match input {
        None => CONTROLLER.post_upgrade(None).await,
        Some(SystemInstall::Upgrade(input)) => CONTROLLER.post_upgrade(Some(input)).await,
        _ => trap("Wrong upgrade args for canister upgrade"),
    }
}

#[query(name = "health_status")]
async fn health_status() -> HealthStatus {
    CONTROLLER.health_status().await
}

#[query(name = "system_info")]
async fn system_info() -> ApiResult<SystemInfoResponse> {
    CONTROLLER.system_info().await
}

// Controller initialization and implementation.
lazy_static! {
    static ref CONTROLLER: SystemController = SystemController::new(Arc::clone(&SYSTEM_SERVICE));
}

#[derive(Debug)]
pub struct SystemController {
    system_service: Arc<SystemService>,
}

impl SystemController {
    fn new(system_service: Arc<SystemService>) -> Self {
        Self { system_service }
    }

    #[cfg(any(not(feature = "canbench"), test))]
    async fn initialize(&self, input: wallet_api::SystemInit) {
        let ctx = &call_context();
        self.system_service
            .init_canister(input, ctx)
            .await
            .unwrap_or_else(|err| {
                trap(&format!("Error: initializing canister failed {err}"));
            });
    }

    async fn post_upgrade(&self, input: Option<SystemUpgrade>) {
        self.system_service
            .upgrade_canister(input)
            .await
            .unwrap_or_else(|err| {
                trap(&format!("Error: upgrading canister failed {err}"));
            });
    }

    async fn health_status(&self) -> HealthStatus {
        self.system_service.health_status()
    }

    #[with_middleware(guard = authorize(&call_context(), &[Resource::Settings(SettingsResourceAction::SystemInfo)]))]
    async fn system_info(&self) -> ApiResult<SystemInfoResponse> {
        let system_info = self.system_service.get_system_info();
        let cycles = canister_balance();

        Ok(SystemInfoResponse {
            system: system_info.to_dto(&cycles, env!("CARGO_PKG_VERSION")),
        })
    }
}
