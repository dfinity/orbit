use crate::{
    core::{
        ic_cdk::api::{canister_balance, trap},
        middlewares::{authorize, call_context},
    },
    migration,
    models::resource::{Resource, SystemResourceAction},
    services::{SystemService, SYSTEM_SERVICE},
    SYSTEM_VERSION,
};
use ic_cdk_macros::{post_upgrade, query};
use lazy_static::lazy_static;
use orbit_essentials::api::ApiResult;
use orbit_essentials::with_middleware;
use station_api::{HealthStatus, SystemInfoResponse, SystemInstall, SystemUpgrade};
use std::sync::Arc;

// Canister entrypoints for the controller.
#[cfg(any(not(feature = "canbench"), test))]
#[ic_cdk_macros::init]
async fn initialize(input: Option<SystemInstall>) {
    match input {
        Some(SystemInstall::Init(input)) => CONTROLLER.initialize(input).await,
        Some(SystemInstall::Upgrade(_)) | None => trap("Invalid args to initialize canister"),
    }
}

/// The init is overriden for benchmarking purposes.
///
/// This is only used for benchmarking and is not included in the final canister.
#[cfg(all(feature = "canbench", not(test)))]
#[ic_cdk_macros::init]
pub async fn mock_init() {
    use crate::core::write_system_info;
    use crate::models::SystemInfo;
    use crate::repositories::permission::PERMISSION_REPOSITORY;
    use candid::Principal;

    // Initialize the random number generator with a fixed seed to ensure deterministic
    // results across runs of the benchmarks.
    orbit_essentials::utils::initialize_rng_from_seed([0u8; 32]);

    // Initialize the system info.
    let mut system = SystemInfo::default();
    system.set_upgrader_canister_id(Principal::from_slice(&[25; 29]));

    // Initialize the permission cached entries for repositories.
    PERMISSION_REPOSITORY.build_cache();

    write_system_info(system);
}

#[post_upgrade]
async fn post_upgrade(input: Option<SystemInstall>) {
    match input {
        None => CONTROLLER.post_upgrade(None).await,
        Some(SystemInstall::Upgrade(input)) => CONTROLLER.post_upgrade(Some(input)).await,
        Some(SystemInstall::Init(_)) => trap("Invalid args to upgrade canister"),
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
    async fn initialize(&self, input: station_api::SystemInit) {
        self.system_service
            .init_canister(input)
            .await
            .unwrap_or_else(|err| {
                trap(&format!("Error: initializing canister failed {err}"));
            });
    }

    async fn post_upgrade(&self, input: Option<SystemUpgrade>) {
        // Runs the migrations for the canister to ensure the stable memory schema is up-to-date
        //
        // WARNING: This needs to be done before any other access to stable memory is done, this is because
        // it might clear memory ids and the current codebase might be reusing them and loading a diffirent
        // datatype from the one that was initially stored.
        migration::MigrationHandler::run();

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

    #[with_middleware(guard = authorize(&call_context(), &[Resource::System(SystemResourceAction::SystemInfo)]))]
    async fn system_info(&self) -> ApiResult<SystemInfoResponse> {
        let system_info = self.system_service.get_system_info();
        let cycles = canister_balance();

        Ok(SystemInfoResponse {
            system: system_info.to_dto(&cycles, SYSTEM_VERSION),
        })
    }
}
