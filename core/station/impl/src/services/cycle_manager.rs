use canfund::manager::options::{FundManagerOptions, FundStrategy};
use canfund::manager::RegisterOpts;
use canfund::FundManager;
use ic_cdk::api::management_canister::main::CanisterId;
use ic_cdk::id;
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::sync::Arc;

thread_local! {
    static FUND_MANAGER: RefCell<FundManager> = RefCell::new({
        let mut manager = FundManager::new();
        // Strategy can be default as we always override per canister
        manager.with_options(FundManagerOptions::new().with_interval_secs(60*60*6));
        // The station canister is already being monitored, we need to override the default registration
        // This instance of canfund also won't mint cycles and rely on station to do so
        manager.unregister(id());

        manager
    });
}

lazy_static! {
    pub static ref CYCLE_MANAGER: Arc<CycleManager> = Arc::new(CycleManager::new());
}

#[derive(Debug, Default)]
pub struct CycleManager {}

impl CycleManager {
    fn new() -> Self {
        Self {}
    }

    pub fn add_canister(&self, canister_id: CanisterId, fund_strategy: FundStrategy) {
        FUND_MANAGER.with(|manager| {
            manager.borrow_mut().register(
                canister_id,
                RegisterOpts::new().with_strategy(fund_strategy),
            );
        });

        #[cfg(target_arch = "wasm32")]
        self.toggle_canister_monitoring();
    }

    pub fn remove_canister(&self, canister_id: CanisterId) {
        FUND_MANAGER.with(|manager| {
            manager.borrow_mut().unregister(canister_id);
        });

        #[cfg(target_arch = "wasm32")]
        self.toggle_canister_monitoring();
    }

    #[cfg(target_arch = "wasm32")]
    fn toggle_canister_monitoring(&self) {
        FUND_MANAGER.with(|manager| {
            let mut manager = manager.borrow_mut();
            if manager.is_running() && manager.get_canisters().is_empty() {
                manager.stop();
            } else if !manager.is_running() && !manager.get_canisters().is_empty() {
                manager.start();
            }
        });
    }
}
