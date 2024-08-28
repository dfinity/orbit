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
use orbit_essentials::http::set_certified_data_for_skip_certification;
use orbit_essentials::with_middleware;
use station_api::{HealthStatus, SystemInfoResponse, SystemInstall, SystemUpgrade};
use std::sync::Arc;

// Canister entrypoints for the controller.
#[cfg(any(not(feature = "canbench"), test))]
#[ic_cdk_macros::init]
async fn initialize(input: Option<SystemInstall>) {
    set_certified_data_for_skip_certification();
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
    use candid::Principal;

    // Initialize the random number generator with a fixed seed to ensure deterministic
    // results across runs of the benchmarks.
    orbit_essentials::utils::initialize_rng_from_seed([0u8; 32]);

    // Initialize the system info.
    let mut system = SystemInfo::default();
    system.set_upgrader_canister_id(Principal::from_slice(&[25; 29]));

    write_system_info(system);
}

#[post_upgrade]
async fn post_upgrade(input: Option<SystemInstall>) {
    // Runs the migrations for the canister to ensure the stable memory schema is up-to-date
    //
    // WARNING: This needs to be done before any other access to stable memory is done, this is because
    // it might clear memory ids and the current codebase might be reusing them and loading a diffirent
    // datatype from the one that was initially stored.
    migration::MigrationHandler::run();

    set_certified_data_for_skip_certification();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ic_cdk::next_time;
    use crate::core::{read_system_info, write_system_info};
    use crate::models::request_test_utils::mock_request;
    use crate::models::{RequestStatus, SystemInfo};
    use crate::repositories::REQUEST_REPOSITORY;
    use crate::STABLE_MEMORY_VERSION;
    use candid::Principal;
    use orbit_essentials::repository::Repository;

    #[tokio::test]
    async fn apply_migration_should_migrate_stable_memory_version() {
        let mut system_info = SystemInfo::new(Principal::management_canister(), Vec::new());

        system_info.set_stable_memory_version(0);

        write_system_info(system_info);

        post_upgrade(None).await;

        let mut system_info = read_system_info();

        assert_eq!(
            system_info.get_stable_memory_version(),
            STABLE_MEMORY_VERSION
        );

        // now reset to the original version, add a request and check if the migration is applied again
        let mut request = mock_request();
        request.status = RequestStatus::Processing {
            started_at: next_time(),
        };

        REQUEST_REPOSITORY.insert(request.to_key(), request.clone());

        system_info.set_stable_memory_version(0);
        system_info.set_change_canister_request(request.id);

        write_system_info(system_info);

        post_upgrade(None).await;

        let system_info = read_system_info();

        assert_eq!(
            system_info.get_stable_memory_version(),
            STABLE_MEMORY_VERSION
        );
    }
}
