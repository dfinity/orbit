use crate::core::ic_cdk::api::time;
use crate::core::CallContext;
use crate::core::{write_canister_config, CanisterConfig};
use crate::errors::CanisterError;
use crate::repositories::{UserRepository, USER_REPOSITORY};
use canfund::fetch::cycles::FetchCyclesBalanceFromPrometheusMetrics;
use canfund::manager::options::{EstimatedRuntime, FundManagerOptions, FundStrategy};
use canfund::FundManager;
use control_panel_api::CanisterModules;
use lazy_static::lazy_static;
use orbit_essentials::api::ServiceResult;
use orbit_essentials::repository::Repository;
use std::cell::RefCell;
use std::collections::HashSet;
use std::sync::Arc;

lazy_static! {
    pub static ref CANISTER_SERVICE: Arc<CanisterService> =
        Arc::new(CanisterService::new(Arc::clone(&USER_REPOSITORY)));
}

thread_local! {
    /// Monitor the cycles of canisters and top up if necessary.
    pub static FUND_MANAGER: RefCell<FundManager> = RefCell::new(FundManager::new());
}

#[derive(Default, Debug)]
pub struct CanisterService {
    user_repository: Arc<UserRepository>,
}

impl CanisterService {
    pub fn new(user_repository: Arc<UserRepository>) -> Self {
        Self { user_repository }
    }

    /// Checks if the caller is a controller.
    fn assert_controller(&self, ctx: &CallContext, method: String) -> ServiceResult<()> {
        if !ctx.is_controller() {
            Err(CanisterError::Forbidden { method })?
        }

        Ok(())
    }

    pub async fn upload_canister_modules(&self, modules: CanisterModules) -> ServiceResult<()> {
        self.assert_controller(&CallContext::get(), "upload_canister_modules".to_string())?;

        let mut config =
            CanisterConfig::new(modules.upgrader_wasm_module, modules.station_wasm_module);

        config.last_upgrade_timestamp = time();

        write_canister_config(config);

        Ok(())
    }

    pub async fn init_canister(&self) -> ServiceResult<()> {
        self.start_canister_cycles_monitoring();

        Ok(())
    }

    // Monitor the cycles of active canisters that have been deployed by the control panel
    // and top up if necessary.
    fn start_canister_cycles_monitoring(&self) {
        let users = self.user_repository.list();
        let deployed_stations = users
            .iter()
            .flat_map(|user| {
                user.deployed_stations.iter().filter(|canister_id| {
                    user.stations
                        .iter()
                        .any(|station| station.canister_id == **canister_id)
                })
            })
            .collect::<HashSet<_>>();

        FUND_MANAGER.with(|fund_manager| {
            let mut fund_manager = fund_manager.borrow_mut();

            fund_manager.with_options(
                FundManagerOptions::new()
                    .with_interval_secs(12 * 60 * 60) // twice a day
                    .with_strategy(FundStrategy::BelowEstimatedRuntime(
                        EstimatedRuntime::new()
                            .with_min_runtime_secs(2 * 24 * 60 * 60) // 2 days
                            .with_fund_runtime_secs(5 * 24 * 60 * 60) // 3 days
                            .with_max_runtime_cycles_fund(1_000_000_000_000)
                            .with_fallback_min_cycles(125_000_000_000)
                            .with_fallback_fund_cycles(250_000_000_000),
                    )),
            );
            fund_manager.with_cycles_fetcher(Arc::new(
                FetchCyclesBalanceFromPrometheusMetrics::new(
                    "/metrics".to_string(),
                    "station_canister_cycles_balance".to_string(),
                ),
            ));

            for canister_id in deployed_stations {
                fund_manager.register(*canister_id);
            }

            fund_manager.start();
        });
    }
}
