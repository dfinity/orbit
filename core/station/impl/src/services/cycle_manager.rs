use crate::factories::blockchains::InternetComputer;
use crate::models::CycleObtainStrategy;
use crate::services::{SystemService, SYSTEM_SERVICE};
use canfund::api::cmc::IcCyclesMintingCanister;
use canfund::api::ledger::IcLedgerCanister;
use canfund::manager::options::{FundManagerOptions, FundStrategy, ObtainCyclesOptions};
use canfund::manager::RegisterOpts;
use canfund::operations::obtain::MintCycles;
use canfund::FundManager;
use ic_cdk::api::management_canister::main::CanisterId;
use ic_ledger_types::{Subaccount, MAINNET_CYCLES_MINTING_CANISTER_ID, MAINNET_LEDGER_CANISTER_ID};
use lazy_static::lazy_static;
use std::cell::RefCell;
use std::sync::Arc;

thread_local! {
    static FUND_MANAGER: RefCell<FundManager> = RefCell::new(FundManager::new());
}

lazy_static! {
    pub static ref CYCLE_MANAGER: Arc<CycleManager> =
        Arc::new(CycleManager::new(Arc::clone(&SYSTEM_SERVICE),));
}

#[derive(Debug, Default)]
pub struct CycleManager {}

impl CycleManager {
    fn new(system_service: Arc<SystemService>) -> Self {
        let system_info = system_service.get_system_info();

        FUND_MANAGER.with(|manager| {
            let mut manager = manager.borrow_mut();
            // Strategy can be default as we always override per canister
            // Obtain cycles config is inherited from the system service
            let options = FundManagerOptions::new()
                .with_interval_secs(60 * 60 * 6)
                .with_obtain_cycles_options(get_obtain_cycle_config(
                    system_info.get_cycle_obtain_strategy(),
                ));
            manager.with_options(options);
        });

        Self {}
    }

    pub fn add_canister(&self, canister_id: CanisterId, fund_strategy: FundStrategy) {
        FUND_MANAGER.with(|manager| {
            manager.borrow_mut().register(
                canister_id,
                RegisterOpts::new().with_strategy(fund_strategy),
            );
        });
    }

    pub fn remove_canister(&self, canister_id: CanisterId) {
        FUND_MANAGER.with(|manager| {
            manager.borrow_mut().unregister(canister_id);
        });
    }

    pub fn start(&self) {
        FUND_MANAGER.with(|manager| {
            manager.borrow_mut().start();
        });
    }

    pub fn set_global_minting_strategy(&self, strategy: &CycleObtainStrategy) {
        FUND_MANAGER.with(|manager| {
            let mut fund_manager = manager.borrow_mut();
            let options = fund_manager.get_options();
            let options = options.with_obtain_cycles_options(get_obtain_cycle_config(strategy));
            fund_manager.with_options(options);
        });
    }
}

fn get_obtain_cycle_config(strategy: &CycleObtainStrategy) -> Option<ObtainCyclesOptions> {
    match strategy {
        CycleObtainStrategy::Disabled => None,
        CycleObtainStrategy::MintFromNativeToken { account_id } => Some(ObtainCyclesOptions {
            obtain_cycles: Arc::new(MintCycles {
                ledger: Arc::new(IcLedgerCanister::new(MAINNET_LEDGER_CANISTER_ID)),
                cmc: Arc::new(IcCyclesMintingCanister::new(
                    MAINNET_CYCLES_MINTING_CANISTER_ID,
                )),
                from_subaccount: Subaccount(InternetComputer::subaccount_from_station_account_id(
                    account_id,
                )),
            }),
        }),
    }
}
