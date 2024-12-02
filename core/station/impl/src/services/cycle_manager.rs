use crate::factories::blockchains::InternetComputer;
use crate::models::{AccountKey, CycleObtainStrategy, MonitorExternalCanisterStrategy};
use crate::repositories::ACCOUNT_REPOSITORY;
use canfund::api::cmc::IcCyclesMintingCanister;
use canfund::api::ledger::IcLedgerCanister;
use canfund::manager::options::{FundManagerOptions, ObtainCyclesOptions};
use canfund::manager::RegisterOpts;
use canfund::operations::obtain::MintCycles;
use canfund::FundManager;
use ic_cdk::api::management_canister::main::CanisterId;
use ic_cdk::print;
use ic_ledger_types::{Subaccount, MAINNET_CYCLES_MINTING_CANISTER_ID, MAINNET_LEDGER_CANISTER_ID};
use lazy_static::lazy_static;
use orbit_essentials::repository::Repository;
use std::cell::RefCell;
use std::sync::Arc;
use uuid::Uuid;

thread_local! {
    static FUND_MANAGER: RefCell<FundManager> = RefCell::new(FundManager::new());
}

lazy_static! {
    pub static ref CYCLE_MANAGER: Arc<CycleManager> = Arc::new(CycleManager::new());
}

#[derive(Debug, Default)]
pub struct CycleManager {}

impl CycleManager {
    fn new() -> Self {
        FUND_MANAGER.with(|manager| {
            let mut manager = manager.borrow_mut();
            // Strategy can be default as we always override per canister
            // Obtain cycles config is inherited from the system service
            // Interval is 6 hours for all canisters monitored by the station (i.e. upgrader + external canisters)
            let options = FundManagerOptions::new().with_interval_secs(60 * 60 * 6);
            manager.with_options(options);
        });

        Self {}
    }

    pub fn add_canister(
        &self,
        canister_id: CanisterId,
        fund_strategy: MonitorExternalCanisterStrategy,
        cycle_obtain_strategy: Option<CycleObtainStrategy>,
    ) {
        let mut register_opts = RegisterOpts::new().with_strategy(fund_strategy.into());

        if let Some(strategy) =
            cycle_obtain_strategy.and_then(|strategy| get_obtain_cycle_config(&strategy))
        {
            register_opts = register_opts.with_obtain_cycles_options(strategy);
        }

        FUND_MANAGER.with(|manager| {
            manager.borrow_mut().register(canister_id, register_opts);
        });

        ic_cdk::print(format!(
            "Cycle manager: canister {} added to cycle monitoring.",
            canister_id
        ));
    }

    pub fn remove_canister(&self, canister_id: CanisterId) {
        FUND_MANAGER.with(|manager| {
            manager.borrow_mut().unregister(canister_id);
        });

        ic_cdk::print(format!(
            "Cycle manager: canister {} removed from cycle monitoring.",
            canister_id
        ));
    }

    pub fn start(&self) {
        FUND_MANAGER.with(|manager| {
            manager.borrow_mut().start();
        });

        ic_cdk::print("Cycle manager: monitoring started.");
    }

    pub fn set_global_cycle_obtain_strategy(&self, strategy: &CycleObtainStrategy) {
        FUND_MANAGER.with(|manager| {
            let mut fund_manager = manager.borrow_mut();
            let options = fund_manager.get_options();
            let options = options.with_obtain_cycles_options(get_obtain_cycle_config(strategy));
            fund_manager.with_options(options);
        });

        ic_cdk::print(format!(
            "Cycle manager: obtain cycles strategy changed to {:?}.",
            strategy
        ));
    }
}

fn get_obtain_cycle_config(strategy: &CycleObtainStrategy) -> Option<ObtainCyclesOptions> {
    match strategy {
        CycleObtainStrategy::Disabled => None,
        CycleObtainStrategy::MintFromNativeToken { account_id } => {
            if let Some(account) = ACCOUNT_REPOSITORY.get(&AccountKey { id: *account_id }) {
                Some(ObtainCyclesOptions {
                    obtain_cycles: Arc::new(MintCycles {
                        ledger: Arc::new(IcLedgerCanister::new(MAINNET_LEDGER_CANISTER_ID)),
                        cmc: Arc::new(IcCyclesMintingCanister::new(
                            MAINNET_CYCLES_MINTING_CANISTER_ID,
                        )),
                        from_subaccount: Subaccount(InternetComputer::subaccount_from_seed(
                            &account.seed,
                        )),
                    }),
                })
            } else {
                print(format!(
                    "Account with id `{}` not found, cannot create ObtainCyclesOptions",
                    Uuid::from_bytes(*account_id).hyphenated()
                ));

                None
            }
        }
    }
}
