#![cfg(test)]

use candid::Principal;
use pocket_ic::PocketIc;

mod account_tests;
mod address_book_tests;
mod asset_tests;
mod control_panel_tests;
mod cycles_monitor_tests;
mod dfx_orbit;
mod disaster_recovery_tests;
mod external_canister_tests;
mod http;
mod install_tests;
mod interfaces;
mod named_rule_tests;
mod notification;
mod rate_limiter;
mod register_tests;
mod request_validation_tests;
mod setup;
mod station_migration_tests;
mod station_test_data;
mod system_upgrade_tests;
mod transfer_tests;
mod upgrader_migration_tests;
mod upgrader_test_data;
mod user;
mod utils;

pub struct TestEnv {
    pub env: PocketIc,
    pub canister_ids: CanisterIds,
    pub controller: Principal,
    pub minter: Principal,
}

#[derive(Clone, Copy, Debug)]
pub struct CanisterIds {
    pub icp_ledger: Principal,
    pub icp_index: Principal,
    pub cycles_minting_canister: Principal,
    pub control_panel: Principal,
    pub station: Principal,
}
